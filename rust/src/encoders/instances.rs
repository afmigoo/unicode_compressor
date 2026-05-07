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
pub const EN_WIKI_32_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::en_wiki_32_utf8::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_wiki_32_utf8::UNICODE2TOKEN,
  token_max_chars: dict::bpe::en_wiki_32_utf8::TOKEN_MAX_CHARS,
};
pub const EN_WIKI_ALPHA_64_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::en_wiki_alpha_64_utf8::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_wiki_alpha_64_utf8::UNICODE2TOKEN,
  token_max_chars: dict::bpe::en_wiki_alpha_64_utf8::TOKEN_MAX_CHARS,
};
pub const EN_WIKI_PUNCT_64_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::en_wiki_punct_64_utf8::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_wiki_punct_64_utf8::UNICODE2TOKEN,
  token_max_chars: dict::bpe::en_wiki_punct_64_utf8::TOKEN_MAX_CHARS,
};
pub const RU_WIKI_32_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::ru_wiki_32_utf8::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_wiki_32_utf8::UNICODE2TOKEN,
  token_max_chars: dict::bpe::ru_wiki_32_utf8::TOKEN_MAX_CHARS,
};
pub const RU_WIKI_ALPHA_64_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::ru_wiki_alpha_64_utf8::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_wiki_alpha_64_utf8::UNICODE2TOKEN,
  token_max_chars: dict::bpe::ru_wiki_alpha_64_utf8::TOKEN_MAX_CHARS,
};
pub const RU_WIKI_PUNCT_64_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::ru_wiki_punct_64_utf8::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_wiki_punct_64_utf8::UNICODE2TOKEN,
  token_max_chars: dict::bpe::ru_wiki_punct_64_utf8::TOKEN_MAX_CHARS,
};

// available codes:
// ['!', '"', '#', '$', '%', '&', "'", '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~']
pub const ADAPTIVE_ENCODER: AdaptiveEncoder = AdaptiveEncoder {
  encoders_inventory: &[
    ('!', &DUMMY_ENCODER),
    ('"', &CODING_ENCODER),
    ('#', &MESHCORETEL_RU_ENCODER),
    ('$', &EN_WIKI_32_UTF8_ENCODER),
    ('%', &EN_WIKI_ALPHA_64_UTF8_ENCODER),
    ('&', &EN_WIKI_PUNCT_64_UTF8_ENCODER),
    ('\'', &RU_WIKI_32_UTF8_ENCODER),
    ('(', &RU_WIKI_ALPHA_64_UTF8_ENCODER),
    (')', &RU_WIKI_PUNCT_64_UTF8_ENCODER),
  ]
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("adaptive", &ADAPTIVE_ENCODER),
  ("bpe_dummy", &DUMMY_ENCODER),
  ("token_coding", &CODING_ENCODER),
  ("token_meshcoretel_ru", &MESHCORETEL_RU_ENCODER),
  ("token_en_wiki_32_utf8", &EN_WIKI_32_UTF8_ENCODER),
  ("token_en_wiki_alpha_64_utf8", &EN_WIKI_ALPHA_64_UTF8_ENCODER),
  ("token_en_wiki_punct_64_utf8", &EN_WIKI_PUNCT_64_UTF8_ENCODER),
  ("token_ru_wiki_32_utf8", &RU_WIKI_32_UTF8_ENCODER),
  ("token_ru_wiki_alpha_64_utf8", &RU_WIKI_ALPHA_64_UTF8_ENCODER),
  ("token_ru_wiki_punct_64_utf8", &RU_WIKI_PUNCT_64_UTF8_ENCODER),
];
