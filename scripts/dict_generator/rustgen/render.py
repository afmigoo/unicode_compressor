from pathlib import Path
from typing import Literal

from . import write
from .. import trainer
from ..alphabet.types import AlphabetVariant

def get_printable_utf8(size: int) -> list[str]:
    result = []
    for i in range(pow(2, 8 * size)):
        try:
            ch = chr(i)
        except ValueError:
            break
        if ch.isprintable() and not ch.isspace() and len(ch.encode('utf-8')) == size:
            result.append(ch)

    return result

def _render_static_dict(
  output_file: str | Path,
  static_dict: dict[str, int],
  encoder_type: Literal['map', 'token'],
):
    write.write_rust_struct(static_dict, output_file, encoder_type)

def render_map_dict(
  output_file: str | Path,
  alphabet: AlphabetVariant,
):
    static_dict = {ch: i for i, ch in enumerate(alphabet.alphabet)}
    _render_static_dict(output_file, static_dict, 'map')

def render_bpe_dict(
    output_file: str | Path,
    dataset_file: str | Path,
    alphabet: list[str],
    vocab_size: int,
):
    token2int = trainer.train_bpe_dict(dataset_file, alphabet, vocab_size)
    _render_static_dict(output_file, token2int, 'token')
