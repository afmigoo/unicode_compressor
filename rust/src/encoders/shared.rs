use phf::Map;

use super::errors::Error;
use super::types::{Transport, Str2IntDict, Int2StrDict};
use super::packing;
use super::strategies;

use crate::options::TokenizationStrategy;

pub fn inventory_str2int_get(token: &str, inventory: &[&Map<&'static str, u16>]) -> Option<u16> {
    for dict in inventory {
        match dict.get(token) {
            Some(num) => return Some(*num),
            None => continue,
        }
    }
    None
}
pub fn inventory_int2str_get(num: u16, inventory: &[&Map<u16, &'static str>]) -> Option<&'static str> {
    for dict in inventory {
        match dict.get(&num) {
            Some(str) => return Some(str),
            None => continue,
        }
    }
    None
}

pub fn remap_str2int(payload: &str, old2new: &Str2IntDict) -> Result<Vec<u16>, Error> {
    let mut payload_remapped = Vec::new();
    
    for ch in payload.chars() {
        match old2new {
            Str2IntDict::Static(static_map) => {
                match static_map.get(&ch.to_string()) {
                    Some(num) => payload_remapped.push(*num),
                    None => return Err(Error::CharacterNotInAlphabet(ch)),
                }
            },
            Str2IntDict::Inventory(inventory) => {
                match inventory_str2int_get(&ch.to_string(), inventory) {
                    Some(num) => payload_remapped.push(num),
                    None => return Err(Error::CharacterNotInAlphabet(ch)),
                }
            },
        }
    }
    Ok(payload_remapped)
}

pub fn remap_int2str(payload: Vec<u16>, old2new: &Int2StrDict) -> Result<String, Error> {
    let mut payload_remapped = String::new();

    for num in payload {
        match old2new {
            Int2StrDict::Static(static_map) => {
                match static_map.get(&num) {
                    Some(str) => payload_remapped.push_str(str),
                    None => return Err(Error::NumberNotInAlphabet(num)),
                }
            },
            Int2StrDict::Inventory(inventory) => {
                match inventory_int2str_get(num, inventory) {
                    Some(str) => payload_remapped.push_str(str),
                    None => return Err(Error::NumberNotInAlphabet(num)),
                }
            },
        }
    }

    Ok(payload_remapped)
}

pub fn encode(payload: Vec<u16>, transport: &Transport, plain_map: &Map<u16, &'static str>) -> Result<String, Error> {
    match transport {
        Transport::UTF8 => {
            remap_int2str(payload, &Int2StrDict::Static(plain_map))
        }
        Transport::BIN(bits) => {
            let packed_bits = packing::pack_int(&payload, *bits)?;
            Ok(packing::base91_encode(&packed_bits))
        }
    }
}

pub fn decode(
    payload: &str, transport: &Transport,
    plain_map: &'static Map<&'static str, u16>, dict_inventory: &'static [&Map<u16, &'static str>]
) -> Result<String, Error> {
    match transport {
        Transport::UTF8 => {
            let ints = remap_str2int(payload, &Str2IntDict::Static(plain_map))?;
            remap_int2str(ints, &Int2StrDict::Inventory(dict_inventory))
        }
        Transport::BIN(_) => {
            let packed_bits = packing::base91_decode(payload);
            let ints = packing::unpack_int(packed_bits);
            remap_int2str(ints, &Int2StrDict::Inventory(dict_inventory))
        }
    }
}

pub fn get_alphabet(dict_inventory: &[&Map<&'static str, u16>]) -> String {
    let mut alphabet = String::new();
    for dict in dict_inventory {
        for tkn in dict.keys() {
            if tkn.chars().count() == 1 {
                alphabet.push_str(tkn);
            }
        }
    }
    alphabet
}

pub fn covered_by_alphabet(payload: &str, dict_inventory: &'static [&Map<&'static str, u16>]) -> Result<(), Error> {
    match remap_str2int(payload, &Str2IntDict::Inventory(dict_inventory)) {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}

pub fn best_bits_for_payload(payload: &str, dict_inventory: &[&Map<&'static str, u16>], token_max_chars: usize, tokenization_strategy: TokenizationStrategy) -> Result<(Vec<u16>, usize), Error> {
    let mut best_bits = None;
    let mut dict_size = 0;

    for i in 0..dict_inventory.len() {
        dict_size += dict_inventory[i].len();
        let ints_result = match tokenization_strategy {
            TokenizationStrategy::FirstMatch => strategies::first_match_utf(
                &payload, &dict_inventory[..i+1], token_max_chars
            ),
            TokenizationStrategy::LongestMatch => strategies::longest_match_utf(&
                &payload, &dict_inventory[..i+1], token_max_chars
            ),
        };
        let ints = match ints_result {
            Ok(ints) => ints,
            Err(_) => continue,
        };

        let bits_per_int = dict_size.next_power_of_two().ilog2() as usize;
        let bits_total = bits_per_int * ints.len();
        best_bits = match best_bits {
            None => Some((bits_total, bits_per_int, ints)),
            Some(least) => if bits_total < least.0 { Some((bits_total, bits_per_int, ints)) } else { Some(least) },
        }
    }
    
    match best_bits {
        None => Err(Error::CouldNotEncode(payload.to_string())),
        Some(best) => Ok((best.2, best.1)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use phf::{phf_map, Map};

    use crate::options::TokenizationStrategy;

    const TOKEN_MAX_CHARS: usize = 16;

    static CHARS_DICT: Map<&'static str, u16> = phf_map! {
        "a" => 1u16,
        "b" => 2u16,
        "c" => 3u16,
    };
    static WORD_DICT: Map<&'static str, u16> = phf_map! {
        "ab" => 1u16,
    };
    static PHRASE_DICT: Map<&'static str, u16> = phf_map! {
        "abc" => 32u16,
    };
    static Y_ONLY_DICT: Map<&'static str, u16> = phf_map! {
        "y" => 1u16,
    };
    static X_DICT: Map<&'static str, u16> = phf_map! {
        "x" => 1u16,
    };

    fn assert_best(
        payload: &str,
        inventory: &[&Map<&'static str, u16>],
        strategy: TokenizationStrategy,
        expected_ints: &[u16],
        expected_bits: usize,
    ) {
        let (ints, bits) = best_bits_for_payload(payload, inventory, TOKEN_MAX_CHARS, strategy)
            .unwrap_or_else(|e| panic!("best_bits_for_payload({payload:?}): {e}"));
        assert_eq!(ints, expected_ints, "payload = {payload:?}");
        assert_eq!(bits, expected_bits, "payload = {payload:?}");
    }

    fn assert_could_not_encode(
        payload: &str,
        inventory: &[&Map<&'static str, u16>],
        strategy: TokenizationStrategy,
    ) {
        let err = best_bits_for_payload(payload, inventory, TOKEN_MAX_CHARS, strategy)
            .unwrap_err();
        assert!(
            matches!(err, Error::CouldNotEncode(_)),
            "expected CouldNotEncode, got {err}"
        );
    }

    #[test]
    fn best_bits_single_dict_chars_first_match() {
        let inv = &[&CHARS_DICT];
        assert_best("abc", inv, TokenizationStrategy::FirstMatch, &[1, 2, 3], 2);
    }

    #[test]
    fn best_bits_single_dict_chars_longest_match() {
        let inv = &[&CHARS_DICT];
        assert_best("abc", inv, TokenizationStrategy::LongestMatch, &[1, 2, 3], 2);
    }

    #[test]
    fn best_bits_prefers_smaller_prefix_when_cheaper() {
        let inv = &[&WORD_DICT, &CHARS_DICT];
        // prefix [WORD]: 1 int, dict_size 1 -> ilog2(next_pow2(1)) = 0, score 0
        // prefix [WORD, CHARS]: 1 int, dict_size 4 -> bits 2, score 2
        assert_best("ab", inv, TokenizationStrategy::FirstMatch, &[1], 0);
    }

    #[test]
    fn best_bits_prefers_more_dicts_when_shorter_tokenization_wins_first_match() {
        let inv = &[&CHARS_DICT, &PHRASE_DICT];
        assert_best("abc", inv, TokenizationStrategy::FirstMatch, &[32], 2);
    }

    #[test]
    fn best_bits_prefers_more_dicts_when_shorter_tokenization_wins_longest_match() {
        let inv = &[&CHARS_DICT, &PHRASE_DICT];
        assert_best("abc", inv, TokenizationStrategy::LongestMatch, &[32], 2);
    }

    #[test]
    fn best_bits_skips_failed_prefix_uses_later() {
        let inv = &[&Y_ONLY_DICT, &X_DICT];
        assert_best("x", inv, TokenizationStrategy::FirstMatch, &[1], 1);
    }

    #[test]
    fn best_bits_encode_failure() {
        let inv = &[&CHARS_DICT];
        assert_could_not_encode("zzz", inv, TokenizationStrategy::FirstMatch);
    }

    #[test]
    fn best_bits_empty_inventory() {
        let inv: &[&Map<&'static str, u16>] = &[];
        assert_could_not_encode("a", inv, TokenizationStrategy::FirstMatch);
    }

    #[test]
    fn best_bits_smoke_en_32() {
        use super::super::dictionaries::en_32;

        let inv = en_32::TOKEN2INT;
        assert_best("abc", inv, TokenizationStrategy::FirstMatch, &[1, 2, 3], 5);
    }
}
