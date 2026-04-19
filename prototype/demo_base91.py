import base91
import zlib

class Base91Encoder:
    def __init__(self, alphabet: str, compress: bool = False):
        self.alphabet = alphabet
        self.char_to_index = {ch: i for i, ch in enumerate(alphabet)}
        self.index_to_char = {i: ch for i, ch in enumerate(alphabet)}
        self.compress = compress

    def encode(self, text: str) -> str:
        try:
            payload = bytes(self.char_to_index[ch] for ch in text)
            if self.compress:
                payload = zlib.compress(payload, level=9)
        except KeyError as err:
            raise ValueError(f"Character not in alphabet: {err.args[0]!r}") from None
        return base91.encode(payload)

    def decode(self, encoded: str) -> str:
        payload = base91.decode(encoded)
        if self.compress:
            payload = zlib.decompress(payload)
        try:
            return "".join(self.index_to_char[b] for b in payload)
        except KeyError as err:
            raise ValueError(f"Byte out of alphabet range: {err.args[0]}") from None
