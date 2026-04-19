from collections import Counter

from encoders.BaseEncoder import BaseEncoder

class StaticDictEncoder(BaseEncoder):
    def __init__(self, alphabet: str):
        super().__init__(alphabet)
        self.__token2unicode = self.compute_dict(alphabet)
        self.__unicode2token = {unicode: token for token, unicode in self.__token2unicode.items()}

    def encode(self, text: str, full_search: bool = False) -> str:
        if full_search:
            return self._encode_full_search(text)
        return self._encode_greedy(text)

    def _encode_greedy(self, text: str) -> str:
        max_len = max(len(token) for token in self.__token2unicode.keys())
        
        def _recurse(tokenized: list[str], untokenized: str) -> str:
            if untokenized in self.__token2unicode:
                return ''.join(tokenized) + self.__token2unicode[untokenized]

            for token_size in range(min(max_len, len(untokenized)), 0, -1):
                token = untokenized[:token_size]
                if token in self.__token2unicode:
                    return _recurse(tokenized + [self.__token2unicode[token]], untokenized[token_size:])
            
            raise RuntimeError(f"Could not encode {untokenized[:max_len]}")

        return _recurse(list(), text)

    def _encode_full_search(self, text: str) -> str:
        max_len = max(len(token) for token in self.__token2unicode.keys())
        
        def _recurse(tokenized: list[str], untokenized: str) -> str:
            if untokenized in self.__token2unicode:
                return ''.join(tokenized) + self.__token2unicode[untokenized]

            shortest_res = None
            for token_size in range(min(max_len, len(untokenized)), 0, -1):
                token = untokenized[:token_size]
                if token in self.__token2unicode:
                    rtrn = _recurse(tokenized + [self.__token2unicode[token]], untokenized[token_size:])
                    shortest_res = rtrn if shortest_res is None or len(rtrn) < len(shortest_res) else shortest_res
            return shortest_res

        return _recurse(list(), text)

    def decode(self, encoded: str) -> str:
        return ''.join(self.__unicode2token[ch] for ch in encoded)

    def compute_dict(self, alphabet: str) -> dict[str, str]:
        """
        Compute a dictionary mapping from tokens to ids
        """
        raise NotImplementedError

    @property
    def token2unicode(self) -> dict[str, str]:
        return self.__token2unicode

    @property
    def unicode2token(self) -> dict[str, str]:
        return self.__unicode2token

    @property
    def T(self) -> int:
        return self.__T
