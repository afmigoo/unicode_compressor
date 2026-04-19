from pathlib import Path
import json

from encoders import BpeEncoder
from alphabet import ALPHABET

if __name__ == "__main__":
    output_dir = Path(__file__).parent.parent / "web/v0/fixtures/dictionaries"
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
        'bpe_wiki': BpeEncoder(ALPHABET, train_files['wiki']),
        'bpe_wiki_ru': BpeEncoder(ALPHABET, train_files['wiki_ru']),
        'bpe_wiki_en': BpeEncoder(ALPHABET, train_files['wiki_en']),
        'bpe_meshcoretel_ru': BpeEncoder(ALPHABET, train_files['meshcoretel_ru']),
        'bpe_coding': BpeEncoder(ALPHABET, train_files['coding']),
    }

    for name, encoder in encoders.items():
        with open(output_dir / f"{name}.json", "w", encoding="utf-8") as f:
            json.dump(encoder.token2unicode, f, indent=2, ensure_ascii=False)
