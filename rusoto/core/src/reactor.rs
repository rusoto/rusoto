use std::thread;
use std::rc::Rc;

use futures::{Async, Future, Poll, Stream};
use futures::sync::mpsc;
use futures::sync::oneshot;
use tokio_core::reactor::{Core, Handle};

use super::{SignedRequest, HttpResponse, HttpDispatchError, DispatchSignedRequest};

pub struct Reactor {
    request_sender: mpsc::UnboundedSender<ReactorRequest>
}

struct ReactorRequest {
    signed_request: SignedRequest,
    response_sender: oneshot::Sender<Result<HttpResponse, HttpDispatchError>>
}

pub struct ReactorFuture {
    response_receiver: oneshot::Receiver<Result<HttpResponse, HttpDispatchError>>
}

impl Future for ReactorFuture {
    type Item = HttpResponse;
    type Error = HttpDispatchError;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.response_receiver.poll().expect("failed to retrieve response from reactor") {
            Async::NotReady => Ok(Async::NotReady),
            Async::Ready(result) => result.map(Async::Ready)
        }
    }
}

impl Reactor {
    pub fn new<D: DispatchSignedRequest + 'static, E: Send + 'static, F: FnOnce(Handle) -> Result<D, E> + Send + 'static>(make_dispatcher: F) -> Result<Reactor, E> {
        let (init_tx, init_rx) = oneshot::channel();

        thread::spawn(move || {
            let mut core = Core::new().expect("failed to create event loop");

            let (tx, rx) = mpsc::unbounded();

            let handle = core.handle();
            let dispatcher = match make_dispatcher(handle.clone()) {
                Ok(dispatcher) => {
                    if let Err(_) = init_tx.send(Ok(Reactor { request_sender: tx })) {
                        panic!("failed to send back reactor");
                    }
                    Rc::new(dispatcher)
                },
                Err(err) => {
                    if let Err(_) = init_tx.send(Err(err)) {
                        panic!("failed to send back reactor");
                    }
                    return;
                }
            };

            let future = rx.for_each(move |reactor_request: ReactorRequest| {
                let dispatcher = dispatcher.clone();
                handle.spawn_fn(move || {
                    let signed_request = reactor_request.signed_request;
                    let response_sender = reactor_request.response_sender;
                    dispatcher.dispatch(signed_request).then(move |result| {
                        Ok(drop(response_sender.send(result)))
                    })
                });
                Ok(())
            });

            core.run(future).expect("reactor execution failed");
        });

        init_rx.wait().expect("failed to initiate reactor")
    }
}

impl DispatchSignedRequest for Reactor {
    type Future = ReactorFuture;

    fn dispatch(&self, signed_request: SignedRequest) -> Self::Future {
        let (tx, rx) = oneshot::channel();
        let request = ReactorRequest {
            signed_request: signed_request,
            response_sender: tx
        };
        if let Some(err) = self.request_sender.unbounded_send(request).err() {
            panic!("failed to send request to reactor: {}", err);
        }
        ReactorFuture {
            response_receiver: rx
        }
    }
}
