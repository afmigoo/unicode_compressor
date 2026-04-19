from datasets import load_dataset
from pathlib import Path
import os

token = os.getenv("HUGGINGFACE_TOKEN")
N = 100
LANGUAGES = [
    # "python",
    # "go",
    "rust",
    # "c",
    # "clojure",
    # "arduino"
]

def crawl_language(language, split):
    return load_dataset(
        "bigcode/the-stack",
        data_dir=f"data/{language}",
        split=split,
        streaming=True,
        token=token
    )

for language in LANGUAGES:
    for split in ["train", "validation"]:
        output_dir = Path(__file__).parent.parent / "corpus/raw/coding" / language
        output_dir.mkdir(exist_ok=True, parents=True)
        ds = crawl_language(language, split)
        with open(output_dir / f"{split}.txt", "w") as f:
            for i, row in enumerate(ds):
                print(f"Crawling {language} {split}... [{i + 1}/{N}]" )
                if i >= N:
                    break
                print(f"[{i + 1}/{N}]" )
                f.write(row["content"] + "\n")
                f.flush()
        break
