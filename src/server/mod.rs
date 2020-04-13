mod login;
mod user_router;
use hyper::{Body, Method, Request, Response};
mod utils;
use crate::controller::Controller;
use std::convert::Infallible;

type RequestBody = Request<Body>;
type ResponseBody = Result<Response<Body>, Infallible>;

pub struct Router {
    controller: Controller,
}

impl Router {
    pub fn new(controller: Controller) -> Self {
        Router { controller }
    }
}

impl Router {
    pub async fn new_server(&mut self, req: Request<Body>) -> ResponseBody {
        let mut response = Response::new(Body::from("unknown endpoint"));
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/login") | (&Method::POST, "/create-user") => {
                Ok(self.user_router(req).await)
            }
            _ => Ok(response),
        }
    }
}
