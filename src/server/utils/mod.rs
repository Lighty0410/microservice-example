use hyper::{Body, Request};
use serde::Deserialize;

pub(in crate::server) async fn parse_body<'a, T: Deserialize<'a>>(
    req: Request<Body>,
    buffer: &'a mut Vec<u8>,
) -> Result<T, String> {
    let body = hyper::body::to_bytes(req.into_body())
        .await
        .or_else(|err| Err(format!("cannot read body: {}", err)))?;

    *buffer = body.iter().cloned().collect::<Vec<u8>>();

    serde_json::from_slice::<T>(buffer)
        .or_else(|err| Err(format!("cannot parse body from json: {}", err)))
}
