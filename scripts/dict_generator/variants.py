import itertools
from typing import Literal

from .types import EncoderVariant, MapEncoderVariant, TokenEncoderVariant
from .dataset import DATASETS
from .alphabet import ALPHABETS

def get_encoder_variants(split: Literal['train', 'test']) -> list[EncoderVariant]:
    alphabets = ALPHABETS
    datasets = DATASETS
    vocab_sizes = [1914]
    token_types = ['utf8']

    map_options = itertools.product(alphabets, token_types)
    token_options = itertools.product(alphabets, datasets, vocab_sizes, token_types)
    options = []
    for (alph, token_type) in map_options:
        options.append(MapEncoderVariant(
            lang=alph['lang'],
            alphabet=alph['alphabet'],
            alphabet_name=alph['name'],
            token_type=token_type
        ))
    for (alph, ds, vocab_size, token_type) in token_options:
        if ds['lang'] != alph['lang']:
          continue
        options.append(TokenEncoderVariant(
            lang=alph['lang'],
            alphabet=alph['alphabet'],
            alphabet_name=alph['name'],
            dataset=ds['train'], 
            vocab_size=vocab_size, 
            token_type=token_type
        ))
    return options
