#!/usr/bin/env python3
"""
Crawls Meshtastic Map text messages into a single corpus JSONL file.

Each row includes a fastText language-id prediction (e.g. eng_Latn). Split into
per-category training files afterward:

  python3 scripts/prune_duplicates.py
  python3 scripts/make_meshtastic_datasets.py --categorize-by lang
  python3 scripts/make_meshtastic_datasets.py --categorize-by script

fasttext pip package seems to be unmaintained and it breaks with numpy 2.0+.
so we use pinned version from requirements_fasttext.txt: pip install -r requirements_fasttext.txt

usage:
  python3 scripts/crawl/meshtastic_map.py
  python3 scripts/crawl/meshtastic_map.py --batch-size 50 --batches 200 --delay 2
  python3 scripts/crawl/meshtastic_map.py --last-id 33195451 --batches 50
"""

from __future__ import annotations

import argparse
import json
from collections import Counter
from pathlib import Path
from time import sleep

import fasttext
import requests
from huggingface_hub import hf_hub_download

MODEL_PATH = hf_hub_download(
  repo_id="facebook/fasttext-language-identification",
  filename="model.bin",
)
FASTTEXT_MODEL = fasttext.load_model(MODEL_PATH)

API_BASE = "https://meshtastic.liamcottle.net/api/v1/text-messages"

OUTPUT_DIR = Path(__file__).parent.parent.parent / "corpus" / "meshtastic"
OUTPUT_FILE = OUTPUT_DIR / "_all.jsonl"


def predict_fasttext_label(text: str) -> str:
  lines = [ln for ln in text.splitlines() if ln.strip()]
  if not lines:
    return "unknown_unknown"
  guesses = Counter()
  for line in lines:
    guess = FASTTEXT_MODEL.predict(line)[0][0].replace("__label__", "")
    guesses[guess] += 1
  return guesses.most_common(1)[0][0]


def api_url(batch_size: int, last_id: int | None = None) -> str:
  url = f"{API_BASE}?count={batch_size}&order=desc"
  if last_id is not None:
    url += f"&last_id={last_id}"
  return url


def crawl_batches(
  batches: int,
  batch_size: int,
  delay: float,
  start_last_id: int | None = None,
):
  url = api_url(batch_size, start_last_id)
  for i in range(batches):
    print(f"Crawling batch {i + 1} of {batches} - {url}")
    resp = requests.get(url, timeout=60)
    if not resp.ok:
      raise RuntimeError(resp.text)
    data = resp.json()
    messages = data.get("text_messages") or []
    print(f"{len(messages)} messages found")
    if not messages:
      break
    for msg in messages:
      yield msg
    last_id = min(int(m["id"]) for m in messages)
    url = api_url(batch_size, last_id)
    if i + 1 < batches:
      sleep(delay)


def message_row(msg: dict) -> dict | None:
  text = (msg.get("text") or "").strip()
  if not text:
    return None
  return {
    "text": text,
    "date": msg.get("created_at") or "",
    "packet_id": str(msg.get("packet_id") or ""),
    "fasttext_prediction": predict_fasttext_label(text),
  }


def parse_args() -> argparse.Namespace:
  parser = argparse.ArgumentParser(
    description="Crawl Meshtastic Map messages into corpus/meshtastic/_all.jsonl.",
  )
  parser.add_argument(
    "--batch-size",
    type=int,
    default=50,
    help="Messages per API request (count param, default: 50).",
  )
  parser.add_argument(
    "--batches",
    type=int,
    default=200,
    help="Number of paginated API requests (default: 200).",
  )
  parser.add_argument(
    "--delay",
    type=float,
    default=2.0,
    help="Seconds to sleep between requests (default: 2).",
  )
  parser.add_argument(
    "--last-id",
    type=int,
    default=None,
    metavar="ID",
    help=(
      "Start pagination before this message id (API last_id with order=desc). "
      "Omit to begin from the newest messages."
    ),
  )
  return parser.parse_args()


def main() -> None:
  args = parse_args()
  OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
  written = 0
  skipped = 0

  with open(OUTPUT_FILE, "a", encoding="utf-8") as out:
    for msg in crawl_batches(
      args.batches,
      args.batch_size,
      args.delay,
      start_last_id=args.last_id,
    ):
      row = message_row(msg)
      if row is None:
        skipped += 1
        continue
      out.write(json.dumps(row, ensure_ascii=False) + "\n")
      out.flush()
      written += 1

  print(f"Skipped {skipped} empty messages")
  print(f"Wrote {written} rows to {OUTPUT_FILE}")


if __name__ == "__main__":
  main()
