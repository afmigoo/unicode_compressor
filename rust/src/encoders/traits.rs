use phf::Map;

use super::errors::Error;

pub trait Encoder: Send + Sync {
    fn token2unicode(&self) -> &'static Map<&'static str, &'static str>;
    fn unicode2token(&self) -> &'static Map<&'static str, &'static str>;

    fn encode(&self, payload: &str) -> Result<String, Error> {
        let token2unicode = self.token2unicode();
        let mut encoded = String::new();

        for ch in payload.chars() {
            match token2unicode.get(&ch.to_string()) {
                Some(ch) => encoded.push_str(ch),
                None => return Err(Error::CharacterNotInAlphabet(ch.to_string())),
            }
        }
        Ok(encoded)
    }
    fn decode(&self, payload: &str) -> Result<String, Error> {
        let unicode2token = self.unicode2token();
        let mut decoded = String::new();

        for ch in payload.chars() {
            match unicode2token.get(&ch.to_string()) {
                Some(ch) => decoded.push_str(ch),
                None => return Err(Error::CharacterNotInAlphabet(ch.to_string())),
            }
        }
        Ok(decoded)
    }
}
