mod login;
mod user_router;
use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
mod utils;
use crate::controller::Controller;
use crate::database;

type RequestBody = Request<Body>;
type ResponseBody = Result<Response<Body>, String>;

#[derive(Clone)]
pub struct Router {
    controller: Controller,
}

impl Router {
    pub fn new(controller: Controller) -> Self {
        Router { controller }
    }
}

impl Router {
    pub async fn new_server(&self, req: Request<Body>) -> ResponseBody {
        let mut response = Response::new(Body::empty());
        match (req.method(), req.uri().path()) {
            (&Method::GET, "/login") | (&Method::POST, "/create-user") => {
                self.user_router(req).await
            }
            _ => Ok(response),
        }
    }
}
