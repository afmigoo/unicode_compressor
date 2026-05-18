import hashlib
import json
import os
from pathlib import Path

from datasets import load_dataset

token = os.getenv("HUGGINGFACE_TOKEN")
if token:
    print("Token found")
else:
    raise ValueError("Token not found")

N = 100
LANGUAGES = [
    # "python",
    # "go",
    # "rust",
    # "c",
    # "clojure",
    "arduino",
]
output_dir = Path(__file__).parent.parent.parent / "corpus"
output_dir.mkdir(parents=True, exist_ok=True)


def crawl_language(language, split):
    return load_dataset(
        "bigcode/the-stack",
        data_dir=f"data/{language}",
        split=split,
        streaming=True,
        token=token,
    )

def code_row(row: dict) -> dict:
    text = row.get("content") or ""
    return {
        "text": text,
    }


for language in LANGUAGES:
    print(f"Crawling {language}")
    for split in ["train", "test"]:
        ds = crawl_language(language, split)
        with open(output_dir / f"coding_{split}.jsonl", "a", encoding="utf-8") as f:
            for i, row in enumerate(ds):
                print(f"Crawling {language} {split}... [{i + 1}/{N}]")
                if i >= N:
                    break
                out = code_row(row)
                f.write(json.dumps(out, ensure_ascii=False) + "\n")
                f.flush()
        break
    break
