use super::types::{Encoder, MapEncoder, TokenEncoder, AdaptiveEncoder};
use crate::{dictionaries as dict};

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

pub const ADAPTIVE_ENCODER: AdaptiveEncoder = AdaptiveEncoder {
  encoders_inventory: &[
    ('0', &DUMMY_ENCODER),
    ('1', &CODING_ENCODER),
    ('2', &MESHCORETEL_RU_ENCODER),
    ('3', &WIKI_EN_ENCODER),
    ('4', &WIKI_RU_ENCODER),
    ('5', &WIKI_ENCODER),
  ]
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("adaptive", &ADAPTIVE_ENCODER),
  ("bpe_dummy", &DUMMY_ENCODER),
  ("token_coding", &CODING_ENCODER),
  ("token_meshcoretel_ru", &MESHCORETEL_RU_ENCODER),
  ("token_wiki_en", &WIKI_EN_ENCODER),
  ("token_wiki_ru", &WIKI_RU_ENCODER),
  ("token_wiki", &WIKI_ENCODER),
];
