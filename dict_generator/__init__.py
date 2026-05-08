from pathlib import Path
from enum import Enum
from typing import Literal

from . import dataset, alphabet, trainer, rustgen

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
  token_type: Literal['bin', 'utf8'],
):
    if token_type == 'utf8':
        utf8_chars = get_printable_utf8(1) + get_printable_utf8(2) + get_printable_utf8(3)
        if len(static_dict) > len(utf8_chars):
            raise ValueError(f"Static dict size is larger than the number of printable UTF-8 characters: {len(static_dict)}")
        static_dict = {k: utf8_chars[v] for k, v in static_dict.items()}
    rustgen.write(static_dict, output_file, token_type)

def render_map_dict(
  output_file: str | Path,
  alphabet: list[str],
  token_type: Literal['bin', 'utf8'],
):
    static_dict = {ch: i for i, ch in enumerate(alphabet)}
    _render_static_dict(output_file, static_dict, token_type)

def render_bpe_dict(
    output_file: str | Path,
    dataset_file: str | Path,
    alphabet: list[str],
    vocab_size: int,
    token_type: Literal['bin', 'utf8'],
):
    token2int = trainer.train_bpe_dict(dataset_file, alphabet, vocab_size)
    _render_static_dict(output_file, token2int, token_type)
