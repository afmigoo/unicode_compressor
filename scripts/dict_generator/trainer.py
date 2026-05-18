from pathlib import Path
from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.trainers import BpeTrainer
from tokenizers.pre_tokenizers import FixedLength

from dict_generator.dataset import Dataset
from dict_generator.alphabet.types import AlphabetVariant
from dict_generator.alphabet.types import UnrestrictedAlphabet

def train_bpe_dict(dataset: Dataset, alphabet: AlphabetVariant, vocab_size: int) -> dict[str, int]:
  if isinstance(alphabet, UnrestrictedAlphabet):
    initial_alphabet = ['\n', ' ']
  else:
    initial_alphabet = alphabet.alphabet

  trainer = BpeTrainer(
    vocab_size=vocab_size,
    show_progress=True,
    max_token_length=16,
    initial_alphabet=initial_alphabet,
    special_tokens=[
      '<end-of-stream>'
    ]
  )
  tokenizer = Tokenizer(BPE())
  tokenizer.pre_tokenizer = FixedLength(length=256)

  tokenizer.train_from_iterator(dataset.alphabet_filtered('train', alphabet), trainer)
  token2int = dict(sorted(tokenizer.get_vocab().items(), key=lambda item: item[1]))
  if len(token2int) != vocab_size:
    raise RuntimeError(f"Dataset did not fit into the vocabulary size. Expected {vocab_size}, got {len(token2int)}")
  del token2int['<end-of-stream>']
  return token2int
