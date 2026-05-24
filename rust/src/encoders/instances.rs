// generated with jinja2
use super::types::{Encoder, MapEncoder, TokenEncoder, EchoEncoder, AdaptiveEncoder, Transport};
use super::dictionaries;

// Echo encoder
pub const ECHO_ENCODER: EchoEncoder = EchoEncoder;

pub const RU_32_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_32::TOKEN2INT,
  int2token: &dictionaries::ru_32::INT2TOKEN,
  transport: Transport::BIN(5),
};
pub const RU_256_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_256::TOKEN2INT,
  int2token: &dictionaries::ru_256::INT2TOKEN,
  transport: Transport::BIN(8),
};
pub const EN_32_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_32::TOKEN2INT,
  int2token: &dictionaries::en_32::INT2TOKEN,
  transport: Transport::BIN(5),
};
pub const EN_128_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_128::TOKEN2INT,
  int2token: &dictionaries::en_128::INT2TOKEN,
  transport: Transport::BIN(7),
};
pub const CYR_512_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::cyr_512::TOKEN2INT,
  int2token: &dictionaries::cyr_512::INT2TOKEN,
  transport: Transport::BIN(9),
};
pub const LAT_128_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::lat_128::TOKEN2INT,
  int2token: &dictionaries::lat_128::INT2TOKEN,
  transport: Transport::BIN(7),
};
pub const LAT_1024_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::lat_1024::TOKEN2INT,
  int2token: &dictionaries::lat_1024::INT2TOKEN,
  transport: Transport::BIN(10),
};
pub const RU_WIKI_32_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_32_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_32_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_32_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_32_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_32_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_WIKI_256_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_256_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_256_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_256_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_256_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_256_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_WIKI_32_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_32_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_32_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_32_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_32_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_32_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_WIKI_128_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_128_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_128_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_128_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_128_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_128_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_CODING_32_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_coding_32_2048::TOKEN2INT,
  int2token: &dictionaries::en_coding_32_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_CODING_32_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_coding_32_2048::TOKEN2INT,
  int2token: &dictionaries::en_coding_32_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_CODING_128_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_coding_128_2048::TOKEN2INT,
  int2token: &dictionaries::en_coding_128_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_CODING_128_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_coding_128_2048::TOKEN2INT,
  int2token: &dictionaries::en_coding_128_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_32_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_32_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_32_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_32_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_32_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_32_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_256_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_256_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_256_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_256_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_256_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_256_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const CYR_MESHTASTIC_512_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::cyr_meshtastic_512_2048::TOKEN2INT,
  int2token: &dictionaries::cyr_meshtastic_512_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const CYR_MESHTASTIC_512_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::cyr_meshtastic_512_2048::TOKEN2INT,
  int2token: &dictionaries::cyr_meshtastic_512_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const LAT_MESHTASTIC_128_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::lat_meshtastic_128_2048::TOKEN2INT,
  int2token: &dictionaries::lat_meshtastic_128_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const LAT_MESHTASTIC_128_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::lat_meshtastic_128_2048::TOKEN2INT,
  int2token: &dictionaries::lat_meshtastic_128_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const LAT_MESHTASTIC_1024_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::lat_meshtastic_1024_2048::TOKEN2INT,
  int2token: &dictionaries::lat_meshtastic_1024_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const LAT_MESHTASTIC_1024_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::lat_meshtastic_1024_2048::TOKEN2INT,
  int2token: &dictionaries::lat_meshtastic_1024_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};

