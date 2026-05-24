from ._punct import _PUNCT_64

CYR_512 = [
    chr(i) for i in range(1024, 1327 + 1)
    if chr(i).isprintable() and not chr(i).isspace()
] + _PUNCT_64
assert len(CYR_512) <= 511, f"Expected <=511, got {len(CYR_512)}"
