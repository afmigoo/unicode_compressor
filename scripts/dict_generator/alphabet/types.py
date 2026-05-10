from dataclasses import dataclass

@dataclass
class AlphabetVariant:
    lang: str
    alphabet: list[str]
    name: str

    def __str__(self):
        return f'{self.lang}_{self.name}'

    def __repr__(self):
        return str(self)

    def contains(self, char: str) -> bool:
        return char in self.alphabet

@dataclass
class UnrestrictedAlphabet(AlphabetVariant):
    def __init__(self):
        super().__init__(lang='all', alphabet=None, name='all')
    
    def contains(self, char: str) -> bool:
        return True
