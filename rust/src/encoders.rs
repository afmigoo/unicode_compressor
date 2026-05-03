use crate::options::EncodeOptions;

mod shared;
mod strategies;

pub mod errors; use errors::Error;
pub mod types; use types::Encoder;
pub mod instances;

#[cfg(test)]
mod test;

pub fn get_encoder(encoder: &str) -> Result<&dyn Encoder, Error> {
  match instances::NAMED_ENCODERS.iter().find(|(name, _)| *name == encoder) {
    Some((_, encoder)) => Ok(*encoder),
    None => Err(Error::EncoderNotFound(encoder.to_string())),
  }
}

pub fn encode(payload: &str, encoder: &str, options: &EncodeOptions) -> Result<String, Error> {
  let encoder = get_encoder(&encoder)?;
  match encoder.can_encode(payload) {
    Ok(_) => encoder.encode(payload, options),
    Err(e) => Err(e),
  }
}

pub fn decode(payload: &str, encoder: &str) -> Result<String, Error> {
  let encoder = get_encoder(&encoder)?;
  encoder.decode(payload)
}
