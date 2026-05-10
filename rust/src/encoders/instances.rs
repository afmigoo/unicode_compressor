// generated with jinja2
use super::types::{Encoder, MapEncoder, TokenEncoder, EchoEncoder, AdaptiveEncoder, Transport};
use super::dictionaries;

// Echo encoder
pub const ECHO_ENCODER: EchoEncoder = EchoEncoder;

pub const RU_32_UTF8_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_32::TOKEN2INT,
  int2token: &dictionaries::ru_32::INT2TOKEN,
  transport: Transport::UTF8,
};
pub const RU_32_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_32::TOKEN2INT,
  int2token: &dictionaries::ru_32::INT2TOKEN,
  transport: Transport::BIN(5),
};
pub const RU_PUNCT_64_UTF8_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_punct_64::TOKEN2INT,
  int2token: &dictionaries::ru_punct_64::INT2TOKEN,
  transport: Transport::UTF8,
};
pub const RU_PUNCT_64_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_punct_64::TOKEN2INT,
  int2token: &dictionaries::ru_punct_64::INT2TOKEN,
  transport: Transport::BIN(6),
};
pub const RU_ALPHA_64_UTF8_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_alpha_64::TOKEN2INT,
  int2token: &dictionaries::ru_alpha_64::INT2TOKEN,
  transport: Transport::UTF8,
};
pub const RU_ALPHA_64_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_alpha_64::TOKEN2INT,
  int2token: &dictionaries::ru_alpha_64::INT2TOKEN,
  transport: Transport::BIN(6),
};
pub const RU_128_UTF8_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_128::TOKEN2INT,
  int2token: &dictionaries::ru_128::INT2TOKEN,
  transport: Transport::UTF8,
};
pub const RU_128_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::ru_128::TOKEN2INT,
  int2token: &dictionaries::ru_128::INT2TOKEN,
  transport: Transport::BIN(7),
};
pub const EN_32_UTF8_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_32::TOKEN2INT,
  int2token: &dictionaries::en_32::INT2TOKEN,
  transport: Transport::UTF8,
};
pub const EN_32_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_32::TOKEN2INT,
  int2token: &dictionaries::en_32::INT2TOKEN,
  transport: Transport::BIN(5),
};
pub const EN_PUNCT_64_UTF8_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_punct_64::TOKEN2INT,
  int2token: &dictionaries::en_punct_64::INT2TOKEN,
  transport: Transport::UTF8,
};
pub const EN_PUNCT_64_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_punct_64::TOKEN2INT,
  int2token: &dictionaries::en_punct_64::INT2TOKEN,
  transport: Transport::BIN(6),
};
pub const EN_ALPHA_64_UTF8_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_alpha_64::TOKEN2INT,
  int2token: &dictionaries::en_alpha_64::INT2TOKEN,
  transport: Transport::UTF8,
};
pub const EN_ALPHA_64_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_alpha_64::TOKEN2INT,
  int2token: &dictionaries::en_alpha_64::INT2TOKEN,
  transport: Transport::BIN(6),
};
pub const EN_128_UTF8_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_128::TOKEN2INT,
  int2token: &dictionaries::en_128::INT2TOKEN,
  transport: Transport::UTF8,
};
pub const EN_128_BIN_MAP: MapEncoder = MapEncoder {
  token2int: &dictionaries::en_128::TOKEN2INT,
  int2token: &dictionaries::en_128::INT2TOKEN,
  transport: Transport::BIN(7),
};
pub const RU_WIKI_ALL_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_all_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_all_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_ALL_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_all_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_all_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_WIKI_ALL_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_all_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_all_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_ALL_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_all_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_all_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_ALL_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_all_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_all_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_ALL_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_all_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_all_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_WIKI_32_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_32_64::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_32_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_32_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_32_64::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_32_64::INT2TOKEN,
  transport: Transport::BIN(6),
  token_max_chars: 16,
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
pub const RU_MESHCORETEL_32_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_32_64::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_32_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_32_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_32_64::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_32_64::INT2TOKEN,
  transport: Transport::BIN(6),
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
pub const RU_WIKI_PUNCT_64_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_punct_64_64::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_punct_64_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_PUNCT_64_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_punct_64_64::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_punct_64_64::INT2TOKEN,
  transport: Transport::BIN(6),
  token_max_chars: 16,
};
pub const RU_WIKI_PUNCT_64_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_punct_64_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_punct_64_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_PUNCT_64_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_punct_64_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_punct_64_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_PUNCT_64_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_punct_64_64::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_punct_64_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_PUNCT_64_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_punct_64_64::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_punct_64_64::INT2TOKEN,
  transport: Transport::BIN(6),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_PUNCT_64_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_punct_64_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_punct_64_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_PUNCT_64_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_punct_64_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_punct_64_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_WIKI_ALPHA_64_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_alpha_64_64::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_alpha_64_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_ALPHA_64_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_alpha_64_64::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_alpha_64_64::INT2TOKEN,
  transport: Transport::BIN(6),
  token_max_chars: 16,
};
pub const RU_WIKI_ALPHA_64_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_alpha_64_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_alpha_64_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_ALPHA_64_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_alpha_64_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_alpha_64_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_ALPHA_64_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_alpha_64_64::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_alpha_64_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_ALPHA_64_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_alpha_64_64::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_alpha_64_64::INT2TOKEN,
  transport: Transport::BIN(6),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_ALPHA_64_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_alpha_64_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_alpha_64_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_ALPHA_64_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_alpha_64_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_alpha_64_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_WIKI_128_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_128_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_128_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_WIKI_128_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_wiki_128_2048::TOKEN2INT,
  int2token: &dictionaries::ru_wiki_128_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_128_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_128_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_128_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const RU_MESHCORETEL_128_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::ru_meshcoretel_128_2048::TOKEN2INT,
  int2token: &dictionaries::ru_meshcoretel_128_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_WIKI_32_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_32_64::TOKEN2INT,
  int2token: &dictionaries::en_wiki_32_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_32_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_32_64::TOKEN2INT,
  int2token: &dictionaries::en_wiki_32_64::INT2TOKEN,
  transport: Transport::BIN(6),
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
pub const EN_WIKI_PUNCT_64_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_punct_64_64::TOKEN2INT,
  int2token: &dictionaries::en_wiki_punct_64_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_PUNCT_64_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_punct_64_64::TOKEN2INT,
  int2token: &dictionaries::en_wiki_punct_64_64::INT2TOKEN,
  transport: Transport::BIN(6),
  token_max_chars: 16,
};
pub const EN_WIKI_PUNCT_64_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_punct_64_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_punct_64_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_PUNCT_64_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_punct_64_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_punct_64_2048::INT2TOKEN,
  transport: Transport::BIN(11),
  token_max_chars: 16,
};
pub const EN_WIKI_ALPHA_64_64_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_alpha_64_64::TOKEN2INT,
  int2token: &dictionaries::en_wiki_alpha_64_64::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_ALPHA_64_64_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_alpha_64_64::TOKEN2INT,
  int2token: &dictionaries::en_wiki_alpha_64_64::INT2TOKEN,
  transport: Transport::BIN(6),
  token_max_chars: 16,
};
pub const EN_WIKI_ALPHA_64_2048_UTF8_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_alpha_64_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_alpha_64_2048::INT2TOKEN,
  transport: Transport::UTF8,
  token_max_chars: 16,
};
pub const EN_WIKI_ALPHA_64_2048_BIN_TOKEN: TokenEncoder = TokenEncoder {
  token2int: &dictionaries::en_wiki_alpha_64_2048::TOKEN2INT,
  int2token: &dictionaries::en_wiki_alpha_64_2048::INT2TOKEN,
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

// Adaptive encoder
pub const ADAPTIVE_ENCODER: AdaptiveEncoder = AdaptiveEncoder {
  encoders_inventory: &[
    ('!', &ECHO_ENCODER),
    ('\"', &RU_32_UTF8_MAP),
    ('#', &RU_32_BIN_MAP),
    ('$', &RU_PUNCT_64_UTF8_MAP),
    ('%', &RU_PUNCT_64_BIN_MAP),
    ('&', &RU_ALPHA_64_UTF8_MAP),
    ('\'', &RU_ALPHA_64_BIN_MAP),
    ('(', &RU_128_UTF8_MAP),
    (')', &RU_128_BIN_MAP),
    ('*', &EN_32_UTF8_MAP),
    ('+', &EN_32_BIN_MAP),
    (',', &EN_PUNCT_64_UTF8_MAP),
    ('-', &EN_PUNCT_64_BIN_MAP),
    ('.', &EN_ALPHA_64_UTF8_MAP),
    ('/', &EN_ALPHA_64_BIN_MAP),
    ('0', &EN_128_UTF8_MAP),
    ('1', &EN_128_BIN_MAP),
    ('2', &RU_WIKI_ALL_2048_UTF8_TOKEN),
    ('3', &RU_WIKI_ALL_2048_BIN_TOKEN),
    ('4', &EN_WIKI_ALL_2048_UTF8_TOKEN),
    ('5', &EN_WIKI_ALL_2048_BIN_TOKEN),
    ('6', &RU_MESHCORETEL_ALL_2048_UTF8_TOKEN),
    ('7', &RU_MESHCORETEL_ALL_2048_BIN_TOKEN),
    ('8', &RU_WIKI_32_64_UTF8_TOKEN),
    ('9', &RU_WIKI_32_64_BIN_TOKEN),
    (':', &RU_WIKI_32_2048_UTF8_TOKEN),
    (';', &RU_WIKI_32_2048_BIN_TOKEN),
    ('<', &RU_MESHCORETEL_32_64_UTF8_TOKEN),
    ('=', &RU_MESHCORETEL_32_64_BIN_TOKEN),
    ('>', &RU_MESHCORETEL_32_2048_UTF8_TOKEN),
    ('?', &RU_MESHCORETEL_32_2048_BIN_TOKEN),
    ('@', &RU_WIKI_PUNCT_64_64_UTF8_TOKEN),
    ('A', &RU_WIKI_PUNCT_64_64_BIN_TOKEN),
    ('B', &RU_WIKI_PUNCT_64_2048_UTF8_TOKEN),
    ('C', &RU_WIKI_PUNCT_64_2048_BIN_TOKEN),
    ('D', &RU_MESHCORETEL_PUNCT_64_64_UTF8_TOKEN),
    ('E', &RU_MESHCORETEL_PUNCT_64_64_BIN_TOKEN),
    ('F', &RU_MESHCORETEL_PUNCT_64_2048_UTF8_TOKEN),
    ('G', &RU_MESHCORETEL_PUNCT_64_2048_BIN_TOKEN),
    ('H', &RU_WIKI_ALPHA_64_64_UTF8_TOKEN),
    ('I', &RU_WIKI_ALPHA_64_64_BIN_TOKEN),
    ('J', &RU_WIKI_ALPHA_64_2048_UTF8_TOKEN),
    ('K', &RU_WIKI_ALPHA_64_2048_BIN_TOKEN),
    ('L', &RU_MESHCORETEL_ALPHA_64_64_UTF8_TOKEN),
    ('M', &RU_MESHCORETEL_ALPHA_64_64_BIN_TOKEN),
    ('N', &RU_MESHCORETEL_ALPHA_64_2048_UTF8_TOKEN),
    ('O', &RU_MESHCORETEL_ALPHA_64_2048_BIN_TOKEN),
    ('P', &RU_WIKI_128_2048_UTF8_TOKEN),
    ('Q', &RU_WIKI_128_2048_BIN_TOKEN),
    ('R', &RU_MESHCORETEL_128_2048_UTF8_TOKEN),
    ('S', &RU_MESHCORETEL_128_2048_BIN_TOKEN),
    ('T', &EN_WIKI_32_64_UTF8_TOKEN),
    ('U', &EN_WIKI_32_64_BIN_TOKEN),
    ('V', &EN_WIKI_32_2048_UTF8_TOKEN),
    ('W', &EN_WIKI_32_2048_BIN_TOKEN),
    ('X', &EN_WIKI_PUNCT_64_64_UTF8_TOKEN),
    ('Y', &EN_WIKI_PUNCT_64_64_BIN_TOKEN),
    ('Z', &EN_WIKI_PUNCT_64_2048_UTF8_TOKEN),
    ('[', &EN_WIKI_PUNCT_64_2048_BIN_TOKEN),
    ('\\', &EN_WIKI_ALPHA_64_64_UTF8_TOKEN),
    (']', &EN_WIKI_ALPHA_64_64_BIN_TOKEN),
    ('^', &EN_WIKI_ALPHA_64_2048_UTF8_TOKEN),
    ('_', &EN_WIKI_ALPHA_64_2048_BIN_TOKEN),
    ('`', &EN_WIKI_128_2048_UTF8_TOKEN),
    ('a', &EN_WIKI_128_2048_BIN_TOKEN),
  ],
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("adaptive", &ADAPTIVE_ENCODER),
  ("ru_32_utf8_map", &RU_32_UTF8_MAP),
  ("ru_32_bin_map", &RU_32_BIN_MAP),
  ("ru_punct_64_utf8_map", &RU_PUNCT_64_UTF8_MAP),
  ("ru_punct_64_bin_map", &RU_PUNCT_64_BIN_MAP),
  ("ru_alpha_64_utf8_map", &RU_ALPHA_64_UTF8_MAP),
  ("ru_alpha_64_bin_map", &RU_ALPHA_64_BIN_MAP),
  ("ru_128_utf8_map", &RU_128_UTF8_MAP),
  ("ru_128_bin_map", &RU_128_BIN_MAP),
  ("en_32_utf8_map", &EN_32_UTF8_MAP),
  ("en_32_bin_map", &EN_32_BIN_MAP),
  ("en_punct_64_utf8_map", &EN_PUNCT_64_UTF8_MAP),
  ("en_punct_64_bin_map", &EN_PUNCT_64_BIN_MAP),
  ("en_alpha_64_utf8_map", &EN_ALPHA_64_UTF8_MAP),
  ("en_alpha_64_bin_map", &EN_ALPHA_64_BIN_MAP),
  ("en_128_utf8_map", &EN_128_UTF8_MAP),
  ("en_128_bin_map", &EN_128_BIN_MAP),
  ("ru_wiki_all_2048_utf8_token", &RU_WIKI_ALL_2048_UTF8_TOKEN),
  ("ru_wiki_all_2048_bin_token", &RU_WIKI_ALL_2048_BIN_TOKEN),
  ("en_wiki_all_2048_utf8_token", &EN_WIKI_ALL_2048_UTF8_TOKEN),
  ("en_wiki_all_2048_bin_token", &EN_WIKI_ALL_2048_BIN_TOKEN),
  ("ru_meshcoretel_all_2048_utf8_token", &RU_MESHCORETEL_ALL_2048_UTF8_TOKEN),
  ("ru_meshcoretel_all_2048_bin_token", &RU_MESHCORETEL_ALL_2048_BIN_TOKEN),
  ("ru_wiki_32_64_utf8_token", &RU_WIKI_32_64_UTF8_TOKEN),
  ("ru_wiki_32_64_bin_token", &RU_WIKI_32_64_BIN_TOKEN),
  ("ru_wiki_32_2048_utf8_token", &RU_WIKI_32_2048_UTF8_TOKEN),
  ("ru_wiki_32_2048_bin_token", &RU_WIKI_32_2048_BIN_TOKEN),
  ("ru_meshcoretel_32_64_utf8_token", &RU_MESHCORETEL_32_64_UTF8_TOKEN),
  ("ru_meshcoretel_32_64_bin_token", &RU_MESHCORETEL_32_64_BIN_TOKEN),
  ("ru_meshcoretel_32_2048_utf8_token", &RU_MESHCORETEL_32_2048_UTF8_TOKEN),
  ("ru_meshcoretel_32_2048_bin_token", &RU_MESHCORETEL_32_2048_BIN_TOKEN),
  ("ru_wiki_punct_64_64_utf8_token", &RU_WIKI_PUNCT_64_64_UTF8_TOKEN),
  ("ru_wiki_punct_64_64_bin_token", &RU_WIKI_PUNCT_64_64_BIN_TOKEN),
  ("ru_wiki_punct_64_2048_utf8_token", &RU_WIKI_PUNCT_64_2048_UTF8_TOKEN),
  ("ru_wiki_punct_64_2048_bin_token", &RU_WIKI_PUNCT_64_2048_BIN_TOKEN),
  ("ru_meshcoretel_punct_64_64_utf8_token", &RU_MESHCORETEL_PUNCT_64_64_UTF8_TOKEN),
  ("ru_meshcoretel_punct_64_64_bin_token", &RU_MESHCORETEL_PUNCT_64_64_BIN_TOKEN),
  ("ru_meshcoretel_punct_64_2048_utf8_token", &RU_MESHCORETEL_PUNCT_64_2048_UTF8_TOKEN),
  ("ru_meshcoretel_punct_64_2048_bin_token", &RU_MESHCORETEL_PUNCT_64_2048_BIN_TOKEN),
  ("ru_wiki_alpha_64_64_utf8_token", &RU_WIKI_ALPHA_64_64_UTF8_TOKEN),
  ("ru_wiki_alpha_64_64_bin_token", &RU_WIKI_ALPHA_64_64_BIN_TOKEN),
  ("ru_wiki_alpha_64_2048_utf8_token", &RU_WIKI_ALPHA_64_2048_UTF8_TOKEN),
  ("ru_wiki_alpha_64_2048_bin_token", &RU_WIKI_ALPHA_64_2048_BIN_TOKEN),
  ("ru_meshcoretel_alpha_64_64_utf8_token", &RU_MESHCORETEL_ALPHA_64_64_UTF8_TOKEN),
  ("ru_meshcoretel_alpha_64_64_bin_token", &RU_MESHCORETEL_ALPHA_64_64_BIN_TOKEN),
  ("ru_meshcoretel_alpha_64_2048_utf8_token", &RU_MESHCORETEL_ALPHA_64_2048_UTF8_TOKEN),
  ("ru_meshcoretel_alpha_64_2048_bin_token", &RU_MESHCORETEL_ALPHA_64_2048_BIN_TOKEN),
  ("ru_wiki_128_2048_utf8_token", &RU_WIKI_128_2048_UTF8_TOKEN),
  ("ru_wiki_128_2048_bin_token", &RU_WIKI_128_2048_BIN_TOKEN),
  ("ru_meshcoretel_128_2048_utf8_token", &RU_MESHCORETEL_128_2048_UTF8_TOKEN),
  ("ru_meshcoretel_128_2048_bin_token", &RU_MESHCORETEL_128_2048_BIN_TOKEN),
  ("en_wiki_32_64_utf8_token", &EN_WIKI_32_64_UTF8_TOKEN),
  ("en_wiki_32_64_bin_token", &EN_WIKI_32_64_BIN_TOKEN),
  ("en_wiki_32_2048_utf8_token", &EN_WIKI_32_2048_UTF8_TOKEN),
  ("en_wiki_32_2048_bin_token", &EN_WIKI_32_2048_BIN_TOKEN),
  ("en_wiki_punct_64_64_utf8_token", &EN_WIKI_PUNCT_64_64_UTF8_TOKEN),
  ("en_wiki_punct_64_64_bin_token", &EN_WIKI_PUNCT_64_64_BIN_TOKEN),
  ("en_wiki_punct_64_2048_utf8_token", &EN_WIKI_PUNCT_64_2048_UTF8_TOKEN),
  ("en_wiki_punct_64_2048_bin_token", &EN_WIKI_PUNCT_64_2048_BIN_TOKEN),
  ("en_wiki_alpha_64_64_utf8_token", &EN_WIKI_ALPHA_64_64_UTF8_TOKEN),
  ("en_wiki_alpha_64_64_bin_token", &EN_WIKI_ALPHA_64_64_BIN_TOKEN),
  ("en_wiki_alpha_64_2048_utf8_token", &EN_WIKI_ALPHA_64_2048_UTF8_TOKEN),
  ("en_wiki_alpha_64_2048_bin_token", &EN_WIKI_ALPHA_64_2048_BIN_TOKEN),
  ("en_wiki_128_2048_utf8_token", &EN_WIKI_128_2048_UTF8_TOKEN),
  ("en_wiki_128_2048_bin_token", &EN_WIKI_128_2048_BIN_TOKEN),
];