import subprocess
import resource
from dataclasses import dataclass
from pathlib import Path
from tqdm import tqdm
import itertools
from collections import defaultdict
import statistics

from dict_generator import dataset, alphabet
from eval import encode, decode

UNIPRESS_BINARY = Path(__file__).parent.parent / 'rust/target/release/unipress'

if __name__ == '__main__':
  corpus_dir = Path(__file__).parent.parent / 'corpus'
  datasets = dataset.DATASETS
  alphabets = alphabet.ALPHABETS
  combinations = itertools.product(datasets, alphabets)
  results = defaultdict(list)
  examples = defaultdict(str)

  for ds, alph in combinations:
    if ds['lang'] != alph['lang']:
      continue
    name = f"{ds['lang']}_{ds['name']}_{alph['name']}"
    print(f"Evaluating {name}...")
    results[name] = []
    if name not in examples:
      examples[name] = next(dataset.alphabet_filtered(ds['test'], alph['alphabet'])).strip()
    for payload in tqdm(dataset.alphabet_filtered(ds['test'], alph['alphabet'])):
      stripped_payload = payload.strip()
      encoding_result = encode(stripped_payload, 'adaptive')
      decoding_result = decode(encoding_result.encoded_payload, 'adaptive')
      assert decoding_result.decoded_payload == stripped_payload, f"Roundtrip failed: {decoding_result.decoded_payload} != {stripped_payload}"
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
