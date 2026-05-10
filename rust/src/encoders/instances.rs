use super::types::{Encoder, EchoEncoder, AdaptiveEncoder};

mod en_128_utf8_map;
mod ru_128_utf8_map;

mod ru_32_bin_map;

mod en_wiki_128_1914_utf8_tkn;
mod en_wiki_all_1914_utf8_tkn;
mod ru_wiki_128_1914_utf8_tkn;
mod ru_wiki_all_1914_utf8_tkn;
mod ru_meshcoretel_128_1914_utf8_tkn;
mod ru_meshcoretel_all_1914_utf8_tkn;

pub const ECHO_ENCODER: EchoEncoder = EchoEncoder;

// available codes:
// ['!', '"', '#', '$', '%', '&', "'", '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~']
pub const ADAPTIVE_ENCODER: AdaptiveEncoder = AdaptiveEncoder {
  encoders_inventory: &[
    ('!', &ECHO_ENCODER),
    ('"', &en_128_utf8_map::ENCODER),
    ('#', &ru_128_utf8_map::ENCODER),
    ('$', &en_wiki_128_1914_utf8_tkn::ENCODER),
    ('%', &en_wiki_all_1914_utf8_tkn::ENCODER),
    ('&', &ru_wiki_128_1914_utf8_tkn::ENCODER),
    ('\'', &ru_wiki_all_1914_utf8_tkn::ENCODER),
    ('(', &ru_meshcoretel_128_1914_utf8_tkn::ENCODER),
    (')', &ru_meshcoretel_all_1914_utf8_tkn::ENCODER),
  ]
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("adaptive", &ADAPTIVE_ENCODER),
  ("en_128_utf8_map", &en_128_utf8_map::ENCODER),
  ("ru_128_utf8_map", &ru_128_utf8_map::ENCODER),
  ("en_wiki_128_1914_utf8_tkn", &en_wiki_128_1914_utf8_tkn::ENCODER),
  ("en_wiki_all_1914_utf8_tkn", &en_wiki_all_1914_utf8_tkn::ENCODER),
  ("ru_wiki_128_1914_utf8_tkn", &ru_wiki_128_1914_utf8_tkn::ENCODER),
  ("ru_wiki_all_1914_utf8_tkn", &ru_wiki_all_1914_utf8_tkn::ENCODER),
  ("ru_meshcoretel_128_1914_utf8_tkn", &ru_meshcoretel_128_1914_utf8_tkn::ENCODER),
  ("ru_meshcoretel_all_1914_utf8_tkn", &ru_meshcoretel_all_1914_utf8_tkn::ENCODER),
  ("ru_32_bin_map", &ru_32_bin_map::ENCODER),
];
