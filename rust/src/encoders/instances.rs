use super::types::{Encoder, MapEncoder, TokenEncoder, AdaptiveEncoder};
use crate::{dictionaries as dict};

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

pub const EN_32_UTF8_MAP_ENCODER: MapEncoder = MapEncoder {
  token2unicode: &dict::bpe::en_32_utf8_map::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_32_utf8_map::UNICODE2TOKEN,
};
pub const EN_ALPHA_64_UTF8_MAP_ENCODER: MapEncoder = MapEncoder {
  token2unicode: &dict::bpe::en_alpha_64_utf8_map::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_alpha_64_utf8_map::UNICODE2TOKEN,
};
pub const EN_PUNCT_64_UTF8_MAP_ENCODER: MapEncoder = MapEncoder {
  token2unicode: &dict::bpe::en_punct_64_utf8_map::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_punct_64_utf8_map::UNICODE2TOKEN,
};
pub const RU_32_UTF8_MAP_ENCODER: MapEncoder = MapEncoder {
  token2unicode: &dict::bpe::ru_32_utf8_map::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_32_utf8_map::UNICODE2TOKEN,
};
pub const RU_ALPHA_64_UTF8_MAP_ENCODER: MapEncoder = MapEncoder {
  token2unicode: &dict::bpe::ru_alpha_64_utf8_map::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_alpha_64_utf8_map::UNICODE2TOKEN,
};
pub const RU_PUNCT_64_UTF8_MAP_ENCODER: MapEncoder = MapEncoder {
  token2unicode: &dict::bpe::ru_punct_64_utf8_map::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_punct_64_utf8_map::UNICODE2TOKEN,
};

pub const EN_WIKI_32_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::en_wiki_32_utf8_tkn::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_wiki_32_utf8_tkn::UNICODE2TOKEN,
  token_max_chars: dict::bpe::en_wiki_32_utf8_tkn::TOKEN_MAX_CHARS,
};
pub const EN_WIKI_ALPHA_64_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::en_wiki_alpha_64_utf8_tkn::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_wiki_alpha_64_utf8_tkn::UNICODE2TOKEN,
  token_max_chars: dict::bpe::en_wiki_alpha_64_utf8_tkn::TOKEN_MAX_CHARS,
};
pub const EN_WIKI_PUNCT_64_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::en_wiki_punct_64_utf8_tkn::TOKEN2UNICODE,
  unicode2token: &dict::bpe::en_wiki_punct_64_utf8_tkn::UNICODE2TOKEN,
  token_max_chars: dict::bpe::en_wiki_punct_64_utf8_tkn::TOKEN_MAX_CHARS,
};
pub const RU_WIKI_32_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::ru_wiki_32_utf8_tkn::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_wiki_32_utf8_tkn::UNICODE2TOKEN,
  token_max_chars: dict::bpe::ru_wiki_32_utf8_tkn::TOKEN_MAX_CHARS,
};
pub const RU_WIKI_ALPHA_64_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::ru_wiki_alpha_64_utf8_tkn::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_wiki_alpha_64_utf8_tkn::UNICODE2TOKEN,
  token_max_chars: dict::bpe::ru_wiki_alpha_64_utf8_tkn::TOKEN_MAX_CHARS,
};
pub const RU_WIKI_PUNCT_64_UTF8_ENCODER: TokenEncoder = TokenEncoder {
  token2unicode: &dict::bpe::ru_wiki_punct_64_utf8_tkn::TOKEN2UNICODE,
  unicode2token: &dict::bpe::ru_wiki_punct_64_utf8_tkn::UNICODE2TOKEN,
  token_max_chars: dict::bpe::ru_wiki_punct_64_utf8_tkn::TOKEN_MAX_CHARS,
};

// available codes:
// ['!', '"', '#', '$', '%', '&', "'", '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~']
pub const ADAPTIVE_ENCODER: AdaptiveEncoder = AdaptiveEncoder {
  encoders_inventory: &[
    ('!', &CODING_ENCODER),

    ('"', &MESHCORETEL_RU_ENCODER),
    ('#', &EN_32_UTF8_MAP_ENCODER),
    ('$', &EN_ALPHA_64_UTF8_MAP_ENCODER),
    ('%', &EN_PUNCT_64_UTF8_MAP_ENCODER),
    ('&', &RU_32_UTF8_MAP_ENCODER),
    ('\'', &RU_ALPHA_64_UTF8_MAP_ENCODER),
    ('(', &RU_PUNCT_64_UTF8_MAP_ENCODER),

    (')', &EN_WIKI_32_UTF8_ENCODER),
    ('*', &EN_WIKI_ALPHA_64_UTF8_ENCODER),
    ('+', &EN_WIKI_PUNCT_64_UTF8_ENCODER),
    (',', &RU_WIKI_32_UTF8_ENCODER),
    ('-', &RU_WIKI_ALPHA_64_UTF8_ENCODER),
    ('.', &RU_WIKI_PUNCT_64_UTF8_ENCODER),
  ]
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("adaptive", &ADAPTIVE_ENCODER),
  ("token_coding", &CODING_ENCODER),

  ("en_32_utf8_map", &EN_32_UTF8_MAP_ENCODER),
  ("en_alpha_64_utf8_map", &EN_ALPHA_64_UTF8_MAP_ENCODER),
  ("en_punct_64_utf8_map", &EN_PUNCT_64_UTF8_MAP_ENCODER),
  ("ru_32_utf8_map", &RU_32_UTF8_MAP_ENCODER),
  ("ru_alpha_64_utf8_map", &RU_ALPHA_64_UTF8_MAP_ENCODER),
  ("ru_punct_64_utf8_map", &RU_PUNCT_64_UTF8_MAP_ENCODER),

  ("meshcoretel_ru_token", &MESHCORETEL_RU_ENCODER),
  ("en_wiki_32_utf8_token", &EN_WIKI_32_UTF8_ENCODER),
  ("en_wiki_alpha_64_utf8_token", &EN_WIKI_ALPHA_64_UTF8_ENCODER),
  ("en_wiki_punct_64_utf8_token", &EN_WIKI_PUNCT_64_UTF8_ENCODER),
  ("ru_wiki_32_utf8_token", &RU_WIKI_32_UTF8_ENCODER),
  ("ru_wiki_alpha_64_utf8_token", &RU_WIKI_ALPHA_64_UTF8_ENCODER),
  ("ru_wiki_punct_64_utf8_token", &RU_WIKI_PUNCT_64_UTF8_ENCODER),
];
