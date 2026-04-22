use std::process::exit;
use clap::Parser;
use std::io::stdin;

mod dictionaries;
mod encoders;

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
    /// List available encoders
    #[arg(short, long, default_value_t = false)]
    list_encoders: bool,
}

fn main() -> Result<(), encoders::errors::Error> {
  let args = Args::parse();

  if args.list_encoders {
    list_encoders();
    return Ok(());
  }

  let mut payload = String::new();
  stdin().read_line(&mut payload).expect("Failed to read payload string");
  let payload = payload.trim();

  match encoders::process_payload(payload, &args.encoder, args.decode) {
    Err(e) => { 
      eprintln!("Failed to process payload: {e}");
      exit(1);
    }
    Ok(processed_payload) => { 
      println!("{}", processed_payload);
      return Ok(());
    },
  };
}

fn list_encoders() {
  for encoder in encoders::NAMED_ENCODERS.keys() {
    println!("{}", encoder);
  }
}
