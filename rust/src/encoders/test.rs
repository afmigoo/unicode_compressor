use rand::prelude::*;
use clap::ValueEnum;

use super::instances;
use super::types::{Encoder};
use super::dictionaries::DICT_INVENTORIES;

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
          .chain([1_000, 10_000].into_iter());

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
        ("Привет, world!", "0˅éyƑnf#"),
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

#[test]
fn dicts_are_mirrored() {
    
    for (token2int_inv, int2token_inv) in DICT_INVENTORIES {
        assert_eq!(token2int_inv.len(), int2token_inv.len());

        for i in 0..token2int_inv.len() {
            assert_eq!(token2int_inv[i].len(), int2token_inv[i].len());
            for token in token2int_inv[i].keys() {
                let int = token2int_inv[i].get(token).unwrap();
                assert_eq!(int2token_inv[i].get(int).unwrap(), token);
            }
        }

    }
}

#[test]
fn dicts_are_sized_powers_of_two() {
    for (i, (token2int_inv, int2token_inv)) in DICT_INVENTORIES.iter().enumerate() {
        let mut t2i_cumulative_size: usize = 1;
        let mut i2t_cumulative_size: usize = 1;
        assert!(token2int_inv.len() == int2token_inv.len(), "token2int_inv and int2token_inv must have the same length ({i})");
        // the last dict may be incomplete, so we don't check it
        for i in 0..token2int_inv.len() - 1 {
            t2i_cumulative_size += token2int_inv[i].len();
            i2t_cumulative_size += int2token_inv[i].len();
            assert!(t2i_cumulative_size.is_power_of_two(), "t2i_cumulative_size is not a power of two: {t2i_cumulative_size} ({i})");
            assert!(i2t_cumulative_size.is_power_of_two(), "i2t_cumulative_size is not a power of two: {i2t_cumulative_size} ({i})");
        }
    }
}

#[test]
fn dicts_are_incremental() {
    for (_, int2token_inv) in DICT_INVENTORIES {
        for i in 0..int2token_inv.len() - 1 {
            let mut keys = int2token_inv[i].keys().collect::<Vec<&u16>>();
            keys.sort();
            for j in 0..keys.len() - 1 {
                assert!(*keys[j] + 1 == *keys[j + 1], "int2token_inv is not incremental ({i}, {j})");
            }
        }
    }
}
