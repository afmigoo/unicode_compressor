from dataclasses import dataclass
from typing import Literal, Optional
from pathlib import Path

from .rustgen import write
from .trainer import train_bpe_dict
from .alphabet.types import AlphabetVariant
from .dataset import Dataset

@dataclass
class DictVariant:
    alphabet: AlphabetVariant

    def __str__(self):
        return f'{self.alphabet.lang}_{self.alphabet.name}'

    def render(self, output_file: Path | str):
        raise NotImplementedError

@dataclass
class MapDictVariant(DictVariant):  
    def render(self, output_file: Path | str):
        static_dict = {ch: i + 1 for i, ch in enumerate(self.alphabet.alphabet)}
        return write.write_rust_dict(static_dict, output_file)

@dataclass
class TokenDictVariant(DictVariant):
    vocab_size: int
    dataset: Dataset

    def __str__(self):
        return f'{self.dataset.lang}_{self.dataset.name}_{self.alphabet.name}_{self.vocab_size}'    

    def render(self, output_file: Path | str):
        static_dict = train_bpe_dict(self.dataset, self.alphabet, self.vocab_size)
        return write.write_rust_dict(static_dict, output_file)

@dataclass
class EncoderVariant:
    dict: DictVariant
    type: Literal['map', 'token']
    transport: Literal['utf8', 'bin']

    def __str__(self):
        return f'{self.dict}_{self.transport}_{self.type}'
