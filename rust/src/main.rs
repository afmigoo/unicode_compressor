use std::process::exit;
use std::io::{stdin,read_to_string};
use clap::Parser;

use unipress::encoders::{instances::NAMED_ENCODERS, errors::Error, encode, decode, get_encoder};
use unipress::options::{EncodeOptions, TokenizationStrategy};

/// Unipress - Unicode-based text compression tool. 
/// Takes payload from stdin and outputs the processed payload to stdout.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Decode the payload, by default encode is assumed
    #[arg(short, long, default_value_t = false)]
    decode: bool,
    /// Encoder to use
    #[arg(short, long, default_value = "adaptive")]
    encoder: String,
    /// Compression level
    #[arg(short, long, default_value_t = TokenizationStrategy::FirstMatch)]
    tokenization_strategy: TokenizationStrategy,
    /// List available encoders
    #[arg(short, long, default_value_t = false)]
    list_encoders: bool,
    /// List the alphabet of the selected encoder
    #[arg(long, default_value_t = false)]
    alphabet: bool,
}

fn main() -> Result<(), Error> {
  let args = Args::parse();

  if args.list_encoders {
    for (name, _) in NAMED_ENCODERS {
      println!("{}", name);
    }
    return Ok(());
  }

  if args.alphabet {
    let encoder = get_encoder(&args.encoder)?;
    println!("{}", encoder.get_alphabet());
    return Ok(());
  }

  let payload = read_to_string(stdin())
    .expect("Failed to read payload string")
    .replace("\r\n", "\n"); // Replace Windows CRLF with Unix LF
  let options = EncodeOptions { tokenization_strategy: args.tokenization_strategy.clone() };

  let processed_payload = if args.decode {
    decode(&payload, &args.encoder)
  } else {
    encode(&payload, &args.encoder, &options)
  };
  
  match processed_payload {
    Ok(processed_payload) => { 
      print!("{}", processed_payload);
      return Ok(());
    }
    Err(e) => { 
      eprintln!("Failed to process payload: {e}");
      exit(1);
    }
  }
}
