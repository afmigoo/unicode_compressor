use phf::Map;

use super::traits::Encoder;

pub struct MapEncoder {
    pub token2unicode: &'static Map<&'static str, &'static str>,
    pub unicode2token: &'static Map<&'static str, &'static str>,
}

impl Encoder for MapEncoder {
    fn token2unicode(&self) -> &'static Map<&'static str, &'static str> {
        &self.token2unicode
    }
    fn unicode2token(&self) -> &'static Map<&'static str, &'static str> {
        &self.unicode2token
    }
}
