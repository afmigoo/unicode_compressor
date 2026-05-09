use wasm_bindgen::prelude::*;
use crate::encoders;
use crate::options::TokenizationStrategy;
use crate::options::EncodeOptions;

#[wasm_bindgen]
// TODO: ensure errors propagate correctly to js
pub fn encode(payload: &str, encoder: &str, tokenization_strategy: &str) -> Result<String, String> {
    let tokenization_strategy_option = match tokenization_strategy {
        "first-match" => TokenizationStrategy::FirstMatch,
        "longest-match" => TokenizationStrategy::LongestMatch,
        _ => {
            return Err(format!(
                "invalid tokenization strategy '{}'. Supported values: first-match, longest-match.",
                tokenization_strategy
            ));
        }
    };
    let opts = EncodeOptions { tokenization_strategy: tokenization_strategy_option };
    encoders::encode(payload, encoder, &opts).map_err(|e| e.to_string())
}

#[wasm_bindgen]
pub fn decode(payload: &str, encoder: &str) -> Result<String, String> {
    encoders::decode(payload, encoder).map_err(|e| e.to_string())
}

#[wasm_bindgen]
pub fn list_encoders() -> Vec<String> {
    let mut encoders = Vec::new();
    for (name, _) in encoders::instances::NAMED_ENCODERS {
        encoders.push(name.to_string());
    }
    encoders
}
