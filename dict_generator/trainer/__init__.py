from pathlib import Path
from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.trainers import BpeTrainer

from dict_generator import dataset

def train_bpe_dict(file: str | Path, alphabet: list[str], vocab_size: int) -> dict[str, int]:
  trainer = BpeTrainer(
    vocab_size=vocab_size,
    show_progress=False
  )
  tokenizer = Tokenizer(BPE())
  ds = dataset.alphabet_filtered(file, alphabet)

  tokenizer.train_from_iterator(ds, trainer)
  token2int = dict(sorted(tokenizer.get_vocab().items(), key=lambda item: item[1]))
  return token2int

