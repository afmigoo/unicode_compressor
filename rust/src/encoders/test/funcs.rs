use rand::prelude::*;
use clap::ValueEnum;

use crate::encoders::get_encoder;
use crate::encoders::instances;
use crate::encoders::types::Encoder;
use crate::options::{EncodeOptions, TokenizationStrategy};

use super::generated_adaptive_alphabet_roundtrips::ADAPTIVE_ALPHABET_ROUNDTRIPS;
use super::generated_alphabet_roundtrips::ENCODER_ALPHABET_ROUNDTRIPS;

fn roundtrip_assert(name: &str, encoder: &dyn Encoder, payload: &str) {
    for strategy in TokenizationStrategy::value_variants() {
        let options = EncodeOptions {
            tokenization_strategy: *strategy,
        };

        let encoded = encoder.encode(payload, &options).unwrap_or_else(|e| {
            panic!("encode {name} ({strategy}): {e}");
        });
        let decoded = encoder.decode(&encoded).unwrap_or_else(|e| {
            panic!("decode {name} ({strategy}): {e}");
        });

        assert_eq!(
            decoded, payload,
            "roundtrip mismatch for encoder {name} with strategy {strategy}"
        );
    }
}

#[test]
fn roundtrip_encoder_alphabet() {
    for (name, encoder) in instances::NAMED_ENCODERS {
        if *name == "adaptive" {
            continue;
        }

        let alphabet = encoder.get_alphabet();

        roundtrip_assert(*name, *encoder, &alphabet);
        roundtrip_assert(*name, &instances::ADAPTIVE_ENCODER, &alphabet);
    }
}

#[test]
fn roundtrip_encoder_random_payloads() {
    let mut rng: ThreadRng = rand::rng();

    for (name, encoder) in instances::NAMED_ENCODERS {
        if *name == "adaptive" {
            continue;
        }

        let alphabet = encoder.get_alphabet();
        let lengths = (0..100)
          .chain([1_000, 10_000, 100_000].into_iter());

        for payload_size in lengths {
          let payload = String::from_iter(alphabet.chars().sample(&mut rng, payload_size));

          roundtrip_assert(*name, *encoder, &payload);
          roundtrip_assert(*name, &instances::ADAPTIVE_ENCODER, &payload);
        }

    }
}

/// Full-alphabet roundtrips from `scripts/generate_static_tests.py`.
#[test]
fn roundtrip_hardcoded_alphabets() {
    let options = EncodeOptions {
        tokenization_strategy: TokenizationStrategy::FirstMatch,
    };

    for (encoder_name, payload, encoded_target) in ENCODER_ALPHABET_ROUNDTRIPS {
        let encoder = get_encoder(encoder_name).unwrap_or_else(|e| panic!("get encoder {encoder_name}: {e}"));
        let encoded = encoder.encode(payload, &options).unwrap_or_else(|e| panic!("encode {encoder_name}: {e}"));
        assert_eq!(encoded, *encoded_target, "encode mismatch for {encoder_name}");
        let decoded = encoder.decode(encoded_target).unwrap_or_else(|e| panic!("decode {encoder_name}: {e}"));
        assert_eq!(decoded, *payload, "decode mismatch for {encoder_name}");
    }
}

/// Each encoder alphabet encoded via adaptive — from `scripts/generate_static_tests.py`.
#[test]
fn roundtrip_hardcoded_adaptive_alphabets() {
    let options = EncodeOptions {
        tokenization_strategy: TokenizationStrategy::FirstMatch,
    };

    for (encoder_name, payload, encoded_target) in ADAPTIVE_ALPHABET_ROUNDTRIPS {
        let encoded = instances::ADAPTIVE_ENCODER
            .encode(payload, &options)
            .unwrap_or_else(|e| panic!("encode adaptive ({encoder_name}): {e}"));

        assert_eq!(
            encoded, *encoded_target,
            "encode mismatch for adaptive ({encoder_name})"
        );

        let decoded = instances::ADAPTIVE_ENCODER
            .decode(encoded_target)
            .unwrap_or_else(|e| panic!("decode adaptive ({encoder_name}): {e}"));

        assert_eq!(decoded, *payload, "decode mismatch for adaptive ({encoder_name})");
    }
}
