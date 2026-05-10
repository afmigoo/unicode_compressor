use phf::Map;

use super::errors::Error;
use super::types::Transport;
use super::packing;

pub fn remap_utf2int(payload: &str, old2new: &Map<&'static str, u16>) -> Result<Vec<u16>, Error> {
    let mut payload_remapped = Vec::new();

    for ch in payload.chars() {
        match old2new.get(&ch.to_string()) {
            Some(num) => payload_remapped.push(*num),
            None => return Err(Error::CharacterNotInAlphabet(ch)),
        }
    }
    Ok(payload_remapped)
}

pub fn remap_int2utf(payload: Vec<u16>, old2new: &Map<u16, &'static str>) -> Result<String, Error> {
    let mut payload_remapped = String::new();

    for num in payload.iter() {
        match old2new.get(num) {
            Some(ch) => payload_remapped.push_str(*ch),
            None => return Err(Error::NumberNotInAlphabet(*num)),
        }
    }
    Ok(payload_remapped)
}

pub fn encode(payload: Vec<u16>, transport: &Transport, int2utf: &Map<u16, &'static str>) -> Result<String, Error> {
    match transport {
        Transport::UTF8 => {
            remap_int2utf(payload, &int2utf)
        }
        Transport::BIN(bits) => {
            let packed_bits = packing::pack_int(&payload, *bits)?;
            Ok(packing::base91_encode(&packed_bits))
        }
    }
}

pub fn decode(payload: &str, transport: &Transport, utf2int: &Map<&'static str, u16>, int2token: &Map<u16, &'static str>) -> Result<String, Error> {
    match transport {
        Transport::UTF8 => {
            let ints = remap_utf2int(payload, &utf2int)?;
            remap_int2utf(ints, int2token)
        }
        Transport::BIN(bits) => {
            let packed_bits = packing::base91_decode(payload);
            let ints = packing::unpack_int(packed_bits, *bits);
            remap_int2utf(ints, int2token)
        }
    }
}

pub fn get_alphabet(token2int: &Map<&'static str, u16>) -> String {
    let mut alphabet = String::new();
    for tkn in token2int.keys() {
        if tkn.chars().count() == 1 {
            alphabet.push_str(tkn);
        }
    }
    alphabet
}

pub fn covered_by_alphabet(payload: &str, token2encoded: &Map<&'static str, u16>) -> Result<(), Error> {
    for ch in payload.chars() {
        match token2encoded.get(&ch.to_string()) {
            Some(_) => continue,
            None => return Err(Error::CharacterNotInAlphabet(ch)),
        }
    }
    Ok(())
}
