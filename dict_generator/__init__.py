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

def render_bpe_dict(
    output_file: str | Path,
    dataset_file: str | Path,
    alphabet: list[str],
    vocab_size: int,
    token_type: Literal['u16', 'utf8'],
):
    if isinstance(dataset_file, str):
        dataset_file = Path(dataset_file)
    if not token_type in ['u16', 'utf8']:
        raise ValueError(f"Invalid token type: {token_type}")

    token2int = trainer.train_bpe_dict(dataset_file, alphabet, vocab_size)
    if token_type == 'u16':
        static_dict = {k: f'{v}u16' for k, v in token2int.items()}
    elif token_type == 'utf8':
        utf8_chars = get_printable_utf8(1) + get_printable_utf8(2)
        if vocab_size > len(utf8_chars):
            raise ValueError(f"Vocab size is larger than the number of printable UTF-8 characters: {vocab_size}")
        static_dict = {k: utf8_chars[v] for k, v in token2int.items()}

    rustgen.write(static_dict, output_file)
