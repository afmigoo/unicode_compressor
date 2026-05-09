from pathlib import Path
import re

from datasets import load_dataset

def meshcoretel_filter(s: str):
  # remove mentions as they do not make sense in the encoded format
  return re.sub(r'^@\[[^\]]*\] ', '', s)

def raw(file: str | Path):
  ds = load_dataset('json', data_files=str(file))['train']
  for s in ds:
    if 'meshcoretel' in file:
      s['text'] = meshcoretel_filter(s['text'])
    yield s['text']

def alphabet_filtered(file: str | Path, alphabet: list[str] | None):
  alphabet_map = None if alphabet is None else set(alphabet)
  for s in raw(file):
    if alphabet is None:
      yield s
      continue
    filtered_s = ''
    for ch in s:
      if ch in alphabet_map:
        filtered_s += ch
      elif ch.lower() in alphabet_map:
        filtered_s += ch.lower()
      elif ch.upper() in alphabet_map:
        filtered_s += ch.upper()
    if len(filtered_s) > 0:
      if not 'coding' in file:
        filtered_s = re.sub(r'\s+', ' ', filtered_s)
      yield filtered_s

DATASETS = [
  {'lang': 'ru', 'name': 'wiki', 'train': 'corpus/ru_wiki_train.jsonl', 'test': 'corpus/ru_wiki_test.jsonl'},
  {'lang': 'en', 'name': 'wiki', 'train': 'corpus/en_wiki_train.jsonl', 'test': 'corpus/en_wiki_test.jsonl'},
  {'lang': 'ru', 'name': 'meshcoretel', 'train': 'corpus/ru_meshcoretel_train.jsonl', 'test': 'corpus/ru_meshcoretel_test.jsonl'},
]
