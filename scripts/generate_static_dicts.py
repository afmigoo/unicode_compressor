from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.trainers import BpeTrainer

from dict_generator import render_bpe_dict, render_map_dict, alphabet
from dict_generator.dataset import DATASETS
from dict_generator.alphabet import ALPHABETS
from pathlib import Path
import itertools

if __name__ == '__main__':
    corpus_dir = Path(__file__).parent.parent / 'corpus'
    output_dir = Path(__file__).parent.parent / 'rust/src/encoders/instances'

    alphabets = ALPHABETS
    datasets = DATASETS
    #token_types = ['utf8', 'bin']
    token_types = ['utf8']

    #### utf-8 dicts ####
    # map dicts
    options = itertools.product(alphabets, token_types)
    for (alph, token_type) in options:
        name = f'{alph["lang"]}_{alph["name"]}_{token_type}_map'
        output_file = output_dir / f'{name}.rs'
        print(f"Generating {name}...")
        render_map_dict(output_file, alph['alphabet'], token_type)

    # token-to-unicode dicts
    options = itertools.product(alphabets, datasets)

    for (alph, ds) in options:
        if ds['lang'] != alph['lang']:
          continue
        name = f'{ds["lang"]}_{ds["name"]}_{alph["name"]}_utf8_tkn'
        output_file = output_dir / f'{name}.rs'
        print(f"Generating {name}...")
        render_bpe_dict(
          output_file=output_file,
          dataset_file=ds['train'],
          alphabet=alph['alphabet'],
          vocab_size=1914,
          token_type='utf8'
        )
