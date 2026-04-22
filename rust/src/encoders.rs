use std::collections::HashMap;
use std::sync::LazyLock;

use crate::dictionaries;

pub mod errors;
pub mod traits;

mod map_encoder;
mod token_encoder;

pub fn process_payload(payload: &str, encoder: &str, decode: bool) -> Result<String, errors::Error> {
  let encoder = match NAMED_ENCODERS.get(encoder) {
    Some(encoder) => encoder,
    None => return Err(errors::Error::EncoderNotFound(encoder.to_string())),
  };
  
  let processed_payload = if decode {
    encoder.decode(payload)?
  } else {
    encoder.encode(payload)?
  };

  return Ok(processed_payload);
}

pub static NAMED_ENCODERS: LazyLock<HashMap<&'static str, Box<dyn traits::Encoder>>> = 
LazyLock::new(|| {
  HashMap::from([
    ("bpe_dummy",
      Box::new(map_encoder::MapEncoder {
        token2unicode: &dictionaries::bpe::dummy::TOKEN2UNICODE,
        unicode2token: &dictionaries::bpe::dummy::UNICODE2TOKEN,
      }) as Box<dyn traits::Encoder>),
    ("bpe_coding",
      Box::new(token_encoder::TokenEncoder {
        token2unicode: &dictionaries::bpe::coding::TOKEN2UNICODE,
        unicode2token: &dictionaries::bpe::coding::UNICODE2TOKEN,
        token_max_chars: dictionaries::bpe::coding::TOKEN_MAX_CHARS,
      }) as Box<dyn traits::Encoder>),
    ("bpe_meshcoretel_ru",
      Box::new(token_encoder::TokenEncoder {
        token2unicode: &dictionaries::bpe::meshcoretel_ru::TOKEN2UNICODE,
        unicode2token: &dictionaries::bpe::meshcoretel_ru::UNICODE2TOKEN,
        token_max_chars: dictionaries::bpe::meshcoretel_ru::TOKEN_MAX_CHARS,
      }) as Box<dyn traits::Encoder>),
    ("bpe_wiki_en",
      Box::new(token_encoder::TokenEncoder {
        token2unicode: &dictionaries::bpe::wiki_en::TOKEN2UNICODE,
        unicode2token: &dictionaries::bpe::wiki_en::UNICODE2TOKEN,
        token_max_chars: dictionaries::bpe::wiki_en::TOKEN_MAX_CHARS,
      }) as Box<dyn traits::Encoder>),
    ("bpe_wiki_ru",
      Box::new(token_encoder::TokenEncoder {
        token2unicode: &dictionaries::bpe::wiki_ru::TOKEN2UNICODE,
        unicode2token: &dictionaries::bpe::wiki_ru::UNICODE2TOKEN,
        token_max_chars: dictionaries::bpe::wiki_ru::TOKEN_MAX_CHARS,
      }) as Box<dyn traits::Encoder>),
    ("bpe_wiki",
      Box::new(token_encoder::TokenEncoder {
        token2unicode: &dictionaries::bpe::wiki::TOKEN2UNICODE,
        unicode2token: &dictionaries::bpe::wiki::UNICODE2TOKEN,
        token_max_chars: dictionaries::bpe::wiki::TOKEN_MAX_CHARS,
      }) as Box<dyn traits::Encoder>),
  ])
});
