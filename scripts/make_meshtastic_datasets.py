#!/usr/bin/env python3
"""
Split corpus/meshtastic/_all.jsonl into per-category bucket JSONL files.

Reads fasttext_prediction from the consolidated crawl file and writes
{text, date, packet_id} rows to corpus/meshtastic/{category}_{lang|script}_meshtastic.jsonl.

Run after crawl and prune:

  python3 scripts/crawl/meshtastic_map.py
  python3 scripts/prune_duplicates.py
  python3 scripts/make_meshtastic_datasets.py --categorize-by lang
  python3 scripts/make_meshtastic_datasets.py --categorize-by script
  python3 scripts/make_meshtastic_datasets.py --categorize-by lang --min-messages 100
"""

from __future__ import annotations

import argparse
import json
from collections import defaultdict
from pathlib import Path
from typing import Literal, TextIO

REPO_ROOT = Path(__file__).parent.parent
DEFAULT_INPUT = REPO_ROOT / "corpus" / "meshtastic" / "_all.jsonl"
DEFAULT_OUTPUT_DIR = REPO_ROOT / "corpus" / "meshtastic"
DEFAULT_MIN_MESSAGES = 100


def category_from_prediction(
  prediction: str,
  categorize_by: Literal["lang", "script"],
) -> str:
  if "_" in prediction:
    lang, script = prediction.split("_", 1)
  else:
    lang, script = "unknown", "unknown"
  return lang if categorize_by == "lang" else script


def bucket_row(row: dict) -> dict:
  return {
    "text": row["text"],
    "date": row["date"],
    "packet_id": row["packet_id"],
  }

def filter_text(text: str) -> str:
  n_alpha_chars = sum(1 for c in text if c.isalpha())
  if n_alpha_chars / len(text) < 0.5: # at least 50% of the text must be alphabetic
    return ''
  return text

def _count_categories(
  input_path: Path,
  categorize_by: Literal["lang", "script"],
) -> dict[str, int]:
  counts: dict[str, int] = defaultdict(int)
  with open(input_path, encoding="utf-8") as f:
    for line in f:
      line = filter_text(line.strip())
      if not line:
        continue
      row = json.loads(line)
      category = category_from_prediction(
        row.get("fasttext_prediction") or "unknown_unknown",
        categorize_by,
      )
      counts[category] += 1
  return dict(counts)


def make_datasets(
  input_path: Path,
  output_dir: Path,
  categorize_by: Literal["lang", "script"],
  min_messages: int,
) -> tuple[dict[str, int], dict[str, int]]:
  """Write bucket files. Returns (written_counts, skipped_counts)."""
  output_dir.mkdir(parents=True, exist_ok=True)
  for path in output_dir.glob(f"*_{categorize_by}_meshtastic.jsonl"):
    path.unlink()

  all_counts = _count_categories(input_path, categorize_by)
  qualifying = {c for c, n in all_counts.items() if n >= min_messages}
  skipped = {c: n for c, n in all_counts.items() if c not in qualifying}

  handles: dict[str, TextIO] = {}
  written: dict[str, int] = defaultdict(int)

  try:
    with open(input_path, encoding="utf-8") as f:
      for line in f:
        line = filter_text(line.strip())
        if not line:
          continue
        row = json.loads(line)
        category = category_from_prediction(
          row.get("fasttext_prediction") or "unknown_unknown",
          categorize_by,
        )
        if category not in qualifying:
          continue
        if category not in handles:
          path = output_dir / f"{category}_{categorize_by}_meshtastic.jsonl"
          handles[category] = open(path, "w", encoding="utf-8")
        handles[category].write(
          json.dumps(bucket_row(row), ensure_ascii=False) + "\n"
        )
        written[category] += 1
  finally:
    for handle in handles.values():
      handle.close()

  return dict(written), skipped


def parse_args() -> argparse.Namespace:
  parser = argparse.ArgumentParser(
    description="Split meshtastic _all.jsonl into per-category corpus files.",
  )
  parser.add_argument(
    "--categorize-by",
    choices=("lang", "script"),
    required=True,
    help="Bucket by ISO 639-3 language (eng) or ISO 15924 script (Latn).",
  )
  parser.add_argument(
    "--input",
    type=Path,
    default=DEFAULT_INPUT,
    help=f"Consolidated crawl file (default: {DEFAULT_INPUT}).",
  )
  parser.add_argument(
    "--output-dir",
    type=Path,
    default=DEFAULT_OUTPUT_DIR,
    help=f"Directory for bucket files (default: {DEFAULT_OUTPUT_DIR}).",
  )
  parser.add_argument(
    "--min-messages",
    type=int,
    default=DEFAULT_MIN_MESSAGES,
    help=f"Minimum number of messages to include in a bucket (default: {DEFAULT_MIN_MESSAGES}).",
  )
  return parser.parse_args()


def main() -> None:
  args = parse_args()
  if not args.input.is_file():
    raise FileNotFoundError(f"Missing input file: {args.input}")

  written, skipped = make_datasets(
    args.input,
    args.output_dir,
    args.categorize_by,
    args.min_messages,
  )

  if not written:
    print(f"No buckets with >={args.min_messages} messages")
  else:
    print(
      f"Wrote {sum(written.values())} rows in {len(written)} buckets "
      f"(min {args.min_messages} messages) to {args.output_dir}:"
    )
    for category in sorted(written):
      name = f"{category}_{args.categorize_by}_meshtastic.jsonl"
      print(f"  {name}: {written[category]}")

  if skipped:
    print(f"Skipped {len(skipped)} buckets below {args.min_messages} messages:")
    for category in sorted(skipped, key=lambda c: skipped[c], reverse=True):
      print(f"  {category}: {skipped[category]}")


if __name__ == "__main__":
  main()
