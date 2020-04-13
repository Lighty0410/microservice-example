use crate::model;
use crate::server::{utils, RequestBody, ResponseBody, Router};
use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
use serde::{Deserialize, Serialize};

impl Router {
    pub(super) async fn user_router(&self, req: RequestBody) -> ResponseBody {
        match (req.method(), req.uri().path()) {
            (&Method::POST, "/create-user") => self.create_user(req).await,
            (&Method::GET, "/login") => Ok(Response::new(Body::from("NOT IMPLEMENTED YET"))),
            _ => Err("unknown endpoint".to_string()),
        }
    }

    pub(super) async fn create_user(&self, req: RequestBody) -> Result<Response<Body>, String> {
        let response = Response::new(Body::empty());
        let user: model::User = utils::parse_body(req, &mut Vec::<u8>::new()).await?;

        self.controller.create_user(user);

        Ok(response)
    }
}
