import itertools
from typing import Literal

from .types import EncoderVariant, MapEncoderVariant, TokenEncoderVariant
from .dataset import DATASETS
from .alphabet import ALPHABETS

def get_encoder_variants(split: Literal['train', 'test']) -> list[EncoderVariant]:
    options = []
    
    alphabets = ALPHABETS
    datasets = DATASETS
    vocab_sizes = [1914]
    token_types = ['utf8']

    map_options = itertools.product(alphabets, token_types)
    for (alph, token_type) in map_options:
        # map dicts can not be trained with no alphabet
        if alph['alphabet'] is None:
            continue
        options.append(MapEncoderVariant(
            lang=alph['lang'],
            alphabet=alph['alphabet'],
            alphabet_name=alph['name'],
            token_type=token_type
        ))

    token_options = itertools.product(datasets, alphabets, vocab_sizes, token_types)
    for (ds, alph, vocab_size, token_type) in token_options:
        # token dicts can be trained with None alphabet
        # this means that the token dicts are trained with all the characters
        if ds['lang'] != alph['lang'] and alph['alphabet'] is not None:
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
