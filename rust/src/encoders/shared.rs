use phf::Map;

use super::errors::Error;

pub fn remap_utf2utf(payload: &str, old2new: &Map<&'static str, &'static str>) -> Result<String, Error> {
    let mut payload_remapped = String::new();

    for ch in payload.chars() {
        match old2new.get(&ch.to_string()) {
            Some(ch) => payload_remapped.push_str(ch),
            None => return Err(Error::CharacterNotInAlphabet(ch)),
        }
    }
    Ok(payload_remapped)
}

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

pub fn covered_by_utf_alphabet(payload: &str, token2encoded: &Map<&'static str, &'static str>) -> Result<(), Error> {
    for ch in payload.chars() {
        match token2encoded.get(&ch.to_string()) {
            Some(_) => continue,
            None => return Err(Error::CharacterNotInAlphabet(ch)),
        }
    }
    Ok(())
}

pub fn covered_by_bin_alphabet(payload: &str, token2encoded: &Map<&'static str, u16>) -> Result<(), Error> {
    for ch in payload.chars() {
        match token2encoded.get(&ch.to_string()) {
            Some(_) => continue,
            None => return Err(Error::CharacterNotInAlphabet(ch)),
        }
    }
    Ok(())
}
