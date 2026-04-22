use wasm_bindgen::prelude::*;

mod encoders;
mod dictionaries;

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
