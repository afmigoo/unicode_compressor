import base64
import zlib

class Base85Encoder:
    def __init__(self, compress: bool = False):
        self.compress = compress

    def encode(self, text: str) -> str:
        payload = text.encode('utf-8')
        if self.compress:
            payload = zlib.compress(payload, level=9)
        return base64.b85encode(payload).decode('utf-8')

    def decode(self, encoded: str) -> str:
        payload = base64.b85decode(encoded)
        if self.compress:
            payload = zlib.decompress(payload)
        return payload.decode('utf-8')
