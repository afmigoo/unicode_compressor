from pathlib import Path
import re

from alphabet import ALPHABET_MAP

def filter_alphabet(text: str) -> str:
    return ''.join(ch for ch in text if ch in ALPHABET_MAP)

def collapse_whitespace(text: str) -> str:
    stripped_text = '\n'.join([x.strip() for x in text.split('\n')])
    collapsed_spaces = re.sub(r' +', ' ', stripped_text)
    collapsed_newlines = re.sub(r'\n+', '\n', collapsed_spaces)
    return collapsed_newlines

def remove_mentions(text: str) -> str:
    return re.sub(r'@\[[^\]]+\]', '', text)

if __name__ == "__main__":
    raw_dir = Path(__file__).parent / "corpus/raw"
    processed_dir = Path(__file__).parent / "corpus/processed"
    processed_dir.mkdir(exist_ok=True)

    for walked_dir, dirs, files in raw_dir.walk():
        for raw_file_name in files:
            raw_file = Path(walked_dir) / raw_file_name
            processed_file = processed_dir / raw_file.relative_to(raw_dir)
            processed_file.parent.mkdir(exist_ok=True, parents=True)

            processed_text = raw_file.read_text()
            if 'meshcoretel' in walked_dir.name:
                processed_text = remove_mentions(processed_text)
            processed_text = filter_alphabet(processed_text)
            if not 'coding' in str(walked_dir):
                processed_text = collapse_whitespace(processed_text)
            processed_file.write_text(processed_text)
