from string import punctuation

EN_32 = [
  'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
  'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z', ' ', "'", '?', '.', ','
]; assert len(EN_32) == 31, f"Expected 31, got {len(EN_32)}"

EN_PUNCT_64 = EN_32 + [
  '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '\n', '"', '&', '/', '\\',
  ':', '%', '`', '+', '-', '=', '@', '[', ']', '(', ')', '<',  '>', '_', '#', '*'
]; assert len(EN_PUNCT_64) == 63, f"Expected 63, got {len(EN_PUNCT_64)}"

EN_ALPHA_64 = EN_32 + [
  'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K',  'L', 'M', 'N', 'O', 'P',
  'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '\n', '-', '=', '+', '/', '\\'
]; assert len(EN_ALPHA_64) == 63, f"Expected 63, got {len(EN_ALPHA_64)}"

EN_128 = 'abcdefghijklmnopqrstuvwxyz'
EN_128 += EN_128.upper()
EN_128 += ' \n'
EN_128 += '0123456789'
EN_128 += punctuation
EN_128 = list(EN_128)
assert len(EN_128) <= 128, f"Expected <=128, got {len(EN_128)}"
assert len(set(EN_128)) == len(EN_128), f"Expected unique"
