from pathlib import Path
from random import choices, seed
from tqdm import tqdm

from dict_generator.variants import get_encoder_variants
from dict_generator.alphabet.types import UnrestrictedAlphabet
from eval import encode, decode

seed(1337)
UNIPRESS_BINARY = Path(__file__).parent.parent / 'rust/target/release/unipress'

if __name__ == '__main__':
    encoder_variants = get_encoder_variants()
    lengths = list(range(0, 10 ** 3 + 1)) + [10 ** x for x in range(4, 7)]

    #### utf-8 dicts ####
    for encoder_variant in encoder_variants:
        if isinstance(encoder_variant.dict.alphabet, UnrestrictedAlphabet):
            continue
        for payload_len in tqdm(lengths, desc=f"{encoder_variant}"):
            payload = ''.join(choices(encoder_variant.dict.alphabet.alphabet, k=payload_len))
            encoded = encode(payload, str(encoder_variant)).encoded_payload
            decoded = decode(encoded, str(encoder_variant)).decoded_payload
            assert decoded == payload, \
                f"Roundtrip failed: \"{payload}\" != \"{decoded}\""
