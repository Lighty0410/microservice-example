use anyhow::Result;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::str;

pub(super) fn encode_string_base64(password: &mut String) {
    let for_encode = password.clone();
    *password = base64::encode(for_encode)
}

pub(super) fn decode_string_base64(password: &str) -> Result<String> {
    let decoded = base64::decode(password)?;

    let password = str::from_utf8(&decoded)?;

    Ok(password.to_string())
}

pub(super) fn generate_token() -> String {
    let mut token = rand::thread_rng()
        .sample_iter(Alphanumeric)
        .take(7)
        .collect::<String>();
    encode_string_base64(&mut token);

    token
}
