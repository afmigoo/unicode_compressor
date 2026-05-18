from multiprocessing import Pool
from pathlib import Path

from dict_generator import write
from dict_generator.types import DictVariant
from dict_generator.variants import get_dict_variants, MAX_VOCAB_SIZE
from dict_generator import get_printable_utf8

_REPO_ROOT = Path(__file__).parent.parent
corpus_dir = _REPO_ROOT / 'corpus'
output_dir = _REPO_ROOT / 'rust/src/encoders/dictionaries'
mod_file = _REPO_ROOT / 'rust/src/encoders/dictionaries.rs'


def generate_one_variant(dict_variant: DictVariant) -> tuple[str, str | None]:
    name = str(dict_variant)
    output_file = output_dir / f'{name}.rs'
    print(f"Generating {name}...")
    try:
        dict_variant.render(output_file)
        return name, None
    except RuntimeError as e:
        return name, str(e)


if __name__ == '__main__':
    dict_variants = get_dict_variants()

    # clear the module file
    with open(mod_file, 'w', encoding='utf-8') as f:
        pass

    #### utf-8 dicts ####
    with Pool(processes=1) as pool:
        results = pool.map(generate_one_variant, dict_variants)

    mod_lines: list[str] = []
    for name, err in results:
        if err is not None:
            print(f"Error generating {name}: {err}")
        else:
            mod_lines.append(f"pub mod {name};\n")
    with open(mod_file, 'a', encoding='utf-8') as f:
        f.writelines(mod_lines)

    #### int2utf dicts ####
    usable_chars = get_printable_utf8(1) + get_printable_utf8(2) + get_printable_utf8(3)
    usable_chars = usable_chars[:MAX_VOCAB_SIZE]
    char2int = {ch: i + 1 for i, ch in enumerate(usable_chars)}
    output_file = output_dir / 'plain_map.rs'
    print(f"Generating int2utf...")
    write.write_rust_char_dict(char2int, output_file)
    with open(mod_file, 'a', encoding='utf-8') as f:
        f.write(f"pub mod plain_map;\n")
