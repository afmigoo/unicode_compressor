pub mod encoders;
pub mod dictionaries;
pub mod options;

/// Wasm bindings
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use encoders::consts::ENCODER_NAMES;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn encode(payload: &str, encoder: &str, level: CompressionLevel) -> String {
    match encoders::process_payload(payload, encoder, false, &EncodeOptions { level: level }) {
        Ok(processed_payload) => processed_payload,
        Err(_) => "<failed>".to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn decode(payload: &str, encoder: &str) -> String {
    match encoders::process_payload(payload, encoder, true) {
        Ok(processed_payload) => processed_payload,
        Err(_) => "<failed>".to_string(),
    }
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn list_encoders() -> Vec<String> {
    let mut encoders = Vec::new();
    for encoder in ENCODER_NAMES {
        encoders.push(encoder.to_string());
    }
    encoders
}
