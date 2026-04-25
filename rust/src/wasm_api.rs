use wasm_bindgen::prelude::*;
use crate::encoders;
use crate::options::CompressionLevel;
use crate::options::EncodeOptions;

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn encode(payload: &str, encoder: &str, level: &str) -> String {
    let opts = EncodeOptions {
        level: match level {
            "fast" => CompressionLevel::Fast,
            "balanced" => CompressionLevel::Balanced,
            _ => return "<failed>".to_string(),
        },
    };
    match encoders::encode(payload, encoder, &opts) {
        Ok(processed_payload) => processed_payload,
        Err(_) => "<failed>".to_string(),
    }
}

#[wasm_bindgen]
pub fn decode(payload: &str, encoder: &str) -> String {
    match encoders::decode(payload, encoder) {
        Ok(processed_payload) => processed_payload,
        Err(_) => "<failed>".to_string(),
    }
}

#[wasm_bindgen]
pub fn list_encoders() -> Vec<String> {
    let mut encoders = Vec::new();
    for encoder in encoders::consts::ENCODER_NAMES {
        encoders.push(encoder.to_string());
    }
    encoders
}
