import itertools
from typing import Literal

from .types import DictVariant, MapDictVariant, TokenDictVariant, EncoderVariant
from .dataset import DATASETS
from .alphabet import ALPHABETS
from .alphabet.types import UnrestrictedAlphabet

VOCAB_SIZES = [64, 512, 2048]
MAX_VOCAB_SIZE = VOCAB_SIZES[-1]

def get_dict_variants() -> list[DictVariant]:
    options = []
    
    token_dict_options = itertools.product(DATASETS, ALPHABETS, VOCAB_SIZES)

    for alph in ALPHABETS:
        if isinstance(alph, UnrestrictedAlphabet):
            continue
        options.append(MapDictVariant(
            alphabet=alph,
        ))

    for (ds, alph, vocab_size) in token_dict_options:
        # leaving variants with only matching language on dataset and alphabet
        if alph.lang != ds.lang and not isinstance(alph, UnrestrictedAlphabet):
            continue
        if not isinstance(alph, UnrestrictedAlphabet) and len(alph.alphabet) > vocab_size:
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

    allowed_types ={
        'map': MapDictVariant,
        'token': TokenDictVariant,
    }

    encoder_variants = itertools.product(dict_variants, encoder_types, transports)
    for (dict_variant, encoder_type, transport) in encoder_variants:
        if not isinstance(dict_variant, allowed_types[encoder_type]):
            continue
        # for token encoders with utf8 transport leaving only max sized variants
        if isinstance(dict_variant, TokenDictVariant) and transport == 'utf8' and dict_variant.vocab_size < MAX_VOCAB_SIZE:
            continue
        # for map encoders leaving only binary transport
        if isinstance(dict_variant, MapDictVariant) and transport != 'bin':
            continue
        options.append(EncoderVariant(
            dict=dict_variant,
            type=encoder_type,
            transport=transport,
        ))
    
    return options
