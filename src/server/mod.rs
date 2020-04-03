mod add_new_user;
mod login;
use hyper::{Body, HeaderMap, Method, Request, Response, Server, StatusCode};
mod utils;
use crate::controller::Controller;
use crate::database;

pub struct Router {
    controller: Controller,
}

impl Router {
    pub fn new(controller: Controller) -> Self {
        Router { controller }
    }
}

pub async fn new_server(req: Request<Body>) -> Result<Response<Body>, String> {
    let db = database::UserCollection::new();
    let controller = Controller::new(db);
    let handler = Router::new(controller);

    let mut response = Response::new(Body::empty());
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/login") => match handler.login_user(req).await {
            Ok(resp) => {
                *response.status_mut() = StatusCode::CREATED;
                Ok(resp)
            }
            Err(e) => {
                println!("{}", e);
                *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                Err(e)
            }
        },
        _ => Ok(response),
    }
}
