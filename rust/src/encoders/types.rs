use std::collections::HashSet;
use phf::Map;

use super::errors::Error;
use super::shared;
use super::dictionaries::plain_map;

use crate::options::{EncodeOptions};

pub enum Transport {
    UTF8,
    BIN(usize),
}

pub enum Str2IntDict<'a> {
    Inventory(&'a [&'static Map<&'static str, u16>]),
    Static(&'a Map<&'static str, u16>),
}
pub enum Int2StrDict<'a> {
    Inventory(&'a [&'static Map<u16, &'static str>]),
    Static(&'a Map<u16, &'static str>),
}

pub trait Encoder: Send + Sync {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error>;
    fn decode(&self, payload: &str) -> Result<String, Error>;
    fn get_alphabet(&self) -> String;
    fn can_encode(&self, payload: &str) -> Result<(), Error>;
}

pub struct MapEncoder {
    pub token2int: &'static [&'static Map<&'static str, u16>],
    pub int2token: &'static [&'static Map<u16, &'static str>],
    pub transport: Transport,
} impl Encoder for MapEncoder {
    fn encode(&self, payload: &str, _: &EncodeOptions) -> Result<String, Error> {
        let ints = shared::remap_str2int(payload, &Str2IntDict::Inventory(self.token2int))?;
        shared::encode(ints, &self.transport, &plain_map::INT2UTF)
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        shared::decode(payload, &self.transport, &plain_map::UTF2INT, self.int2token)
    }
    fn get_alphabet(&self) -> String {
        shared::get_alphabet(self.token2int)
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        shared::covered_by_alphabet(payload, self.token2int)
    }
}


pub struct TokenEncoder {
    pub token2int: &'static [&'static Map<&'static str, u16>],
    pub int2token: &'static [&'static Map<u16, &'static str>],
    pub transport: Transport,
    pub token_max_chars: u8,
} impl Encoder for TokenEncoder {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error> {
        let (ints, bits) = shared::best_bits_for_payload(payload, self.token2int, self.token_max_chars as usize, options.tokenization_strategy)?;
        let transport = match self.transport {
            Transport::UTF8 => Transport::UTF8,
            Transport::BIN(_) => Transport::BIN(bits),
        };
        shared::encode(ints, &transport, &plain_map::INT2UTF)
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        shared::decode(payload, &self.transport, &plain_map::UTF2INT, &self.int2token)
    }
    fn get_alphabet(&self) -> String {
        shared::get_alphabet(&self.token2int)
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
    fn get_alphabet(&self) -> String {
        String::new()
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
    fn get_alphabet(&self) -> String {
        let mut alphabet = HashSet::<char>::new();
        for (_, encoder) in self.encoders_inventory {
            for ch in encoder.get_alphabet().chars() {
                alphabet.insert(ch);
            }
        }
        String::from_iter(alphabet.iter())
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
