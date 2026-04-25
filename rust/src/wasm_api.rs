use wasm_bindgen::prelude::*;
use crate::encoders;
use crate::options::CompressionLevel;
use crate::options::EncodeOptions;

#[wasm_bindgen]
// TODO: ensure errors propagate correctly to js
pub fn encode(payload: &str, encoder: &str, level: &str) -> Result<String, String> {
    let level = match level {
        "fast" => CompressionLevel::Fast,
        "balanced" => CompressionLevel::Balanced,
        _ => {
            return Err(format!(
                "invalid encode level '{}'. Supported values: fast, balanced.",
                level
            ));
        }
    };
    let opts = EncodeOptions { level };
    encoders::encode(payload, encoder, &opts).map_err(|e| e.to_string())
}

#[wasm_bindgen]
pub fn decode(payload: &str, encoder: &str) -> Result<String, String> {
    encoders::decode(payload, encoder).map_err(|e| e.to_string())
}

#[wasm_bindgen]
pub fn list_encoders() -> Vec<String> {
    let mut encoders = Vec::new();
    for encoder in encoders::consts::ENCODER_NAMES {
        encoders.push(encoder.to_string());
    }
    encoders
}
