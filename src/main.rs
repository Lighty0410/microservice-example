use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use hyper::{HeaderMap, Method, StatusCode};
use std::net::SocketAddr;
mod controller;
mod database;
mod model;
mod server;
use controller::Controller;

type RequestBody = Request<Body>;

#[tokio::main]
async fn main() {
    let user_collection = database::UserCollection::new();
    let controller = controller::Controller::new(user_collection);
    let router = server::Router::new(controller);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(move |_conn| {
        let router = router.clone();
        async move {
            Ok::<_, String>(service_fn(move |req: RequestBody| {
                let router = router.clone();
                async move { Ok::<_, String>(router.new_server(req).await?) }
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
