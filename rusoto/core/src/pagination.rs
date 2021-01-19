//! Traits and helperfunctions for dealing with paged responses

use futures::{stream, Stream, TryStreamExt, Future};
use crate::{RusotoError};
use std::pin::Pin;

/// A stream of results returned over multiple pages by amazon
pub type RusotoStream<I, E> = Pin<Box<dyn Stream<Item = Result<I, RusotoError<E>>>>>;

/// Trait representing a request that can have a token added to fetch subsequent pages
pub trait PagedRequest {
    /// The type of the paging token, usuaully `Option<String>` but sometimes a tuple or something mor exotic
    type Token;

    /// Set the token to get the next page
    /// See [PagedOutput::pagination_token]
    fn with_pagination_token(self, key: Self::Token) -> Self;
} 


/// Trait representing the partial output of a request that may have more pages of results 
pub trait PagedOutput {
    /// The type of the item that is being paged through
    type Item;
    /// The type of the paging token, usuaully `Option<String>` but sometimes a tuple or something mor exotic
    type Token;
    
    /// Get the token for sending in the request to get the next page
    /// See [PagedRequest::with_pagination_token]
    fn pagination_token(&self) -> Self::Token;

    /// Throw away the pagining metadata and extract the list of results
    fn into_pagination_page(self) -> Vec<Self::Item>;

    /// Are there more pages of results after this one
    fn has_another_page(&self) -> bool;
}

enum PageState<C, R, F> {
    Next(C, R, F),
    End,
}    


/// A helper function to wrap a simple call that takes a pagination token and keep calling it
/// until the last page is reached, returns the results as a stream
pub fn all_pages<C, R, O, I, T, F, E>(client: C, init: R, f: F) -> RusotoStream<I, E> where
    C: Send + 'static,
    R: PagedRequest<Token = T> + 'static,
    I: Send + 'static,
    O: PagedOutput<Token = T, Item = I>,
    F: for<'a> FnMut(&'a C, &'a R) -> Pin<Box<dyn Future<Output = Result<O, RusotoError<E>>> + 'a>> + 'static,
{
    
    Box::pin(
        stream::try_unfold(
            PageState::Next(client, init, f),
            move |state| { async move {
                if let PageState::Next(client, input, mut f) = state {
                    let resp: O = f(&client, &input).await?;
                    let next_state = if resp.has_another_page() {
                        PageState::Next(client, input.with_pagination_token(resp.pagination_token()), f)
                    } else {
                        PageState::End
                    };
                    Ok(Some((
                        stream::iter(resp.into_pagination_page().into_iter().map(Ok)),
                        next_state,
                    )))
                } else {
                    Ok(None) as Result<_, RusotoError<E>>
                }
            }},
        )
        .try_flatten())
}