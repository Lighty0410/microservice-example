use hyper::{Body, Request};
use serde::{Deserialize, Serialize};

pub(super) async fn parse_body<'a, T: Deserialize<'a>>(
    req: Request<Body>,
    buffer: &'a mut Vec<u8>,
) -> Result<T, String> {
    let body = hyper::body::to_bytes(req.into_body())
        .await
        .or_else(|e| Err(format!("cannot read body: {}", e)))?;

    *buffer = body.iter().cloned().collect::<Vec<u8>>();

    serde_json::from_slice::<T>(buffer)
        .or_else(|e| Err(format!("cannot parse body from json: {}", e)))
}

pub fn struct_to_json<T: Serialize>(some_struct: T) -> Result<String, String> {
    serde_json::to_string(&some_struct)
        .or_else(|e| Err(format!("cannot encode structure to json: {:?}", e)))
}

pub fn redis_to_struct<'a, T: Deserialize<'a>>(buffer: &'a [u8]) -> Result<T, String> {
    serde_json::from_slice::<T>(buffer)
        .or_else(|e| Err(format!("cannot decode structure to json: {:?}", e)))
}
