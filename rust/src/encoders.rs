use crate::options::EncodeOptions;

pub mod errors; use errors::Error;
pub mod traits; use traits::Encoder;
pub mod consts;

mod map_encoder;
mod token_encoder;

#[cfg(test)]
mod test;

pub fn get_encoder(encoder: &str) -> Result<&dyn Encoder, Error> {
  match consts::NAMED_ENCODERS.iter().find(|(name, _)| *name == encoder) {
    Some((_, encoder)) => Ok(*encoder),
    None => Err(Error::EncoderNotFound(encoder.to_string())),
  }
}

pub fn encode(payload: &str, encoder: &str, options: &EncodeOptions) -> Result<String, Error> {
  let encoder = get_encoder(&encoder)?;
  encoder.encode(payload, options)
}

pub fn decode(payload: &str, encoder: &str) -> Result<String, Error> {
  let encoder = get_encoder(&encoder)?;
  encoder.decode(payload)
}
