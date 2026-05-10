from pathlib import Path

from dict_generator import render
from dict_generator.variants import get_dict_variants
from dict_generator.alphabet.types import UnrestrictedAlphabet

if __name__ == '__main__':
    corpus_dir = Path(__file__).parent.parent / 'corpus'
    output_dir = Path(__file__).parent.parent / 'rust/src/encoders/dictionaries'

    dict_variants = get_dict_variants()

    #### utf-8 dicts ####
    for dict_variant in dict_variants:
        if isinstance(dict_variant.alphabet, UnrestrictedAlphabet):
            continue
        name = str(dict_variant)
        output_file = output_dir / f'{name}.rs'
        print(f"Generating {name}...")
        dict_variant.render(output_file)
