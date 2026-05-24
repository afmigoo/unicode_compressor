from string import punctuation

_PUNCT_8 = list(' \n,.)(:-')
assert len(_PUNCT_8) == 8, f"Expected 8, got {len(_PUNCT_8)}"
_PUNCT_32 = list(' \n,.)(/_:-"*=;{}[]>\'<#?!&\\+%@°^—')
assert len(_PUNCT_32) == 32, f"Expected 32, got {len(_PUNCT_32)}"

_PUNCT_128 = _PUNCT_32 \
    + list(punctuation) \
    + list('0123456789') \
    + list('$–―‒ー─－━−‐‑¯±~…«»”“·•©№')
_PUNCT_128 = sorted(list(set(_PUNCT_128)))
assert len(_PUNCT_128) <= 128, f"Expected <=128, got {len(_PUNCT_128)}"
