use std::process::exit;
use std::io::{stdin,read_to_string};
use clap::Parser;

use unipress::encoders::{consts::ENCODER_NAMES, errors::Error, encode, decode};
use unipress::options::{CompressionLevel, EncodeOptions};

/// Unipress - Unicode-based text compression tool. 
/// Takes payload from stdin and outputs the processed payload to stdout.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Decode the payload, by default encode is assumed
    #[arg(short, long, default_value_t = false)]
    decode: bool,
    /// Encoder to use
    #[arg(short, long, default_value = "bpe_meshcoretel_ru")]
    encoder: String,
    /// Compression level
    #[arg(short, long, default_value_t = CompressionLevel::Fast)]
    compression_level: CompressionLevel,
    /// List available encoders
    #[arg(short, long, default_value_t = false)]
    list_encoders: bool,
}

fn main() -> Result<(), Error> {
  let args = Args::parse();

  if args.list_encoders {
    for encoder in ENCODER_NAMES {
      println!("{}", encoder);
    }
    return Ok(());
  }

  let payload = read_to_string(stdin()).expect("Failed to read payload string");
  let payload = payload.trim();
  let options = EncodeOptions { level: args.compression_level };

  let processed_payload = if args.decode {
    decode(payload, &args.encoder)
  } else {
    encode(payload, &args.encoder, &options)
  };
  
  match processed_payload {
    Ok(processed_payload) => { 
      println!("{}", processed_payload);
      return Ok(());
    }
    Err(e) => { 
      eprintln!("Failed to process payload: {e}");
      exit(1);
    }
  }
}
