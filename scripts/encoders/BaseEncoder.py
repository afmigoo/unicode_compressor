
class BaseEncoder:
    def __init__(self, alphabet: str):
        self.__alphabet = alphabet

    def encode(self, text: str) -> str:
        raise NotImplementedError

    def decode(self, encoded: str) -> str:
        raise NotImplementedError

    @property
    def alphabet(self) -> str:
        return self.__alphabet
