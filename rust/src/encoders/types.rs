use phf::Map;

use super::errors::Error;
use super::shared;
use super::strategies;

use crate::options::{EncodeOptions, TokenizationStrategy};

pub trait Encoder: Send + Sync {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error>;
    fn decode(&self, payload: &str) -> Result<String, Error>;
    fn can_encode(&self, payload: &str) -> Result<(), Error>;
}

pub struct MapEncoder {
    pub token2encoded: &'static Map<&'static str, &'static str>,
    pub encoded2token: &'static Map<&'static str, &'static str>,
} impl Encoder for MapEncoder {
    fn encode(&self, payload: &str, _: &EncodeOptions) -> Result<String, Error> {
        shared::remap(payload, self.token2encoded)
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        shared::remap(payload, self.encoded2token)
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        shared::covered_by_alphabet(payload, self.token2encoded)
    }
}

pub struct TokenEncoder {
    pub token2encoded: &'static Map<&'static str, &'static str>,
    pub encoded2token: &'static Map<&'static str, &'static str>,
    pub token_max_chars: u8,
} impl Encoder for TokenEncoder {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error> {
        match options.tokenization_strategy {
            TokenizationStrategy::FirstMatch => return strategies::first_match(&payload, self.token2encoded, self.token_max_chars as usize),
            TokenizationStrategy::LongestMatch => return strategies::longest_match(&payload, self.token2encoded, self.token_max_chars as usize),
        }
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        shared::remap(payload, self.encoded2token)
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        shared::covered_by_alphabet(payload, self.token2encoded)
    }
}

pub struct EchoEncoder;
impl Encoder for EchoEncoder {
    fn encode(&self, payload: &str, _: &EncodeOptions) -> Result<String, Error> {
        Ok(payload.to_string())
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        Ok(payload.to_string())
    }
    fn can_encode(&self, _: &str) -> Result<(), Error> {
        Ok(())
    }
}

pub struct AdaptiveEncoder<'a> {
    pub encoders_inventory: &'a[(char, &'a dyn Encoder)],
} impl Encoder for AdaptiveEncoder<'_> {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error> {
        let mut shortest_encoded = None;
        for (id, encoder) in self.encoders_inventory {
            if encoder.can_encode(payload).is_ok() {
                let encoded = match encoder.encode(payload, options) {
                    Ok(encoded) => id.to_string() + &encoded,
                    Err(e) => return Err(e),
                };
                shortest_encoded = match shortest_encoded {
                    None => Some(encoded),
                    Some(shortest) => if encoded.len() < shortest.len() { Some(encoded) } else { Some(shortest) },
                }
            }
        }
        match shortest_encoded {
            None => Err(Error::CouldNotEncode(payload.to_string())),
            Some(encoded) => Ok(encoded),
        }
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        let encoder_id = payload
            .chars().next()
            .ok_or(Error::CouldNotDecodeEmptyString)?;

        let (_, decoder) = self.encoders_inventory
            .iter()
            .find(|&(id, _)| *id == encoder_id)
            .ok_or(Error::DecoderNotFound(encoder_id))?;

        decoder.decode(&payload[1..])
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        for (_, encoder) in self.encoders_inventory {
            if encoder.can_encode(payload).is_ok() {
                return Ok(());
            }
        }
        Err(Error::NoEncoderFitAlphabet(payload.to_string()))
    }
}
