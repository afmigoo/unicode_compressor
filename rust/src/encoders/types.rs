use phf::Map;

use super::errors::Error;
use super::shared;
use super::strategies;
use super::packing;

use crate::options::{EncodeOptions, TokenizationStrategy};

pub trait Encoder: Send + Sync {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error>;
    fn decode(&self, payload: &str) -> Result<String, Error>;
    fn can_encode(&self, payload: &str) -> Result<(), Error>;
}

pub struct MapEncoderUTF {
    pub token2encoded: &'static Map<&'static str, &'static str>,
    pub encoded2token: &'static Map<&'static str, &'static str>,
} impl Encoder for MapEncoderUTF {
    fn encode(&self, payload: &str, _: &EncodeOptions) -> Result<String, Error> {
        shared::remap_utf2utf(payload, self.token2encoded)
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        shared::remap_utf2utf(payload, self.encoded2token)
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        shared::covered_by_utf_alphabet(payload, self.token2encoded)
    }
}

pub struct MapEncoderBIN {
    pub token2encoded: &'static Map<&'static str, u16>,
    pub encoded2token: &'static Map<u16, &'static str>,
    pub bits: usize,
} impl Encoder for MapEncoderBIN {
    fn encode(&self, payload: &str, _: &EncodeOptions) -> Result<String, Error> {
        let ints = shared::remap_utf2int(payload, self.token2encoded)?;
        eprintln!("encoded ints: {:?}", &ints);
        let packed_bits = packing::pack_int(&ints, self.bits)?;
        eprintln!("encoded packed bits: {:?}", &packed_bits);
        Ok(packing::base91_encode(&packed_bits))
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        let packed_bits = packing::base91_decode(payload);
        eprintln!("decoded packed bits: {:?}", &packed_bits);
        let ints = packing::unpack_int(packed_bits, self.bits);
        eprintln!("decoded ints: {:?}", &ints);
        shared::remap_int2utf(ints, self.encoded2token)
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        shared::covered_by_bin_alphabet(payload, self.token2encoded)
    }
}

pub struct TokenEncoderUTF {
    pub token2encoded: &'static Map<&'static str, &'static str>,
    pub encoded2token: &'static Map<&'static str, &'static str>,
    pub token_max_chars: u8,
} impl Encoder for TokenEncoderUTF {
    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error> {
        match options.tokenization_strategy {
            TokenizationStrategy::FirstMatch => return strategies::first_match_utf(&payload, self.token2encoded, self.token_max_chars as usize),
            TokenizationStrategy::LongestMatch => return strategies::longest_match_utf(&payload, self.token2encoded, self.token_max_chars as usize),
        }
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        shared::remap_utf2utf(payload, self.encoded2token)
    }
    fn can_encode(&self, payload: &str) -> Result<(), Error> {
        shared::covered_by_utf_alphabet(payload, self.token2encoded)
    }
}

// pub struct TokenEncoderBIN {
//     pub token2encoded: &'static Map<&'static str, &'static str>,
//     pub encoded2token: &'static Map<&'static str, &'static str>,
//     pub token_max_chars: u8,
// } impl Encoder for TokenEncoderBIN {
//     fn encode(&self, payload: &str, _: &EncodeOptions) -> Result<String, Error> {
//         strategies::first_match_bin(&payload, self.token2encoded, self.token_max_chars as usize)
//     }
//     fn decode(&self, payload: &str) -> Result<String, Error> {
//         shared::remap_utf2utf(payload, self.encoded2token)
//     }
//     fn can_encode(&self, payload: &str) -> Result<(), Error> {
//         shared::covered_by_utf_alphabet(payload, self.token2encoded)
//     }
// }

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
