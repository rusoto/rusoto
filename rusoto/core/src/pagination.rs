//! Traits and helperfunctions for dealing with paged responses

use crate::RusotoResult;
use futures::{Future, Stream, StreamExt, TryStreamExt};
use std::borrow::Cow;

/// A stream of results returned over multiple pages by amazon
pub type RusotoStream<'a, I, E> = Box<dyn Stream<Item = RusotoResult<I, E>> + 'a>;

/// Trait representing a request or response that can have a token added to fetch subsequent pages
/// It will normally be used to move the `Token` from the reponse to the next request to be made
pub trait Paged {
    /// The type of the paging token, usuaully `Option<String>` but sometimes a tuple or something more exotic
    type Token: Clone;

    /// Takes the token out of the request/repsonse leaving `None` in it's place
    /// If the token is part of result set it is cloned instead.
    fn take_pagination_token(&mut self) -> Self::Token;

    /// Get the token for getting the next page
    // COW because while most iplementations will return a reference
    // Some need to assemble the token so reference would not live past end of method
    fn pagination_token(&self) -> Cow<Self::Token>;
}

/// Trait representing a request that can have a token added to fetch subsequent pages
/// It will normally be used to move the `Token` from the reponse to the next request to be made
pub trait PagedRequest: Paged {
    /// Set the token to get the next page
    fn set_pagination_token(&mut self, key: Self::Token);
}

/// Trait representing the partial output of a request that may have more pages of results
pub trait PagedOutput: Paged {
    /// The type of the item that is being paged through
    type Item;

    /// Throw away the pagining metadata and extract the list of results
    fn into_pagination_page(self) -> Vec<Self::Item>;

    /// Are there more pages of results after this one
    // normally just `pagination_token().is_some()` but some service have a field for this
    fn has_another_page(&self) -> bool;
}

enum PageState<T> {
    Token(T),
    Done,
}

/// A helper function to wrap a simple call that takes a pagination token and keep calling it
/// until the last page is reached, returns the results as a stream
pub fn aws_stream<T: 'static, PO, E, F, I>(
    token: T,
    action: impl FnMut(T) -> F,
) -> impl Stream<Item = RusotoResult<I, E>>
where
    PO: PagedOutput<Token = T, Item = I>,
    F: Future<Output = RusotoResult<PO, E>>,
{
    futures::stream::try_unfold(
        (PageState::Token(token), action),
        move |(state, mut action)| async move {
            let token = match state {
                PageState::Token(token) => token,
                PageState::Done => return RusotoResult::<_, E>::Ok(None),
            };

            let mut response = action(token).await?;

            let next_state = if response.has_another_page() {
                PageState::Token(response.take_pagination_token())
            } else {
                PageState::Done
            };

            Ok(Some((
                futures::stream::iter(response.into_pagination_page().into_iter()).map(Ok),
                (next_state, action),
            )))
        },
    )
    .try_flatten()
}
