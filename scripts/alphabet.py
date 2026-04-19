from string import punctuation

__RU_LOWER = "абвгдеёжзийклмнопрстуфхцчшщъыьэюя"
__EN_LOWER = "abcdefghijklmnopqrstuvwxyz"

ALPHABET = " \n"
ALPHABET += punctuation
ALPHABET += "0123456789"
ALPHABET += __RU_LOWER
ALPHABET += __RU_LOWER.upper()
ALPHABET += __EN_LOWER
ALPHABET += __EN_LOWER.upper()

ALPHABET_MAP = set(ALPHABET)

