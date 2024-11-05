use base64::Engine;
use wasm_minimal_protocol::*;

initiate_protocol!();

#[wasm_func]
pub fn encode64(data: &[u8], flags: &[u8]) -> Vec<u8> {
    let pad = flags.get(0) == Some(&1);
    let url = flags.get(1) == Some(&1);

    let engine = match (pad, url) {
        (true, true) => base64::prelude::BASE64_URL_SAFE,
        (true, false) => base64::prelude::BASE64_STANDARD,
        (false, true) => base64::prelude::BASE64_URL_SAFE_NO_PAD,
        (false, false) => base64::prelude::BASE64_STANDARD_NO_PAD,
    };

    engine.encode(data).into_bytes()
}

#[wasm_func]
pub fn decode64(data: &[u8], flags: &[u8]) -> Result<Vec<u8>, String> {
    let pad = flags.get(0) == Some(&1);
    let url = flags.get(1) == Some(&1);

    let engine = match (pad, url) {
        (true, true) => base64::prelude::BASE64_URL_SAFE,
        (true, false) => base64::prelude::BASE64_STANDARD,
        (false, true) => base64::prelude::BASE64_URL_SAFE_NO_PAD,
        (false, false) => base64::prelude::BASE64_STANDARD_NO_PAD,
    };

    engine.decode(data).map_err(|e| e.to_string())
}

#[wasm_func]
pub fn encode32(data: &[u8], flags: &[u8]) -> Vec<u8> {
    let pad = flags.get(0) == Some(&1);
    let hex = flags.get(1) == Some(&1);

    let alphabet = match hex {
        true => base32::Alphabet::Rfc4648Hex { padding: pad },
        false => base32::Alphabet::Rfc4648 {padding: pad }
    };

    base32::encode(alphabet, data).into_bytes()
}

#[wasm_func]
pub fn decode32(data: &[u8], flags: &[u8]) -> Result<Vec<u8>, String> {
    let pad = flags.get(0) == Some(&1);
    let hex = flags.get(1) == Some(&1);

    let alphabet = match hex {
        true => base32::Alphabet::Rfc4648Hex { padding: pad },
        false => base32::Alphabet::Rfc4648 {padding: pad }
    };

    let string = std::str::from_utf8(data).map_err(|e| e.to_string())?;
    base32::decode(alphabet, string).ok_or("invalid base32 string".to_string())
}

#[wasm_func]
pub fn encode16(data: &[u8]) -> Vec<u8> {
    hex::encode(data).into_bytes()
}

#[wasm_func]
pub fn decode16(data: &[u8]) -> Result<Vec<u8>, String> {
    hex::decode(data).map_err(|e| e.to_string())
}
