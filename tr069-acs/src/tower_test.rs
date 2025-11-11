// use std::{fmt::Error, future::Future, pin::Pin, process::Output, time::Duration};
//
// struct HttpRequest;
// struct HttpResponse;
// struct RequestHandler;
//
// #[derive(Clone)]
// struct TimeOut<T> {
//     inner_handler: T,
//     duration: Duration,
// }
// trait Handler<Request> {
//     // type Future: Future<Output = Result<HttpResponse, Error>>; // The output of this function must
//     //                                                            // implement the Future trait
//     // fn call(&mut self, request: HttpRequest) -> Self::Future;
//     type Response;
//
//     type Error;
//
//     type Future: Future<Output = Result<Self::Response, Self::Error>>;
//
//     fn call(&mut self, request: Request) -> Self::Future;
// }
//
// impl Handler<HttpRequest> for RequestHandler {
//     type Response = HttpResponse;
//     type Error = Error;
//     type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
//     fn call(&mut self, request: HttpRequest) -> Self::Future {}
// }
// impl Handler for RequestHandler {
//     type Future = Pin<Box<Future<Output = Result<HttpResponse, Error>>>;
//     fn call(&mut self, request: HttpRequest) -> Self::Future {
//         Box::pin(async move {
//             if request.path()
//         });
//     }
// }

use std::time::Duration;
use tokio::time::sleep;
use tower::Service;

#[derive(Debug, Clone)]
struct TimeOut<S> {
    inner: S,
    timeout: Duration,
}

impl<S> TimeOut<S> {
    pub fn new(inner: S, timeout: Duration) -> Self {
        TimeOut { inner, timeout }
    }
}

pub struct ResponseFuture<F> {
    response_future: F,
    sleep: Sleep,
}
impl<S, Request> Service<Resquest> for TimeOut<S>
where
    S: Service<Request>,
{
    type Error = S::Error;
    type Response = S::Response;
    type Future = ResponseFuture<S::Future>;
    fn call(&mut self, req: Resquest) -> Self::Future {
        let response_future = self.inner.call(req);
        let sleep = tokio::time::sleep(self.timeout);

        ResponseFuture {
            respose_future: respose_future,
            sleep,
        }
    }

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }
}
// // impl<S, Request> Service<Request> for TimeOut<S>
// // where
// //     S: Service<Request>,
// // {
// //     type Response = S::Response;
// //     type Error = S::Error;
// //     type Future = S::Future;
// // }
//
// fn takes_ref<F>(f: F)
// where
//     F: for<'a> Fn(&'a str),
// {
//     let s = String::from("hello");
//     f(&s);
// }
//
// fn takes_ref_hrtb<'a, F>(f: F)
// where
//     F:
