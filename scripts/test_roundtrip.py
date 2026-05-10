from pathlib import Path
from random import choices, seed
from tqdm import tqdm

from dict_generator.variants import get_encoder_variants
from eval import encode, decode

seed(1337)
UNIPRESS_BINARY = Path(__file__).parent.parent / 'rust/target/release/unipress'

if __name__ == '__main__':
    encoder_variants = get_encoder_variants(split='train')
    lengths = [10 ** x for x in range(4, 7)] + list(range(0, 10 ** 3 + 1))

    #### utf-8 dicts ####
    for encoder_variant in encoder_variants:
        if encoder_variant.alphabet is None:
            continue
        if str(encoder_variant) != 'ru_32_bin_map':
            continue
        for payload_len in tqdm(lengths, desc=f"{encoder_variant}"):
            payload = ''.join(choices(encoder_variant.alphabet, k=payload_len))
            encoded = encode(payload, str(encoder_variant)).encoded_payload
            decoded = decode(encoded, str(encoder_variant)).decoded_payload
            assert decoded == payload, f"Roundtrip failed: {payload} -> {encoded} -> {decoded}"
