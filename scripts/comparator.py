from v0.demo_base64 import Base64Encoder
from v0.demo_utf8 import UTF8Encoder
from v0.demo_base91 import Base91Encoder
from v0.demo_decider import DeciderEncoder
from v0.demo_base85 import Base85Encoder
from encoders import BpeEncoder

import os
import json
from pathlib import Path
from string import punctuation

ALPHABET = " \n"
ALPHABET += punctuation
ALPHABET += "0123456789"
ALPHABET += "абвгдеёжзийклмнопрстуфхцчшщъыьэюя"
ALPHABET += "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"
ALPHABET += "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
ALPHABET += "abcdefghijklmnopqrstuvwxyz"


print(f"Alphabet length: {len(ALPHABET)}")

encoders = {
    'utf8': UTF8Encoder(ALPHABET),
    'utf8_optimize': UTF8Encoder(ALPHABET, optimize_preference=['cyrillic', 'english']),
    'base64': Base64Encoder(ALPHABET),
    'base64_compress': Base64Encoder(ALPHABET, compress=True),
    'base91': Base91Encoder(ALPHABET, compress=False),
    'base91_compress': Base91Encoder(ALPHABET, compress=True),
    'base85': Base85Encoder(ALPHABET, compress=False),
    'base85_compress': Base85Encoder(ALPHABET, compress=True),
    'bpe_wiki': BpeEncoder(ALPHABET, train_files=[
        Path(__file__).parent / "corpus/processed/wiki/ru.txt",
        Path(__file__).parent / "corpus/processed/wiki/en.txt",
    ]),
    'bpe_wiki_ru': BpeEncoder(ALPHABET, train_files=[
        Path(__file__).parent / "corpus/processed/wiki/ru.txt",
    ]),
    'bpe_wiki_en': BpeEncoder(ALPHABET, train_files=[
        Path(__file__).parent / "corpus/processed/wiki/en.txt",
    ]),
    'bpe_meshcoretel_ru': BpeEncoder(ALPHABET, train_files=[
        Path(__file__).parent / "corpus/processed/meshcoretel/ru.txt",
    ]),
    'bpe_coding': BpeEncoder(ALPHABET, compress=False, train_files=[
        Path(__file__).parent / "corpus/processed/coding/c/train.txt",
        Path(__file__).parent / "corpus/processed/coding/arduino/train.txt",
        Path(__file__).parent / "corpus/processed/coding/python/train.txt",
        Path(__file__).parent / "corpus/processed/coding/go/train.txt",
        Path(__file__).parent / "corpus/processed/coding/rust/train.txt",
        Path(__file__).parent / "corpus/processed/coding/clojure/train.txt",
    ])
}
encoders['decider'] = DeciderEncoder(ALPHABET, list(encoders.values()))

dataset = json.load(open(Path(__file__).parent / "eval_dataset/ru_tiny_msg.json", "r", encoding="utf-8"))

results = {
    enc: []
    for enc in encoders
}
for encoder_name, encoder in encoders.items():
    print('-' * os.get_terminal_size().columns)
    print(f"{encoder_name}:")
    for text in dataset:
        encoded = encoder.encode(text)
        print(f"{text} -> {encoded}")
        results[encoder_name].append((text, encoded))
        assert encoder.decode(encoded) == text, f"Payload corrupted: {text} -> {encoded} -> {encoder.decode(encoded)}"

comp_results = {
    encoder_name: []
    for encoder_name in encoders
}
for encoder_name, results in results.items():
    size_differences = []
    for text, encoded in results:
        start_size = len(text.encode('utf-8'))
        end_size = len(encoded.encode('utf-8'))
        size_differences.append(end_size / start_size)
    comp_results[encoder_name] = sum(size_differences) / len(size_differences)
    print(f"{encoder_name}: {comp_results[encoder_name]}")
