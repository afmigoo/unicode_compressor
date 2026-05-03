use phf::Map;

use super::errors::Error;

pub fn remap(payload: &str, old2new: &Map<&'static str, &'static str>) -> Result<String, Error> {
    let mut payload_remapped = String::new();

    for ch in payload.chars() {
        match old2new.get(&ch.to_string()) {
            Some(ch) => payload_remapped.push_str(ch),
            None => return Err(Error::CharacterNotInAlphabet(ch)),
        }
    }
    Ok(payload_remapped)
}

pub fn covered_by_alphabet(payload: &str, token2unicode: &Map<&'static str, &'static str>) -> Result<(), Error> {
    for ch in payload.chars() {
        match token2unicode.get(&ch.to_string()) {
            Some(_) => continue,
            None => return Err(Error::CharacterNotInAlphabet(ch)),
        }
    }
    Ok(())
}
