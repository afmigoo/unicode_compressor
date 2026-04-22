from pathlib import Path
import json

from encoders import BpeEncoder
from alphabet import ALPHABET


def rust_str_lit(s: str) -> str:
    escaped = (
        s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
        .replace("\0", "\\0")
    )
    return f"\"{escaped}\""


if __name__ == "__main__":
    # GENERATE_FORMAT = 'json'
    GENERATE_FORMAT = 'rust'
    output_dirs = {
        'json': "web/v0/fixtures/dictionaries",
        'rust': "src/dictionaries/bpe",
    }

    output_dir = Path(__file__).parent.parent / output_dirs[GENERATE_FORMAT]
    output_dir.mkdir(exist_ok=True)

    train_files = {
        'wiki': [
            Path(__file__).parent / "corpus/processed/wiki/ru.txt",
            Path(__file__).parent / "corpus/processed/wiki/en.txt",
        ],
        'wiki_ru': [Path(__file__).parent / "corpus/processed/wiki/ru.txt"],
        'wiki_en': [Path(__file__).parent / "corpus/processed/wiki/en.txt"],
        'meshcoretel_ru': [Path(__file__).parent / "corpus/processed/meshcoretel/ru.txt"],
        'coding': [
            Path(__file__).parent / "corpus/processed/coding/c/train.txt",
            Path(__file__).parent / "corpus/processed/coding/arduino/train.txt",
            Path(__file__).parent / "corpus/processed/coding/python/train.txt",
            Path(__file__).parent / "corpus/processed/coding/go/train.txt",
            Path(__file__).parent / "corpus/processed/coding/rust/train.txt",
            Path(__file__).parent / "corpus/processed/coding/clojure/train.txt",
        ],
    }

    encoders = {
        'wiki': BpeEncoder(ALPHABET, train_files['wiki']),
        'wiki_ru': BpeEncoder(ALPHABET, train_files['wiki_ru']),
        'wiki_en': BpeEncoder(ALPHABET, train_files['wiki_en']),
        'meshcoretel_ru': BpeEncoder(ALPHABET, train_files['meshcoretel_ru']),
        'coding': BpeEncoder(ALPHABET, train_files['coding']),
    }

    for name, encoder in encoders.items():
        if GENERATE_FORMAT == 'json':
            with open(output_dir / f"{name}.json", "w", encoding="utf-8") as f:
                json.dump(encoder.token2unicode, f, indent=2, ensure_ascii=False)
        elif GENERATE_FORMAT == 'rust':
            with open(output_dir / f"{name}.rs", "w", encoding="utf-8") as f:
                token_max_bytes = max(len(token.encode('utf-8')) for token in encoder.token2unicode.keys())
                f.write("use phf::{phf_map, Map};\n\n")
                f.write(f"pub const TOKEN_MAX_CHARS: u8 = {token_max_bytes};\n\n")
                # token to unicode
                f.write(f"pub static TOKEN2UNICODE: Map<&'static str, &'static str> = phf_map! {{\n")
                for token, unicode in encoder.token2unicode.items():
                    f.write(f"  {rust_str_lit(token)} => {rust_str_lit(unicode)},\n")
                f.write("};\n\n")
                # unicode to token 
                f.write(f"pub static UNICODE2TOKEN: Map<&'static str, &'static str> = phf_map! {{\n")
                for token, unicode in encoder.token2unicode.items():
                    f.write(f"  {rust_str_lit(unicode)} => {rust_str_lit(token)},\n")
                f.write("};\n")
        else:
            raise ValueError(f"Invalid generate format: {GENERATE_FORMAT}")