// Adaptive encoder
pub const ADAPTIVE_ENCODER: AdaptiveEncoder = AdaptiveEncoder {
  encoders_inventory: &[
    ('!', &ECHO_ENCODER),
    ('\"', &RU_32_BIN_MAP),
    ('#', &RU_256_BIN_MAP),
    ('$', &EN_32_BIN_MAP),
    ('%', &EN_128_BIN_MAP),
    ('&', &CYR_512_BIN_MAP),
    ('\'', &LAT_128_BIN_MAP),
    ('(', &LAT_1024_BIN_MAP),
    (')', &RU_WIKI_32_2048_UTF8_TOKEN),
    ('*', &RU_WIKI_32_2048_BIN_TOKEN),
    ('+', &RU_WIKI_256_2048_UTF8_TOKEN),
    (',', &RU_WIKI_256_2048_BIN_TOKEN),
    ('-', &EN_WIKI_32_2048_UTF8_TOKEN),
    ('.', &EN_WIKI_32_2048_BIN_TOKEN),
    ('/', &EN_WIKI_128_2048_UTF8_TOKEN),
    ('0', &EN_WIKI_128_2048_BIN_TOKEN),
    ('1', &EN_CODING_32_2048_UTF8_TOKEN),
    ('2', &EN_CODING_32_2048_BIN_TOKEN),
    ('3', &EN_CODING_128_2048_UTF8_TOKEN),
    ('4', &EN_CODING_128_2048_BIN_TOKEN),
    ('5', &RU_MESHCORETEL_32_2048_UTF8_TOKEN),
    ('6', &RU_MESHCORETEL_32_2048_BIN_TOKEN),
    ('7', &RU_MESHCORETEL_256_2048_UTF8_TOKEN),
    ('8', &RU_MESHCORETEL_256_2048_BIN_TOKEN),
    ('9', &CYR_MESHTASTIC_512_2048_UTF8_TOKEN),
    (':', &CYR_MESHTASTIC_512_2048_BIN_TOKEN),
    (';', &LAT_MESHTASTIC_128_2048_UTF8_TOKEN),
    ('<', &LAT_MESHTASTIC_128_2048_BIN_TOKEN),
    ('=', &LAT_MESHTASTIC_1024_2048_UTF8_TOKEN),
    ('>', &LAT_MESHTASTIC_1024_2048_BIN_TOKEN),
  ],
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("adaptive", &ADAPTIVE_ENCODER),
  ("ru_32_bin_map", &RU_32_BIN_MAP),
  ("ru_256_bin_map", &RU_256_BIN_MAP),
  ("en_32_bin_map", &EN_32_BIN_MAP),
  ("en_128_bin_map", &EN_128_BIN_MAP),
  ("cyr_512_bin_map", &CYR_512_BIN_MAP),
  ("lat_128_bin_map", &LAT_128_BIN_MAP),
  ("lat_1024_bin_map", &LAT_1024_BIN_MAP),
  ("ru_wiki_32_2048_utf8_token", &RU_WIKI_32_2048_UTF8_TOKEN),
  ("ru_wiki_32_2048_bin_token", &RU_WIKI_32_2048_BIN_TOKEN),
  ("ru_wiki_256_2048_utf8_token", &RU_WIKI_256_2048_UTF8_TOKEN),
  ("ru_wiki_256_2048_bin_token", &RU_WIKI_256_2048_BIN_TOKEN),
  ("en_wiki_32_2048_utf8_token", &EN_WIKI_32_2048_UTF8_TOKEN),
  ("en_wiki_32_2048_bin_token", &EN_WIKI_32_2048_BIN_TOKEN),
  ("en_wiki_128_2048_utf8_token", &EN_WIKI_128_2048_UTF8_TOKEN),
  ("en_wiki_128_2048_bin_token", &EN_WIKI_128_2048_BIN_TOKEN),
  ("en_coding_32_2048_utf8_token", &EN_CODING_32_2048_UTF8_TOKEN),
  ("en_coding_32_2048_bin_token", &EN_CODING_32_2048_BIN_TOKEN),
  ("en_coding_128_2048_utf8_token", &EN_CODING_128_2048_UTF8_TOKEN),
  ("en_coding_128_2048_bin_token", &EN_CODING_128_2048_BIN_TOKEN),
  ("ru_meshcoretel_32_2048_utf8_token", &RU_MESHCORETEL_32_2048_UTF8_TOKEN),
  ("ru_meshcoretel_32_2048_bin_token", &RU_MESHCORETEL_32_2048_BIN_TOKEN),
  ("ru_meshcoretel_256_2048_utf8_token", &RU_MESHCORETEL_256_2048_UTF8_TOKEN),
  ("ru_meshcoretel_256_2048_bin_token", &RU_MESHCORETEL_256_2048_BIN_TOKEN),
  ("cyr_meshtastic_512_2048_utf8_token", &CYR_MESHTASTIC_512_2048_UTF8_TOKEN),
  ("cyr_meshtastic_512_2048_bin_token", &CYR_MESHTASTIC_512_2048_BIN_TOKEN),
  ("lat_meshtastic_128_2048_utf8_token", &LAT_MESHTASTIC_128_2048_UTF8_TOKEN),
  ("lat_meshtastic_128_2048_bin_token", &LAT_MESHTASTIC_128_2048_BIN_TOKEN),
  ("lat_meshtastic_1024_2048_utf8_token", &LAT_MESHTASTIC_1024_2048_UTF8_TOKEN),
  ("lat_meshtastic_1024_2048_bin_token", &LAT_MESHTASTIC_1024_2048_BIN_TOKEN),
];