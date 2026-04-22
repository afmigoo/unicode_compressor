use wasm_bindgen::prelude::*;

mod encoders;
mod dictionaries;

use encoders::consts::ENCODER_NAMES;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn encode(payload: &str, encoder: &str) -> String {
    match encoders::process_payload(payload, encoder, false) {
        Ok(processed_payload) => processed_payload,
        Err(_) => "<failed>".to_string(),
    }
}

#[wasm_bindgen]
pub fn decode(payload: &str, encoder: &str) -> String {
    match encoders::process_payload(payload, encoder, true) {
        Ok(processed_payload) => processed_payload,
        Err(_) => "<failed>".to_string(),
    }
}

#[wasm_bindgen]
pub fn list_encoders() -> Vec<String> {
    let mut encoders = Vec::new();
    for encoder in ENCODER_NAMES {
        encoders.push(encoder.to_string());
    }
    encoders
}
