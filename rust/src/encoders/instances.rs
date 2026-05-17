// generated with jinja2
use super::types::{Encoder, MapEncoder, TokenEncoder, EchoEncoder, AdaptiveEncoder, Transport};
use super::dictionaries;

// Echo encoder
pub const ECHO_ENCODER: EchoEncoder = EchoEncoder;

pub const EN_128_BIN_MAP: MapEncoder = MapEncoder {
  token2int: dictionaries::en_128::TOKEN2INT,
  int2token: dictionaries::en_128::INT2TOKEN,
  transport: Transport::BIN(7),
};
pub const EN_32_BIN_MAP: MapEncoder = MapEncoder {
  token2int: dictionaries::en_32::TOKEN2INT,
  int2token: dictionaries::en_32::INT2TOKEN,
  transport: Transport::BIN(5),
};
pub const EN_WIKI_128_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::en_wiki_128::TOKEN2INT,
  int2token: dictionaries::en_wiki_128::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_WIKI_128_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::en_wiki_128::TOKEN2INT,
  int2token: dictionaries::en_wiki_128::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_32_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::en_wiki_32::TOKEN2INT,
  int2token: dictionaries::en_wiki_32::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_WIKI_32_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::en_wiki_32::TOKEN2INT,
  int2token: dictionaries::en_wiki_32::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_256_BIN_MAP: MapEncoder = MapEncoder {
  token2int: dictionaries::ru_256::TOKEN2INT,
  int2token: dictionaries::ru_256::INT2TOKEN,
  transport: Transport::BIN(8),
};
pub const RU_32_BIN_MAP: MapEncoder = MapEncoder {
  token2int: dictionaries::ru_32::TOKEN2INT,
  int2token: dictionaries::ru_32::INT2TOKEN,
  transport: Transport::BIN(5),
};
pub const RU_MESHCORETEL_256_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::ru_meshcoretel_256::TOKEN2INT,
  int2token: dictionaries::ru_meshcoretel_256::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_256_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::ru_meshcoretel_256::TOKEN2INT,
  int2token: dictionaries::ru_meshcoretel_256::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_32_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::ru_meshcoretel_32::TOKEN2INT,
  int2token: dictionaries::ru_meshcoretel_32::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_32_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::ru_meshcoretel_32::TOKEN2INT,
  int2token: dictionaries::ru_meshcoretel_32::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_256_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::ru_wiki_256::TOKEN2INT,
  int2token: dictionaries::ru_wiki_256::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_WIKI_256_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::ru_wiki_256::TOKEN2INT,
  int2token: dictionaries::ru_wiki_256::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_32_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::ru_wiki_32::TOKEN2INT,
  int2token: dictionaries::ru_wiki_32::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_WIKI_32_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: dictionaries::ru_wiki_32::TOKEN2INT,
  int2token: dictionaries::ru_wiki_32::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};

// Adaptive encoder
pub const ADAPTIVE_ENCODER: AdaptiveEncoder = AdaptiveEncoder {
  encoders_inventory: &[
    ('!', &ECHO_ENCODER),
    ('\"', &EN_128_BIN_MAP),
    ('#', &EN_32_BIN_MAP),
    ('$', &EN_WIKI_128_BIN_TOKEN),
    ('%', &EN_WIKI_128_UTF8_TOKEN),
    ('&', &EN_WIKI_32_BIN_TOKEN),
    ('\'', &EN_WIKI_32_UTF8_TOKEN),
    ('(', &RU_256_BIN_MAP),
    (')', &RU_32_BIN_MAP),
    ('*', &RU_MESHCORETEL_256_BIN_TOKEN),
    ('+', &RU_MESHCORETEL_256_UTF8_TOKEN),
    (',', &RU_MESHCORETEL_32_BIN_TOKEN),
    ('-', &RU_MESHCORETEL_32_UTF8_TOKEN),
    ('.', &RU_WIKI_256_BIN_TOKEN),
    ('/', &RU_WIKI_256_UTF8_TOKEN),
    ('0', &RU_WIKI_32_BIN_TOKEN),
    ('1', &RU_WIKI_32_UTF8_TOKEN),
  ],
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("adaptive", &ADAPTIVE_ENCODER),
  ("en_128_bin_map", &EN_128_BIN_MAP),
  ("en_32_bin_map", &EN_32_BIN_MAP),
  ("en_wiki_128_bin_token", &EN_WIKI_128_BIN_TOKEN),
  ("en_wiki_128_utf8_token", &EN_WIKI_128_UTF8_TOKEN),
  ("en_wiki_32_bin_token", &EN_WIKI_32_BIN_TOKEN),
  ("en_wiki_32_utf8_token", &EN_WIKI_32_UTF8_TOKEN),
  ("ru_256_bin_map", &RU_256_BIN_MAP),
  ("ru_32_bin_map", &RU_32_BIN_MAP),
  ("ru_meshcoretel_256_bin_token", &RU_MESHCORETEL_256_BIN_TOKEN),
  ("ru_meshcoretel_256_utf8_token", &RU_MESHCORETEL_256_UTF8_TOKEN),
  ("ru_meshcoretel_32_bin_token", &RU_MESHCORETEL_32_BIN_TOKEN),
  ("ru_meshcoretel_32_utf8_token", &RU_MESHCORETEL_32_UTF8_TOKEN),
  ("ru_wiki_256_bin_token", &RU_WIKI_256_BIN_TOKEN),
  ("ru_wiki_256_utf8_token", &RU_WIKI_256_UTF8_TOKEN),
  ("ru_wiki_32_bin_token", &RU_WIKI_32_BIN_TOKEN),
  ("ru_wiki_32_utf8_token", &RU_WIKI_32_UTF8_TOKEN),
];