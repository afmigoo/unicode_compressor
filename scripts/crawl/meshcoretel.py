#!/usr/bin/env python3
"""
Crawls meshcoretel.ru API for channel messages.

Regions are crawled one at a time; a failed region is skipped and the next is tried.

usage:
  python3 scripts/crawl/meshcoretel.py
  python3 scripts/crawl/meshcoretel.py --region-codes MOW,KZN,ROV --channel-id 27 --batch-size 50 --batches 200
"""

# Russian-speaking meshcoretel region codes.
DEFAULT_RU_REGION_CODES = (
  "BSK", "CEE", "CEK", "GOJ", "GSV", "IJK", "IKT", "IWA", "KHV", "KLD",
  "KLF", "KUF", "KVX", "KZN", "LPK", "MOW", "NTR", "OMS", "OVB", "ROV",
  "RZN", "SVX", "TBW", "TYA", "UFA", "ULV", "VLM", "VOG", "VOZ", "VVO",
)

import argparse
import json
from pathlib import Path
from time import sleep

import requests

LANG = "ru"

output_dir = Path(__file__).parent.parent.parent / "corpus"
output_dir.mkdir(parents=True, exist_ok=True)


def api_url(channel_id: int, region_code: str, batch_size: int) -> str:
  return (
    "https://meshcoretel.ru/api/channels/{channel_id}/messages"
    "?region_code={region_code}&limit={limit}"
  ).format(
    channel_id=channel_id,
    region_code=region_code,
    limit=batch_size,
  )


def crawl_batch(batches: int, url: str, region_code: str):
  _url = url
  headers = {
    'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:150.0) Gecko/20100101 Firefox/150.0',
    # 'Accept': '*/*',
    # 'Accept-Language': 'en-US,en;q=0.9',
    # 'Accept-Encoding': 'gzip, deflate, br, zstd',
    # 'Referer': 'https://meshcoretel.ru/ru/VIE/channels',
  }
  for i in range(batches):
    print(f"[{region_code}] Crawling batch {i + 1} of {batches} - {_url}")
    resp = requests.get(_url, timeout=60, headers=headers)
    if not resp.ok:
      raise Exception(f"HTTP {resp.status_code}: {resp.text}")
    data = resp.json()
    next_before = data["next_before"]
    print(f'{len(data["messages"])} messages found')
    for msg in data["messages"]:
      yield msg
    _url = url + "&before=" + str(next_before)
    sleep(2)


def message_row(msg: dict) -> dict:
  return {
    "text": msg.get("message") or "",
    "date": msg.get("when") or "",
    "hash": msg.get("hash") or "",
  }


def parse_args() -> argparse.Namespace:
  parser = argparse.ArgumentParser(
    description="Crawl meshcoretel.ru channel messages into corpus JSONL.",
  )
  parser.add_argument(
    "--region-codes",
    type=lambda x: [code.strip() for code in x.split(",") if code.strip()],
    default=list(DEFAULT_RU_REGION_CODES),
    metavar="CODES",
    help=(
      "Comma-separated region codes, crawled sequentially "
      "(default: all Russian Federation codes)."
    ),
  )
  parser.add_argument(
    "--channel-id",
    type=int,
    default=27,
    help="Channel ID (default: 27, #public Moscow).",
  )
  parser.add_argument(
    "--batch-size",
    type=int,
    default=50,
    help="Messages per API request (default: 50).",
  )
  parser.add_argument(
    "--batches",
    type=int,
    default=200,
    help="Number of paginated batches to crawl (default: 200).",
  )
  return parser.parse_args()


if __name__ == "__main__":
  args = parse_args()
  out_path = output_dir / f"{LANG}_meshcoretel.jsonl"
  with open(out_path, "a", encoding="utf-8") as f:
    for region_code in args.region_codes:
      url = api_url(args.channel_id, region_code, args.batch_size)
      print(f"Starting region {region_code}")
      try:
        for msg in crawl_batch(args.batches, url, region_code):
          row = message_row(msg)
          f.write(json.dumps(row, ensure_ascii=False) + "\n")
          f.flush()
      except Exception as exc:
        print(f"Region {region_code} failed, continuing: {exc}")
      else:
        print(f"Finished region {region_code}")
