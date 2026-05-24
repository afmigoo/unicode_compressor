from string import punctuation
from ._punct import _PUNCT_64

RU_32 = [
  'а', 'б', 'в', 'г', 'д', 'е', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н', 'о', 'п',
  'р', 'с', 'т', 'у', 'х', 'ц', 'ч', 'ш', 'щ', 'ы', 'ь', 'э', 'ю', 'я', ' '
]; assert len(RU_32) == 31, f"Expected 31, got {len(RU_32)}"

RU_PUNCT_64 = RU_32 + [
  'ё', '0', '1',  '2', '3', '4', '5', '6', '7', '8', '9', '0', '.', ',', '!', '?',
  ':', '%', '\n', '+', '-', '=', '@', '[', ']', '(', ')', '<', '>', '_', '#', '*'
]; assert len(RU_PUNCT_64) == 63, f"Expected 63, got {len(RU_PUNCT_64)}"

RU_ALPHA_64 = RU_32 + [
  'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О', 'П',
  'Р', 'С', 'Т', 'У', 'ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ы', 'Ь', 'Э', 'Ю', 'Я', '\n'
]; assert len(RU_ALPHA_64) == 63, f"Expected 63, got {len(RU_ALPHA_64)}"

RU_128 = 'абвгдеёжзийклмнопрстуфхцчшщъыьэюяaeëopcyx'
RU_128 += RU_128.upper()
RU_128 += ' \n'
RU_128 += '0123456789'
RU_128 += punctuation
RU_128 = list(RU_128)
assert len(RU_128) <= 127, f"Expected <=127, got {len(RU_128)}"
assert len(set(RU_128)) == len(RU_128), f"Expected unique"

RU_256 = RU_128
RU_256 += list(ch for ch in 'abcdefghijklmnopqrstuvwxyz' if ch not in RU_128)
RU_256 += list(ch for ch in 'ABCDEFGHIJKLMNOPQRSTUVWXYZ' if ch not in RU_128)
RU_256 += [ch for ch in _PUNCT_64 if ch not in RU_128]
assert len(RU_256) <= 255, f"Expected <=255, got {len(RU_256)}"
assert len(set(RU_256)) == len(RU_256), f"Expected unique"
