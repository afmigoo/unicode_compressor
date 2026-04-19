from tokenizers import Tokenizer
from tokenizers.models import BPE
from tokenizers.trainers import BpeTrainer

from encoders.StaticDictEncoder import StaticDictEncoder
from alphabet import ALPHABET
from get_printable_utf8 import get_printable_utf8
from pathlib import Path

class BpeEncoder(StaticDictEncoder):
    def __init__(self, alphabet: str, train_files: list[Path|str] = None):
        if not train_files:
            raise ValueError("train_files is required")
        self.__train_files = [str(file) for file in train_files]
        super().__init__(alphabet)

    def encode(self, text: str) -> str:
        return super().encode(text)

    def decode(self, encoded: str) -> str:
        return super().decode(encoded)

    def compute_dict(self, alphabet: str) -> dict[str, str]:
        """
        Compute a dictionary mapping from tokens to ids
        """
        tokenizer = Tokenizer(BPE())
        printable_utf8 = get_printable_utf8(1) + get_printable_utf8(2)
        trainer = BpeTrainer(
            vocab_size=len(printable_utf8),
            initial_alphabet=list(ALPHABET),
            show_progress=False
        )
        tokenizer.train(files=self.__train_files, trainer=trainer)
        token2int = dict(sorted(tokenizer.get_vocab().items(), key=lambda item: item[1]))
        token2unicode = {token: printable_utf8[idx] for token, idx in token2int.items()}
        return token2unicode
