use super::types::{Encoder, EchoEncoder, AdaptiveEncoder};

mod en_32_utf8_map;
mod en_alpha_64_utf8_map;
mod en_punct_64_utf8_map;
mod ru_32_utf8_map;
mod ru_alpha_64_utf8_map;
mod ru_punct_64_utf8_map;

mod en_wiki_32_1914_utf8_tkn;
mod en_wiki_alpha_64_1914_utf8_tkn;
mod en_wiki_punct_64_1914_utf8_tkn;

mod ru_meshcoretel_32_1914_utf8_tkn;
mod ru_meshcoretel_alpha_64_1914_utf8_tkn;
mod ru_meshcoretel_punct_64_1914_utf8_tkn;
mod ru_wiki_32_1914_utf8_tkn;
mod ru_wiki_alpha_64_1914_utf8_tkn;
mod ru_wiki_punct_64_1914_utf8_tkn;

pub const ECHO_ENCODER: EchoEncoder = EchoEncoder;

// available codes:
// ['!', '"', '#', '$', '%', '&', "'", '(', ')', '*', '+', ',', '-', '.', '/', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ':', ';', '<', '=', '>', '?', '@', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '[', '\\', ']', '^', '_', '`', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', '{', '|', '}', '~']
pub const ADAPTIVE_ENCODER: AdaptiveEncoder = AdaptiveEncoder {
  encoders_inventory: &[
    ('!', &ECHO_ENCODER),
    ('"', &en_32_utf8_map::ENCODER),
    ('#', &en_alpha_64_utf8_map::ENCODER),
    ('$', &en_punct_64_utf8_map::ENCODER),
    ('%', &ru_32_utf8_map::ENCODER),
    ('&', &ru_alpha_64_utf8_map::ENCODER),
    ('\'', &ru_punct_64_utf8_map::ENCODER),
    ('(', &en_wiki_32_1914_utf8_tkn::ENCODER),
    (')', &en_wiki_alpha_64_1914_utf8_tkn::ENCODER),
    ('*', &en_wiki_punct_64_1914_utf8_tkn::ENCODER),
    ('+', &ru_meshcoretel_32_1914_utf8_tkn::ENCODER),
    (',', &ru_meshcoretel_alpha_64_1914_utf8_tkn::ENCODER),
    ('-', &ru_meshcoretel_punct_64_1914_utf8_tkn::ENCODER),
    ('.', &ru_wiki_32_1914_utf8_tkn::ENCODER),
    ('/', &ru_wiki_alpha_64_1914_utf8_tkn::ENCODER),
    ('0', &ru_wiki_punct_64_1914_utf8_tkn::ENCODER),
  ]
};

pub const NAMED_ENCODERS: &[(&str, &dyn Encoder)] = &[
  ("adaptive", &ADAPTIVE_ENCODER),
  ("en_32_utf8_map", &en_32_utf8_map::ENCODER),
  ("en_alpha_64_utf8_map", &en_alpha_64_utf8_map::ENCODER),
  ("en_punct_64_utf8_map", &en_punct_64_utf8_map::ENCODER),
  ("ru_32_utf8_map", &ru_32_utf8_map::ENCODER),
  ("ru_alpha_64_utf8_map", &ru_alpha_64_utf8_map::ENCODER),
  ("ru_punct_64_utf8_map", &ru_punct_64_utf8_map::ENCODER),
  ("en_wiki_32_utf8_tkn", &en_wiki_32_1914_utf8_tkn::ENCODER),
  ("en_wiki_alpha_64_utf8_tkn", &en_wiki_alpha_64_1914_utf8_tkn::ENCODER),
  ("en_wiki_punct_64_utf8_tkn", &en_wiki_punct_64_1914_utf8_tkn::ENCODER),
  ("ru_meshcoretel_32_1914_utf8_tkn", &ru_meshcoretel_32_1914_utf8_tkn::ENCODER),
  ("ru_meshcoretel_alpha_64_1914_utf8_tkn", &ru_meshcoretel_alpha_64_1914_utf8_tkn::ENCODER),
  ("ru_meshcoretel_punct_64_1914_utf8_tkn", &ru_meshcoretel_punct_64_1914_utf8_tkn::ENCODER),
  ("ru_wiki_32_utf8_tkn", &ru_wiki_32_1914_utf8_tkn::ENCODER),
  ("ru_wiki_alpha_64_utf8_tkn", &ru_wiki_alpha_64_1914_utf8_tkn::ENCODER),
  ("ru_wiki_punct_64_utf8_tkn", &ru_wiki_punct_64_1914_utf8_tkn::ENCODER),
];
