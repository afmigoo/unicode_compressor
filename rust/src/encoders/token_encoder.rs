use std::cmp::min;
use std::collections::LinkedList;
use std::iter::once;
use phf::Map;

use super::errors::Error;
use super::traits::Encoder;
use crate::options::EncodeOptions;
use crate::options::CompressionLevel;

pub struct TokenEncoder {
    pub token2unicode: &'static Map<&'static str, &'static str>,
    pub unicode2token: &'static Map<&'static str, &'static str>,
    pub token_max_chars: u8,
}

#[derive(Debug)]
struct Region {
    bounds: (usize, usize),
    token_bounds: (usize, usize),
    unicode: &'static str,
}

impl Encoder for TokenEncoder {
    fn token2unicode(&self) -> &'static Map<&'static str, &'static str> {
        &self.token2unicode
    }
    fn unicode2token(&self) -> &'static Map<&'static str, &'static str> {
        &self.unicode2token
    }

    fn encode(&self, payload: &str, options: &EncodeOptions) -> Result<String, Error> {
        match options.level {
            CompressionLevel::Fast => return self.encode_greedy_fast(&payload),
            CompressionLevel::Balanced => return self.encode_greedy(&payload),
        }
    }
}

impl TokenEncoder {
    fn encode_greedy(&self, payload: &str) -> Result<String, Error> {
        let mut encoded = String::new();
        
        let payload_char_idx: Vec<usize> = payload
            .char_indices()
            .map(|(i, _)| i)
            .chain(once(payload.len()))
            .collect();

        let mut regions: LinkedList<Region> = LinkedList::new();
        regions.push_back(self.subregion(
            payload, &payload_char_idx,
            (0, payload_char_idx.len() - 1),
            self.token_max_chars as usize
        )?);

        while regions.len() > 0  {
            let mut curr_region: &Region = regions.back().unwrap();
            // going deeper into left subregion
            if curr_region.token_bounds.0 != curr_region.bounds.0 {
                let left_region = self.subregion(
                    payload, &payload_char_idx,
                    (curr_region.bounds.0, curr_region.token_bounds.0),
                    self.token_max_chars as usize
                )?;
                regions.push_back(left_region);
                continue;
            } else {
                // check if any right subregions can be found
                while curr_region.token_bounds.1 == curr_region.bounds.1 {
                    encoded.push_str(curr_region.unicode);
                    regions.pop_back();
                    curr_region = match regions.back() {
                        Some(region) => region,
                        None => return Ok(encoded),
                    };
                }
                // going deeper into right subregion
                let right_region = self.subregion(
                    payload, &payload_char_idx,
                    (curr_region.token_bounds.1, curr_region.bounds.1),
                    self.token_max_chars as usize
                )?;
                encoded.push_str(curr_region.unicode);
                regions.pop_back();
                regions.push_back(right_region);
                continue;
            }
        }
        return Ok(encoded);
    }

    fn subregion(
        &self,
        payload: &str,
        payload_char_idx: &Vec<usize>,
        i_bounds: (usize, usize),
        max_window: usize
    ) -> Result<Region, Error> {
        let max_window = min(max_window, i_bounds.1 - i_bounds.0 + 1);
        for w_size in (1..max_window+1).rev() {
            for i in i_bounds.0..i_bounds.1-(w_size-1) {
                let slice = &payload[payload_char_idx[i]..payload_char_idx[i+w_size]];
                if let Some(unicode) = self.token2unicode.get(slice) {
                    return Ok(Region {
                        bounds: i_bounds,
                        token_bounds: (i, i+w_size),
                        unicode: unicode,
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

    fn encode_greedy_fast(&self, payload: &str) -> Result<String, Error> {
        let mut encoded = String::new();
        let payload_chars: Vec<char> = payload.chars().collect();
    
        let max_window = min(payload_chars.len(), self.token_max_chars as usize);
        let mut i = 0;
        while i < payload_chars.len() {
            let mut matched_end: Option<usize> = None;
            let max_j = min(i+max_window, payload_chars.len());
            for j in (i+1..max_j+1).rev() {
                let slice: String = payload_chars[i..j].iter().collect();
                if let Some(unicode_char) = self.token2unicode.get(&slice) {
                    encoded.push_str(unicode_char);
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
}
