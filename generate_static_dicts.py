from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.trainers import BpeTrainer

from dict_generator import render_bpe_dict, alphabet
from pathlib import Path
import itertools

if __name__ == '__main__':
    corpus_dir = Path(__file__).parent / 'corpus'
    output_dir = Path(__file__).parent / 'rust/src/dictionaries/bpe'

    # utf-8 dicts
    alphabets = [
      {'lang': 'ru', 'name': '32', 'alphabet': alphabet.RU_32},
      {'lang': 'ru', 'name': 'punct_64', 'alphabet': alphabet.RU_PUNCT_64},
      {'lang': 'ru', 'name': 'alpha_64', 'alphabet': alphabet.RU_ALPHA_64},
      {'lang': 'en', 'name': '32', 'alphabet': alphabet.EN_32},
      {'lang': 'en', 'name': 'punct_64', 'alphabet': alphabet.EN_PUNCT_64},
      {'lang': 'en', 'name': 'alpha_64', 'alphabet': alphabet.EN_ALPHA_64},
    ]
    datasets = [
      {'lang': 'ru', 'name': 'wiki', 'file': corpus_dir / 'ru_wiki_train.jsonl'},
      {'lang': 'en', 'name': 'wiki', 'file': corpus_dir / 'en_wiki_train.jsonl'},
    ]
    options = itertools.product(alphabets, datasets)

    for (alph, ds) in options:
        if ds['lang'] != alph['lang']:
          continue
        output_file = output_dir / f'{ds["lang"]}_{ds["name"]}_{alph["name"]}_utf8.rs'
        print(f"Generating {output_file}...")
        render_bpe_dict(
          output_file=output_file,
          dataset_file=ds['file'],
          alphabet=alph['alphabet'],
          vocab_size=1914,
          token_type='utf8',
        )
