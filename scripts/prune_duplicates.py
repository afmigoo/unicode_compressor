#!/usr/bin/env python3
"""
Prunes duplicate rows from meshcoretel and meshtastic corpus JSONL files.

- corpus/ru_meshcoretel.jsonl — uniqueness by ``hash``
- corpus/meshtastic/_all.jsonl — uniqueness by ``packet_id``

Run before make_meshtastic_datasets.py. Bucket files are not pruned here.

usage: python3 scripts/prune_duplicates.py
"""

from __future__ import annotations

import json
from pathlib import Path

CORPUS_DIR = Path(__file__).parent.parent / "corpus"
MESHCORETEL_FILE = CORPUS_DIR / "ru_meshcoretel.jsonl"
MESHTASTIC_ALL_FILE = CORPUS_DIR / "meshtastic" / "_all.jsonl"


def prune_jsonl(path: Path, key: str) -> tuple[int, int, int]:
  """Dedupe in place (first row wins). Returns (lines_in, lines_out, empty_keys)."""
  seen: set[str] = set()
  unique: list[dict] = []
  empty_keys = 0
  lines_in = 0

  with open(path, encoding="utf-8") as f:
    for line in f:
      line = line.strip()
      if not line:
        continue
      lines_in += 1
      row = json.loads(line)
      value = row.get(key) or ""
      if not value:
        empty_keys += 1
      if value in seen:
        continue
      seen.add(value)
      unique.append(row)

  with open(path, "w", encoding="utf-8") as f:
    for row in unique:
      f.write(json.dumps(row, ensure_ascii=False) + "\n")

  return lines_in, len(unique), empty_keys


def main() -> None:
  if not MESHCORETEL_FILE.is_file():
    raise FileNotFoundError(f"Missing corpus file: {MESHCORETEL_FILE}")

  lines_in, lines_out, empty_keys = prune_jsonl(MESHCORETEL_FILE, "hash")
  pruned = lines_in - lines_out
  print(f"{MESHCORETEL_FILE.name}: kept {lines_out}, pruned {pruned}", end="")
  if empty_keys:
    print(f" ({empty_keys} rows with empty hash)", end="")
  print()

  if not MESHTASTIC_ALL_FILE.is_file():
    raise FileNotFoundError(f"Missing corpus file: {MESHTASTIC_ALL_FILE}")

  lines_in, lines_out, empty_keys = prune_jsonl(MESHTASTIC_ALL_FILE, "packet_id")
  pruned = lines_in - lines_out
  print(f"{MESHTASTIC_ALL_FILE.relative_to(CORPUS_DIR.parent)}: kept {lines_out}, pruned {pruned}", end="")
  if empty_keys:
    print(f" ({empty_keys} rows with empty packet_id)", end="")
  print()


if __name__ == "__main__":
  main()
