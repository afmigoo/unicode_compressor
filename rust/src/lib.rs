pub mod encoders;
pub mod dictionaries;
pub mod options;


/// Wasm bindings
#[cfg(target_arch = "wasm32")]
mod wasm_api;
#[cfg(target_arch = "wasm32")]
pub use wasm_api::*;
