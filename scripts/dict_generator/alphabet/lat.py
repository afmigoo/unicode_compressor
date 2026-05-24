from ._punct import _PUNCT_64, _PUNCT_32

LAT_128 = [
    chr(i) for i in range(32, 126 + 1)
    if chr(i).isprintable() and not chr(i).isspace()
] + _PUNCT_32
assert len(LAT_128) <= 127, f"Expected <=127, got {len(LAT_128)}"

LAT_1024 = [
    chr(i) for i in range(32, 591 + 1)
    if chr(i).isprintable() and not chr(i).isspace()
] + _PUNCT_64
assert len(LAT_1024) <= 1023, f"Expected <=1023, got {len(LAT_1024)}"
