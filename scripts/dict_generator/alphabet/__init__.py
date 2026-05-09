from .ru import *
from .en import *

ALPHABETS = [
    # 'all' is a special case for when no alphabet is specified
    # it is used to train the token dictionaries with no filtering
    {'lang': 'all', 'name': 'all', 'alphabet': None, 'token_type': ['utf8', 'bin']},
    {'lang': 'ru', 'name': '32', 'alphabet': RU_32, 'token_type': ['bin']},
    {'lang': 'ru', 'name': 'punct_64', 'alphabet': RU_PUNCT_64, 'token_type': ['bin']},
    {'lang': 'ru', 'name': 'alpha_64', 'alphabet': RU_ALPHA_64, 'token_type': ['bin']},
    {'lang': 'ru', 'name': '128', 'alphabet': RU_128, 'token_type': ['utf8', 'bin']},
    {'lang': 'en', 'name': '32', 'alphabet': EN_32, 'token_type': ['bin']},
    {'lang': 'en', 'name': 'punct_64', 'alphabet': EN_PUNCT_64, 'token_type': ['bin']},
    {'lang': 'en', 'name': 'alpha_64', 'alphabet': EN_ALPHA_64, 'token_type': ['bin']},
    {'lang': 'en', 'name': '128', 'alphabet': EN_128, 'token_type': ['utf8', 'bin']},
]
