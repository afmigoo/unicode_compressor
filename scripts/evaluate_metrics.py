#!/usr/bin/env python3
"""
Evaluate the compression metrics of the encoders.
Writes metrics.txt file with the results.

usage: python3 scripts/evaluate_metrics.py
"""
import argparse

from pathlib import Path
from typing import Counter
from tqdm import tqdm
from pprint import pprint
import itertools
from collections import defaultdict
import statistics

from dict_generator import dataset, alphabet
from dict_generator.alphabet.types import UnrestrictedAlphabet
from eval import encode, decode

UNIPRESS_BINARY = Path(__file__).parent.parent / 'rust/target/release/unipress'

if __name__ == '__main__':
  corpus_dir = Path(__file__).parent.parent / 'corpus'
  
  datasets = dataset.DATASETS
  alphabets = alphabet.ALPHABETS
  combinations = itertools.product(datasets, alphabets)

  results = defaultdict(list)
  most_used_prefixes = defaultdict(Counter)
  examples = defaultdict(str)

  for ds, alph in combinations:
    if ds.lang != alph.lang and not isinstance(alph, UnrestrictedAlphabet):
      continue

    name = f"{ds}_{alph.name}"

    results[name] = []
    for payload in tqdm(ds.alphabet_filtered('test', alph)):
      stripped_payload = payload.strip()

      if name not in examples:
        examples[name] = stripped_payload

      encoding_result = encode(stripped_payload, 'adaptive')
      decoding_result = decode(encoding_result.encoded_payload, 'adaptive')
      assert decoding_result.decoded_payload == stripped_payload, \
        f"Roundtrip failed: {decoding_result.decoded_payload} != {stripped_payload}"
      
      most_used_prefixes[name][encoding_result.encoded_payload[0]] += 1
      most_used_prefixes['total'][encoding_result.encoded_payload[0]] += 1
      results['total'].append(encoding_result)
      results[name].append(encoding_result)

  print('|Name|compression (avg/mean/std)|User time (avg)|Payload byte size (avg)|N|Example|')
  print('|---|---|---|---|---|---|')
  for name, results in results.items():
    print('|{}|{:.4f} / {:.4f} / {:.4f}|{:.4f}|{:.4f}|{}|{}|'.format(
      name,
      sum(result.compression_rate for result in results) / len(results),
      statistics.median(result.compression_rate for result in results),
      statistics.stdev(result.compression_rate for result in results),
      sum(result.user_time for result in results) / len(results),
      sum(len(result.payload.encode('utf-8')) for result in results) / len(results),
      len(results),
      examples[name][:50] + '...' if len(examples[name]) > 50 else examples[name]
    ))

  pprint(most_used_prefixes)

