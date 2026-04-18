import base64
import zlib

class Base64Encoder:
    def __init__(self, alphabet: str, compress: bool = False):
        self.alphabet = alphabet
        self.compress = compress
        self.char_to_index = {ch: i for i, ch in enumerate(alphabet)}
        self.index_to_char = {i: ch for i, ch in enumerate(alphabet)}

    def encode(self, text: str) -> str:
        """Encode custom-alphabet text into Base64 (printable and copy/paste safe)."""
        try:
            payload = bytes(self.char_to_index[ch] for ch in text)
            if self.compress:
                payload = zlib.compress(payload, level=9)
            # print('payload:', payload)
            # print('payload bytes:', len(payload))
        except KeyError as err:
            raise ValueError(f"Character not in alphabet: {err.args[0]!r}") from None
        return base64.b64encode(payload).decode("ascii")

    def decode(self, encoded: str) -> str:
        """Decode Base64 back to the original custom-alphabet text."""
        payload = base64.b64decode(encoded, validate=True)
        if self.compress:
            payload = zlib.decompress(payload)
        try:
            return "".join(self.index_to_char[b] for b in payload)
        except KeyError as err:
            raise ValueError(f"Byte out of alphabet range: {err.args[0]}") from None


if __name__ == "__main__":
    ALPHABET = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя"
    ALPHABET += "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"
    ALPHABET += "0123456789"
    ALPHABET += "!@#$%^&*()_+-=[]{}|;:,.<>?`~"
    ALPHABET += " "

    encoder = Base64Encoder(ALPHABET)
    inp = input("Enter a string: ")
    encoded = encoder.encode(inp)
    decoded = encoder.decode(encoded)

    print('Bytes utf-8:', len(inp.encode('utf-8')))
    print('Bytes base64:', len(encoded.encode('utf-8')))

    print(f"{inp} -> {encoded} -> {decoded}")
