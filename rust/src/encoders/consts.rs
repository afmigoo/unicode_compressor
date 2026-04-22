use super::map_encoder::MapEncoder;
use super::token_encoder::TokenEncoder;
use super::traits::Encoder;
use crate::dictionaries as dict;

pub const ENCODER_NAMES: &[&str] = &[
  "bpe_dummy",
  "bpe_coding",
  "bpe_meshcoretel_ru",
  "bpe_wiki_en",
  "bpe_wiki_ru",
  "bpe_wiki",
];

pub const DUMMY_ENCODER: MapEncoder = MapEncoder {
  token2unicode: &dict::bpe::dummy::TOKEN2UNICODE,
  unicode2token: &dict::bpe::dummy::UNICODE2TOKEN,
};
pub const CODING_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::coding::TOKEN2UNICODE,
  unicode2token: &dict::bpe::coding::UNICODE2TOKEN,
  token_max_chars: dict::bpe::coding::TOKEN_MAX_CHARS,
};
pub const MESHCORETEL_RU_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::meshcoretel_ru::TOKEN2UNICODE,
  unicode2token: &dict::bpe::meshcoretel_ru::UNICODE2TOKEN,
  token_max_chars: dict::bpe::meshcoretel_ru::TOKEN_MAX_CHARS,
};
pub const WIKI_EN_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::wiki_en::TOKEN2UNICODE,
  unicode2token: &dict::bpe::wiki_en::UNICODE2TOKEN,
  token_max_chars: dict::bpe::wiki_en::TOKEN_MAX_CHARS,
};
pub const WIKI_RU_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::wiki_ru::TOKEN2UNICODE,
  unicode2token: &dict::bpe::wiki_ru::UNICODE2TOKEN,
  token_max_chars: dict::bpe::wiki_ru::TOKEN_MAX_CHARS,
};
pub const WIKI_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::wiki::TOKEN2UNICODE,
  unicode2token: &dict::bpe::wiki::UNICODE2TOKEN,
  token_max_chars: dict::bpe::wiki::TOKEN_MAX_CHARS,
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("bpe_dummy", &DUMMY_ENCODER),
  ("bpe_coding", &CODING_ENCODER),
  ("bpe_meshcoretel_ru", &MESHCORETEL_RU_ENCODER),
  ("bpe_wiki_en", &WIKI_EN_ENCODER),
  ("bpe_wiki_ru", &WIKI_RU_ENCODER),
  ("bpe_wiki", &WIKI_ENCODER),
];
