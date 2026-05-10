import itertools
from typing import Literal

from .types import DictVariant, MapDictVariant, TokenDictVariant
from .dataset import DATASETS
from .alphabet import ALPHABETS
from .alphabet.types import UnrestrictedAlphabet

def get_dict_variants() -> list[DictVariant]:
    options = []
    
    vocab_sizes = [64]
    token_dict_options = itertools.product(ALPHABETS, DATASETS, vocab_sizes)

    for alph in ALPHABETS:
        options.append(MapDictVariant(
            alphabet=alph,
        ))

    for (alph, ds, vocab_size) in token_dict_options:
        options.append(TokenDictVariant(
            alphabet=alph,
            vocab_size=vocab_size,
            dataset=ds,
        ))
    
    return options

def get_encoder_variants(split: Literal['train', 'test']) -> list[EncoderVariant]:
    options = []
    
    alphabets = ALPHABETS
    datasets = DATASETS
    vocab_sizes = [64]
    token_types = ['bin']

    map_options = itertools.product(alphabets, token_types)
    for (alph, token_type) in map_options:
        # map dicts can not be trained with no alphabet
        if alph['alphabet'] is None:
            continue
        if token_type not in alph['token_type']:
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
        if token_type not in alph['token_type']:
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
