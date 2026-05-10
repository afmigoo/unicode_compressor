use phf::Map;

use super::errors::Error;
use super::shared;
use super::strategies;
use super::packing;
use super::dictionaries::plain_map;

use crate::options::{EncodeOptions, TokenizationStrategy};

pub enum Transport {
    UTF8,
    BIN(usize),
}

pub trait Encoder: Send + Sync {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error>;
    fn decode(&self, payload: &str) -> Result<String, Error>;
    fn can_encode(&self, payload: &str) -> Result<(), Error>;
}

pub struct MapEncoder {
    pub token2int: &'static Map<&'static str, u16>,
    pub int2token: &'static Map<u16, &'static str>,
    pub transport: Transport,
} impl Encoder for MapEncoder {
    fn encode(&self, payload: &str, _: &EncodeOptions) -> Result<String, Error> {
        let ints = shared::remap_utf2int(payload, self.token2int)?;
        match self.transport {
            Transport::UTF8 => {
                shared::remap_int2utf(ints, &plain_map::INT2UTF)
            }
            Transport::BIN(bits) => {
                let packed_bits = packing::pack_int(&ints, bits)?;
                Ok(packing::base91_encode(&packed_bits))
            }
        }
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        match self.transport {
            Transport::UTF8 => {
                let ints = shared::remap_utf2int(payload, &plain_map::UTF2INT)?;
                shared::remap_int2utf(ints, self.int2token)
            }
            Transport::BIN(bits) => {
                let packed_bits = packing::base91_decode(payload);
                let ints = packing::unpack_int(packed_bits, bits);
                shared::remap_int2utf(ints, &self.int2token)
            }
        }
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        shared::covered_by_alphabet(payload, self.token2int)
    }
}


pub struct TokenEncoder {
    pub token2int: &'static Map<&'static str, u16>,
    pub int2token: &'static Map<u16, &'static str>,
    pub transport: Transport,
    pub token_max_chars: u8,
} impl Encoder for TokenEncoder {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error> {
        let ints = match options.tokenization_strategy {
            TokenizationStrategy::FirstMatch => strategies::first_match_utf(
                &payload, self.token2int, self.token_max_chars as usize
            ),
            TokenizationStrategy::LongestMatch => strategies::longest_match_utf(&
                &payload, self.token2int, self.token_max_chars as usize
            ),
        }?;
        match self.transport {
            Transport::UTF8 => {
                shared::remap_int2utf(ints, &plain_map::INT2UTF)
            }
            Transport::BIN(bits) => {
                let packed_bits = packing::pack_int(&ints, bits)?;
                Ok(packing::base91_encode(&packed_bits))
            }
        }
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        match self.transport {
            Transport::UTF8 => {
                let ints = shared::remap_utf2int(payload, &plain_map::UTF2INT)?;
                shared::remap_int2utf(ints, self.int2token)
            }
            Transport::BIN(bits) => {
                let packed_bits = packing::base91_decode(payload);
                let ints = packing::unpack_int(packed_bits, bits);
                shared::remap_int2utf(ints, &self.int2token)
            }
        }
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        shared::covered_by_alphabet(payload, self.token2int)
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
