from string import punctuation

_PUNCT_8 = list(' \n,.)(:-')
assert len(_PUNCT_8) == 8, f"Expected 8, got {len(_PUNCT_8)}"
_PUNCT_32 = list(' \n,.)(/_:-"*=;{}[]>\'<#?!&\\+%@°^—')
assert len(_PUNCT_32) == 32, f"Expected 32, got {len(_PUNCT_32)}"

_PUNCT_64 = _PUNCT_32 \
    + list(punctuation) \
    + list('0123456789') \
    + list('$—–─¯±~…«»”“·•©№')
_PUNCT_64 = sorted(list(set(_PUNCT_64)))
assert len(_PUNCT_64) <= 64, f"Expected <=64, got {len(_PUNCT_64)}"
print(_PUNCT_64, len(_PUNCT_64))
