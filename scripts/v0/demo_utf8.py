
class UTF8Encoder:
    PRINTABLE_RANGES = [(32, 126), (161, 255)]
    MAX_CHARS = sum(t[1] - t[0] + 1 for t in PRINTABLE_RANGES)
    ALPH_FREQ = {
        'cyrillic': {
            'О': 11.18,
            'Е': 8.75,
            'А': 7.64,
            'И': 7.09,
            'Н': 6.78,
            'Т': 6.09,
            'С': 4.97,
            'Л': 4.96,
            'В': 4.38,
            'Р': 4.23,
            'К': 3.30,
            'М': 3.17,
            'Д': 3.09,
            'П': 2.47,
            'Ы': 2.36,
            'У': 2.22,
            'Б': 2.01,
            'Я': 1.96,
            'Ь': 1.84,
            'Г': 1.72,
            'З': 1.48,
            'Ч': 1.40,
            'Й': 1.21,
            'Ж': 1.01,
            'Х': 0.95,
            'Ш': 0.72,
            'Ю': 0.47,
            'Ц': 0.39,
            'Э': 0.36,
            'Щ': 0.30,
            'Ф': 0.21,
            'Ё': 0.20,
            'Ъ': 0.02,
        },
        'english': {
            'E': 12.02,
            'T': 9.10,
            'A': 8.12,
            'O': 7.68,
            'I': 7.31,
            'N': 6.95,
            'S': 6.28,
            'R': 6.02,
            'H': 5.92,
            'D': 4.32,
            'L': 3.98,
            'U': 2.88,
            'C': 2.71,
            'M': 2.61,
            'F': 2.30,
            'Y': 2.11,
            'W': 2.09,
            'G': 2.03,
            'P': 1.82,
            'B': 1.49,
            'V': 1.11,
            'K': 0.69,
            'X': 0.17,
            'Q': 0.11,
            'J': 0.10,
            'Z': 0.07,
        }
    }

    def __init__(self, alphabet: str, optimize_preference: list[str] = ['cyrillic', 'english']):
        if optimize_preference:
            alphabet = UTF8Encoder.optimize_alphabet(alphabet, optimize_preference)
        self.alphabet = alphabet
        self.char_to_index = UTF8Encoder.map_alphabet(alphabet)
        self.index_to_char = {v: k for k, v in self.char_to_index.items()}

    def encode(self, text: str) -> str:
        for ch in text:
            if ch not in self.char_to_index:
                raise ValueError(f"Character {ch} not in alphabet")
        return ''.join(self.char_to_index[ch] for ch in text)

    def decode(self, text: str) -> str:
        return ''.join(self.index_to_char[ch] for ch in text)

    @classmethod
    def map_alphabet(cls, alphabet: str) -> dict[str, str]:
        """
        1-byte unicode has MAX_CHARS printable characters (32 - 126, 161 - 255)
        This function maps target alphabet to them
        """
        if len(alphabet) > cls.MAX_CHARS:
            raise ValueError(f"Alphabet is bigger than {cls.MAX_CHARS} characters")

        _ranges = [[t[0], t[1]] for t in cls.PRINTABLE_RANGES]

        mapping = {}
        for ch in alphabet:
            # mapping[ch] = f'{chr(_ranges[0][0])} ({_ranges[0][0]})'
            mapping[ch] = chr(_ranges[0][0])
            _ranges[0][0] += 1
            if _ranges[0][0] > _ranges[0][1]:
                _ranges.pop(0)
        return mapping

    @classmethod
    def optimize_alphabet(cls, alphabet: str, optimize_preference: list[str]) -> str:
        """
        Optimize alphabet by sorting it by frequency of characters
        """
        _weights = {}
        for i, lang in enumerate(optimize_preference):
            k = pow(10000, -i)
            _weights.update({ch: w * k for ch, w in cls.ALPH_FREQ[lang].items()})
            # lowercase letters with weight 100 times higher
            _weights.update({ch.lower(): w * 100 * k for ch, w in cls.ALPH_FREQ[lang].items()})
        
        optimized_alphabet = ''.join(sorted(alphabet, key=lambda x: _weights[x] if x in _weights else 0, reverse=True))
        # print(f"Original alphabet: {alphabet}")
        # print(f"Optimized alphabet: {optimized_alphabet}")
        return optimized_alphabet
