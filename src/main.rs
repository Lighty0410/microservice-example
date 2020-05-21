use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use std::net::SocketAddr;
mod controller;
mod database;
mod model;
mod server;
mod utils;
use crate::database::UserDB;
use controller::Controller;
use server::Router;
use std::convert::Infallible;

type RequestBody = Request<Body>;

#[tokio::main]
async fn main() {
    let user_db = UserDB::new_default();
    let controller = Controller::new(user_db);
    let router = &Router::new(controller);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| {
        let router = router.clone();

        async {
            Ok::<_, String>(service_fn(move |req: RequestBody| {
                let mut router = router.clone();
                async move { Ok::<_, Infallible>(router.new_server(req).await?) }
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
