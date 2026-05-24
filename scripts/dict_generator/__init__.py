from .variants import get_encoder_variants
from .rustgen import write
from .dataset import Dataset

def get_printable_utf8(size: int) -> list[str]:
    result = []
    for i in range(pow(2, 8 * size)):
        try:
            ch = chr(i)
        except ValueError:
            break
        if ch.isprintable() and not ch.isspace() and len(ch.encode('utf-8')) == size:
            result.append(ch)
    return result
