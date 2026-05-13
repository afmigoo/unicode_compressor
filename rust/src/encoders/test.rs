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
        let lengths = (0..20)
          .chain(45..55)
          .chain([100, 1000, 10000].into_iter());

        for payload_size in lengths {
          let payload = String::from_iter(alphabet.chars().sample(&mut rng, payload_size));

          roundtrip_assert(*name, *encoder, &payload);
          roundtrip_assert(*name, &instances::ADAPTIVE_ENCODER, &payload);
        }

    }
}
