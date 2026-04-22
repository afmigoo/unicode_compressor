use std::cmp::min;

use super::errors::Error;
use super::traits::Encoder;

pub struct TokenEncoder {
    pub token2unicode: &'static phf::Map<&'static str, &'static str>,
    pub unicode2token: &'static phf::Map<&'static str, &'static str>,
    pub token_max_chars: u8,
}

impl Encoder for TokenEncoder {
    fn token2unicode(&self) -> &'static phf::Map<&'static str, &'static str> {
        &self.token2unicode
    }
    fn unicode2token(&self) -> &'static phf::Map<&'static str, &'static str> {
        &self.unicode2token
    }

    fn encode(&self, payload: &str) -> Result<String, Error> {
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
