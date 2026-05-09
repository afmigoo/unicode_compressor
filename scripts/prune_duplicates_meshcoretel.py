from pathlib import Path
import json

corpus_file = Path(__file__).parent.parent / "corpus" / "ru_meshcoretel.jsonl"

hashes = set()
unique_data = []

n_lines = 0
with open(corpus_file, "r", encoding="utf-8") as f:
    for line in f:
        n_lines += 1
        data = json.loads(line)
        if data["hash"] in hashes:
            continue
        unique_data.append(data)
        hashes.add(data["hash"])

with open(corpus_file, "w", encoding="utf-8") as f:
    for data in unique_data:
        f.write(json.dumps(data, ensure_ascii=False) + "\n")

print(f"Unique {len(unique_data)} lines")
print(f"Pruned {n_lines - len(unique_data)} duplicates")
