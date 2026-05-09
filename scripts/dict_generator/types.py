from dataclasses import dataclass
from typing import Literal
from pathlib import Path

from .rustgen import render

@dataclass
class EncoderVariant:
    lang: str
    alphabet: list[str]
    alphabet_name: str
    token_type: Literal['bin', 'utf8']
    
    def render(self, output_file: Path | str):
        raise NotImplementedError

@dataclass
class MapEncoderVariant(EncoderVariant):
    def __str__(self):
        return f'{self.lang}_{self.alphabet_name}_{self.token_type}_map'
    
    def render(self, output_file: Path | str):
        return render.render_map_dict(output_file, self.alphabet, self.token_type)

@dataclass
class TokenEncoderVariant(EncoderVariant):
    dataset: Path | str
    vocab_size: int

    def render(self, output_file: Path | str):
        return render.render_bpe_dict(
            output_file, self.dataset, 
            self.alphabet, self.vocab_size, 
            self.token_type)

    def __str__(self):
        ds_name = Path(self.dataset).stem
        ds_name = ds_name.removesuffix('_train')
        return f'{ds_name}_{self.alphabet_name}_{self.vocab_size}_{self.token_type}_tkn'
