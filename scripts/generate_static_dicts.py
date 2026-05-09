from pathlib import Path

from dict_generator import render
from dict_generator.types import MapEncoderVariant, TokenEncoderVariant
from dict_generator.variants import get_encoder_variants

if __name__ == '__main__':
    corpus_dir = Path(__file__).parent.parent / 'corpus'
    output_dir = Path(__file__).parent.parent / 'rust/src/encoders/instances'

    encoder_variants = get_encoder_variants(split='train')
    map_encoder_variants = [v for v in encoder_variants if isinstance(v, MapEncoderVariant)]
    token_encoder_variants = [v for v in encoder_variants if isinstance(v, TokenEncoderVariant)]

    #### utf-8 dicts ####
    # map dicts
    for encoder_variant in map_encoder_variants:
        name = str(encoder_variant)
        output_file = output_dir / f'{name}.rs'
        print(f"Generating {name}...")
        render.render_map_dict(output_file, encoder_variant.alphabet, encoder_variant.token_type)

    # token-to-unicode dicts
    for encoder_variant in token_encoder_variants:
        name = str(encoder_variant)
        output_file = output_dir / f'{name}.rs'
        print(f"Generating {name}...")
        render.render_bpe_dict(
          output_file=output_file,
          dataset_file=encoder_variant.dataset,
          alphabet=encoder_variant.alphabet,
          vocab_size=encoder_variant.vocab_size,
          token_type=encoder_variant.token_type
        )
