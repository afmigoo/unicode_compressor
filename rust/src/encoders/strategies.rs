use phf::Map;
use std::cmp::min;
use std::collections::LinkedList;
use std::iter::once;

use super::errors::Error;
use super::shared::inventory_str2int_get;

/// First match tokenization encoding algorithm
/// ### Parameters
/// - `payload` - the string to encode
/// - `token2int` - the map of tokens to unicode characters
/// - `token_max_chars` - the maximum number of characters in a token
/// ### Algorithm
/// 1. Starts at the beginning of the payload string (`i=0`)
/// 2. Finds longest match in `token2int` map that starts at index `i`
/// 3. Pushes the matched token to the encoded string, advances `i` to the end of the matched token.
/// 4. Go to step 2.
pub fn first_match_utf(
    payload: &str, 
    token2int: &[&Map<&'static str, u16>],
    token_max_chars: usize
) -> Result<Vec<u16>, Error> {
    let mut encoded = Vec::new();
    let payload_chars: Vec<char> = payload.chars().collect();

    let max_window = min(payload_chars.len(), token_max_chars);
    let mut i = 0;
    while i < payload_chars.len() {
        let mut matched_end: Option<usize> = None;
        let max_j = min(i+max_window, payload_chars.len());
        for j in (i+1..max_j+1).rev() {
            let slice: String = payload_chars[i..j].iter().collect();
            if let Some(num) = inventory_str2int_get(&slice, token2int) {
                encoded.push(num);
                matched_end = Some(j);
                break;
            }
        }
        match matched_end {
            Some(end) => i = end,
            None => {
                return Err(Error::CouldNotEncodeSubstring(payload_chars[i..].iter().collect()));
            }
        }
    }
    Ok(encoded)
}

// pub fn first_match_bin(
//     payload: &str, 
//     token2int: &Map<&'static str, &'static str>, 
//     token_max_chars: usize
// ) -> Result<String, Error> {

// }

/// Longest match tokenization encoding algorithm
/// ### Parameters
/// - `payload` - the string to encode
/// - `token2int` - the map of tokens to unicode characters
/// - `token_max_chars` - the maximum number of characters in a token
/// ### Algorithm
/// 1. Finds largest token in the whole payload string
/// 2. Repeats step 1. for the remaining parts of the payload string until the whole payload is encoded.
pub fn longest_match_utf(
    payload: &str, 
    token2int: &[&Map<&'static str, u16>],
    token_max_chars: usize
) -> Result<Vec<u16>, Error> {
    let mut encoded = Vec::new();
    if payload.len() == 0 {
        return Ok(encoded);
    }
    
    let payload_char_idx: Vec<usize> = payload
        .char_indices()
        .map(|(i, _)| i)
        .chain(once(payload.len()))
        .collect();

    let mut regions: LinkedList<Region> = LinkedList::new();
    regions.push_back(subregion(
        payload, &payload_char_idx,
        token2int,
        (0, payload_char_idx.len() - 1),
        token_max_chars
    )?);

    while regions.len() > 0  {
        let mut curr_region: &Region = regions.back().unwrap();
        // going deeper into left subregion
        if curr_region.token_bounds.0 != curr_region.bounds.0 {
            let left_region = subregion(
                payload, &payload_char_idx,
                token2int,
                (curr_region.bounds.0, curr_region.token_bounds.0),
                token_max_chars
            )?;
            regions.push_back(left_region);
            continue;
        } else {
            // check if any right subregions can be found
            while curr_region.token_bounds.1 == curr_region.bounds.1 {
                encoded.push(curr_region.resolved_num);
                regions.pop_back();
                curr_region = match regions.back() {
                    Some(region) => region,
                    None => return Ok(encoded),
                };
            }
            // going deeper into right subregion
            let right_region = subregion(
                payload, &payload_char_idx,
                token2int,
                (curr_region.token_bounds.1, curr_region.bounds.1),
                token_max_chars
            )?;
            encoded.push(curr_region.resolved_num);
            regions.pop_back();
            regions.push_back(right_region);
            continue;
        }
    }
    return Ok(encoded);
}

#[derive(Debug)]
struct Region {
    bounds: (usize, usize),
    token_bounds: (usize, usize),
    resolved_num: u16,
}

fn subregion(
    payload: &str,
    payload_char_idx: &Vec<usize>,
    token2int: &[&Map<&'static str, u16>],
    i_bounds: (usize, usize),
    max_window: usize
) -> Result<Region, Error> {
    let max_window = min(max_window, i_bounds.1 - i_bounds.0 + 1);
    for w_size in (1..max_window+1).rev() {
        // TODO: iterate only over existing sizes
        for i in i_bounds.0..i_bounds.1-(w_size-1) {
            let slice = &payload[payload_char_idx[i]..payload_char_idx[i+w_size]];
            if let Some(num) = inventory_str2int_get(slice, token2int) {
                return Ok(Region {
                    bounds: i_bounds,
                    token_bounds: (i, i+w_size),
                    resolved_num: num,
                });
            }
        }
    }
    return Err(
        Error::CouldNotEncodeSubstring(
            payload[payload_char_idx[i_bounds.0]..payload_char_idx[i_bounds.1]].to_string()
        )
    );
}
