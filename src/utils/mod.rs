use serde::Serialize;

pub fn struct_to_json<T: Serialize>(some_struct: T) -> Result<String, String> {
    serde_json::to_string(&some_struct)
        .or_else(|err| Err(format!("cannot encode structure to json: {:?}", err)))
}
