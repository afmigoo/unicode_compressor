from pathlib import Path

from dict_generator import render
from dict_generator.types import MapEncoderVariant, TokenEncoderVariant
from dict_generator.variants import get_encoder_variants

if __name__ == '__main__':
    corpus_dir = Path(__file__).parent.parent / 'corpus'
    output_dir = Path(__file__).parent.parent / 'rust/src/encoders/instances'

    encoder_variants = get_encoder_variants(split='train')

    #### utf-8 dicts ####
    for encoder_variant in encoder_variants:
        name = str(encoder_variant)
        output_file = output_dir / f'{name}.rs'
        print(f"Generating {name}...")
        encoder_variant.render(output_file)
