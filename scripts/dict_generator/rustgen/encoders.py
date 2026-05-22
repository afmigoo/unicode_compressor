from math import log2, ceil

from dict_generator.variants import (
  get_encoder_variants,
  EncoderVariant,
  MapDictVariant,
  TokenDictVariant,
)
from .const import INSTANCES_RS

def compile_transport(encoder_variant: EncoderVariant) -> str:
  if isinstance(encoder_variant.dict, TokenDictVariant):
    vocab_size = encoder_variant.dict.vocab_size
  elif isinstance(encoder_variant.dict, MapDictVariant):
    vocab_size = len(encoder_variant.dict.alphabet.alphabet)
  else:
    raise ValueError(f'Invalid encoder variant: {encoder_variant}')

  rust_transport_map = {
    'utf8': 'UTF8',
    'bin': f'BIN({ceil(log2(vocab_size))})',
  }
  return rust_transport_map[encoder_variant.transport]


def build_encoders() -> list[dict]:
  return [{
    'name': str(enc),
    'dict_name': str(enc.dict),
    'struct': f'{enc.type.capitalize()}Encoder',
    'transport': compile_transport(enc),
  } for enc in get_encoder_variants()]
