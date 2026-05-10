from pathlib import Path
from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.trainers import BpeTrainer

from dict_generator.dataset import Dataset
from dict_generator.alphabet.types import AlphabetVariant

def train_bpe_dict(dataset: Dataset, alphabet: AlphabetVariant, vocab_size: int) -> dict[str, int]:
  trainer = BpeTrainer(
    vocab_size=vocab_size,
    show_progress=False,
    max_token_length=16,
    special_tokens=[
      '<end-of-stream>'
    ]
  )
  tokenizer = Tokenizer(BPE())

  tokenizer.train_from_iterator(dataset.alphabet_filtered('train', alphabet), trainer)
  token2int = dict(sorted(tokenizer.get_vocab().items(), key=lambda item: item[1]))
  del token2int['<end-of-stream>']
  return token2int
