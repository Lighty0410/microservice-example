use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Server};
use std::net::SocketAddr;
mod controller;
mod database;
mod model;
mod server;
mod utils;
use controller::Controller;
use server::Router;
use std::convert::Infallible;

type RequestBody = Request<Body>;

#[tokio::main]
async fn main() {
    let mongo_db = database::build_mongo();
    let redis_conn = database::build_redis();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let make_svc = make_service_fn(move |_conn| {
        let mongo_db = mongo_db.clone();
        let redis_conn = redis_conn.clone();

        async move {
            Ok::<_, String>(service_fn(move |req: RequestBody| {
                let mongo_db = mongo_db.clone();
                let redis_conn = redis_conn.clone();

                let controller = Controller::new(mongo_db, redis_conn);
                let mut router = Router::new(controller);

                async move { Ok::<_, Infallible>(router.new_server(req).await?) }
            }))
        }
    });
    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
