#!/usr/bin/env python3
"""
Splits datasets into train and test sets.

The list of datasets is hardcoded in the script.

usage: python3 scripts/split_datasets.py
"""

from pathlib import Path
from datasets import load_dataset

if __name__ == '__main__':
  datasets = [
    Path('corpus/ru_wiki.jsonl'),
    Path('corpus/en_wiki.jsonl'),
    Path('corpus/ru_meshcoretel.jsonl'),
    Path('corpus/coding.jsonl'),
  ]

  for f in datasets:
    ds = load_dataset('json', data_files=str(f), split='train')
    ds = ds.train_test_split(test_size=0.1, shuffle=True, seed=583646)
    ds['train'].to_json(f.with_stem(f.stem + '_train'), force_ascii=False)
    ds['test'].to_json(f.with_stem(f.stem + '_test'), force_ascii=False)

