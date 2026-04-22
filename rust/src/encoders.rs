pub mod errors; use errors::Error;
pub mod traits; use traits::Encoder;
pub mod consts;

mod map_encoder;
mod token_encoder;

pub fn get_encoder(encoder: &str) -> Result<&dyn Encoder, Error> {
  match consts::NAMED_ENCODERS.iter().find(|(name, _)| *name == encoder) {
    Some((_, encoder)) => Ok(*encoder),
    None => Err(Error::EncoderNotFound(encoder.to_string())),
  }
}

pub fn process_payload(payload: &str, encoder: &str, decode: bool) -> Result<String, Error> {
  let encoder = get_encoder(&encoder)?;

  let processed_payload = if decode {
    encoder.decode(payload)?
  } else {
    encoder.encode(payload)?
  };

  return Ok(processed_payload);
}
