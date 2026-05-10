import itertools
from typing import Literal

from .types import DictVariant, MapDictVariant, TokenDictVariant, EncoderVariant
from .dataset import DATASETS
from .alphabet import ALPHABETS
from .alphabet.types import UnrestrictedAlphabet

def get_dict_variants() -> list[DictVariant]:
    options = []
    
    vocab_sizes = [64, 2048]
    token_dict_options = itertools.product(ALPHABETS, DATASETS, vocab_sizes)

    for alph in ALPHABETS:
        if isinstance(alph, UnrestrictedAlphabet):
            continue
        options.append(MapDictVariant(
            alphabet=alph,
        ))

    for (alph, ds, vocab_size) in token_dict_options:
        if alph.lang != ds.lang and not isinstance(alph, UnrestrictedAlphabet):
            continue
        if isinstance(alph, UnrestrictedAlphabet):
            if vocab_size < 2000:
                print(f'Skipping {alph} {ds} {vocab_size} because it has less than 2000 tokens')
                continue
        elif len(alph.alphabet) > vocab_size:
            print(f'Skipping {alph} {ds} {vocab_size}, alphabet is bigger than the vocabulary size ({len(alph.alphabet)} > {vocab_size})')
            continue
        options.append(TokenDictVariant(
            alphabet=alph,
            vocab_size=vocab_size,
            dataset=ds,
        ))
    
    return options

def get_encoder_variants() -> list[EncoderVariant]:
    options = []
    dict_variants = get_dict_variants()
    encoder_types = ['map', 'token']
    transports = ['utf8', 'bin']

    allower_types ={
        'map': MapDictVariant,
        'token': TokenDictVariant,
    }

    encoder_variants = itertools.product(dict_variants, encoder_types, transports)
    for (dict_variant, encoder_type, transport) in encoder_variants:
        if not isinstance(dict_variant, allower_types[encoder_type]):
            continue
        options.append(EncoderVariant(
            dict=dict_variant,
            type=encoder_type,
            transport=transport,
        ))
    
    return options
