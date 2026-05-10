from pathlib import Path
from dataclasses import dataclass
import re
from typing import Literal

from datasets import load_dataset

from .alphabet.types import AlphabetVariant

def meshcoretel_filter(s: str):
  # remove mentions as they do not make sense in the encoded format
  return re.sub(r'^@\[[^\]]*\] ', '', s)

@dataclass
class Dataset:
  lang: str
  name: str
  splits: dict[Literal['train', 'test'], str | Path]

  def __str__(self):
    return f'{self.lang}_{self.name}'

  def raw(self, split: Literal['train', 'test']):
    ds = load_dataset('json', data_files=str(self.splits[split]))['train']
    for s in ds:
      if 'meshcoretel' in self.name:
        s['text'] = meshcoretel_filter(s['text'])
      if not 'coding' in self.name:
        s['text'] = re.sub(r'\s+', ' ', s['text'])
      yield s['text']

  def alphabet_filtered(self, split: Literal['train', 'test'], alphabet: AlphabetVariant):
    for s in self.raw(split):
      filtered_s = ''
      for ch in s:
        if alphabet.contains(ch):
          filtered_s += ch
        elif alphabet.contains(ch.lower()):
          filtered_s += ch.lower()
        elif alphabet.contains(ch.upper()):
          filtered_s += ch.upper()
      if len(filtered_s) > 0:
        yield filtered_s

DATASETS: list[Dataset] = [
  # {'lang': 'ru', 'name': 'wiki', 'train': 'corpus/ru_wiki_train.jsonl', 'test': 'corpus/ru_wiki_test.jsonl'},
  # {'lang': 'en', 'name': 'wiki', 'train': 'corpus/en_wiki_train.jsonl', 'test': 'corpus/en_wiki_test.jsonl'},
  Dataset(lang='ru', name='meshcoretel', splits={'train': 'corpus/ru_meshcoretel_train.jsonl', 'test': 'corpus/ru_meshcoretel_test.jsonl'}),
]
