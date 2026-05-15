use rand::prelude::*;
use clap::ValueEnum;

use super::instances;
use super::types::Encoder;

use crate::options::{EncodeOptions, TokenizationStrategy};

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

#[test]
fn roundtrip_hardcoded() {
    let payload_pairs = [
        ("Hello, world!", "2ڽnы®࠾#"),
        ("Привет, мир!", "+ۥȕé׆#"),
        ("abcdefghijklmnopqrstuvwxyz", "$riTHZu):7lnFJ,)f9Ql;A"),
        ("wikipedia is an educational portal", "2ôm͍սڃǾЂ´"),
        ("0123456789", "!0123456789"),
    ];

    for (payload_target, encoded_target) in payload_pairs {
        let options = EncodeOptions {
            tokenization_strategy: TokenizationStrategy::FirstMatch,
        };
        let encoded = instances::ADAPTIVE_ENCODER.encode(payload_target, &options).unwrap();
        assert_eq!(encoded, encoded_target);
        let decoded = instances::ADAPTIVE_ENCODER.decode(&encoded_target).unwrap();
        assert_eq!(decoded, payload_target);
    }
}