from .ru import *
from .en import *
from .types import *

ALPHABETS: list[AlphabetVariant] = [
    # 'all' is a special case for when no alphabet is specified
    # it is used to train the token dictionaries with no filtering
    # UnrestrictedAlphabet(),
    AlphabetVariant(lang='ru', name='32', alphabet=RU_32),
    # AlphabetVariant(lang='ru', name='punct_64', alphabet=RU_PUNCT_64),
    # AlphabetVariant(lang='ru', name='alpha_64', alphabet=RU_ALPHA_64),
    # AlphabetVariant(lang='ru', name='128', alphabet=RU_128),
    AlphabetVariant(lang='ru', name='256', alphabet=RU_256),
    AlphabetVariant(lang='en', name='32', alphabet=EN_32),
    # AlphabetVariant(lang='en', name='punct_64', alphabet=EN_PUNCT_64),
    # AlphabetVariant(lang='en', name='alpha_64', alphabet=EN_ALPHA_64),
    AlphabetVariant(lang='en', name='128', alphabet=EN_128),
]
