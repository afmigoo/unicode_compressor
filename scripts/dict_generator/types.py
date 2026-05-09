from dataclasses import dataclass
from typing import Literal
from pathlib import Path

@dataclass
class EncoderVariant:
    lang: str
    alphabet: list[str]
    alphabet_name: str
    token_type: Literal['bin', 'utf8']

@dataclass
class MapEncoderVariant(EncoderVariant):
    def __str__(self):
        return f'{self.lang}_{self.alphabet_name}_{self.token_type}_map'

@dataclass
class TokenEncoderVariant(EncoderVariant):
    dataset: Path | str
    vocab_size: int

    def __str__(self):
        ds_name = Path(self.dataset).stem
        ds_name = ds_name.removeprefix(f'{self.lang}_')
        ds_name = ds_name.removesuffix('_train')
        return f'{self.lang}_{ds_name}_{self.alphabet_name}_{self.vocab_size}_{self.token_type}_tkn'
