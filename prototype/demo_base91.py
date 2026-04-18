import base91
import zlib

class Base91Encoder:
    def __init__(self, compress: bool = False):
        self.compress = compress

    def encode(self, text: str) -> str:
        payload = text.encode('utf-8')
        if self.compress:
            payload = zlib.compress(payload, level=9)
        return base91.encode(payload)

    def decode(self, encoded: str) -> str:
        payload = base91.decode(encoded)
        if self.compress:
            payload = zlib.decompress(payload)
        return payload.decode('utf-8')
