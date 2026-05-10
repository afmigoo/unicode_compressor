from pathlib import Path

from dict_generator import write
from dict_generator.variants import get_dict_variants
from dict_generator.alphabet.types import UnrestrictedAlphabet
from dict_generator import get_printable_utf8

if __name__ == '__main__':
    corpus_dir = Path(__file__).parent.parent / 'corpus'
    output_dir = Path(__file__).parent.parent / 'rust/src/encoders/dictionaries'
    mod_file = Path(__file__).parent.parent / 'rust/src/encoders/dictionaries.rs'

    dict_variants = get_dict_variants()

    # clear the module file
    with open(mod_file, 'w', encoding='utf-8') as f:
        pass
    
    #### utf-8 dicts ####
    for dict_variant in dict_variants:
        name = str(dict_variant)
        output_file = output_dir / f'{name}.rs'
        print(f"Generating {name}...")
        try:
            dict_variant.render(output_file)
            with open(mod_file, 'a', encoding='utf-8') as f:
                f.write(f"pub mod {name};\n")
        except RuntimeError as e:
            print(f"Error generating {name}: {e}")
            continue

    #### int2utf dicts ####
    usable_chars = get_printable_utf8(1) + get_printable_utf8(2) + get_printable_utf8(3)
    usable_chars = usable_chars[:2048]
    char2int = {ch: i + 1 for i, ch in enumerate(usable_chars)}
    output_file = output_dir / 'plain_map.rs'
    print(f"Generating int2utf...")
    write.write_rust_char_dict(char2int, output_file)
    with open(mod_file, 'a', encoding='utf-8') as f:
        f.write(f"pub mod plain_map;\n")
