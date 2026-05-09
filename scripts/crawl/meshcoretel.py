import json
from pathlib import Path
from time import sleep

import requests

BATCHES = 200
BATCH_SIZE = 50
CHANNEL_ID = 27
REGION_CODE = "MOW"

LANG = "ru"

API_URL = (
    "https://meshcoretel.ru/api/channels/{channel_id}/messages"
    "?region_code={region_code}&limit={limit}"
).format(
    channel_id=CHANNEL_ID,
    region_code=REGION_CODE,
    limit=BATCH_SIZE,
)

output_dir = Path(__file__).parent.parent.parent / "corpus"
output_dir.mkdir(parents=True, exist_ok=True)


def crawl_batch(batches: int, url: str):
    _url = url
    for i in range(batches):
        print(f"Crawling batch {i + 1} of {batches} - {_url}")
        resp = requests.get(_url, timeout=60)
        if not resp.ok:
            raise Exception(resp.text)
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

if __name__ == "__main__":
    out_path = output_dir / f"{LANG}_meshcoretel.jsonl"
    with open(out_path, "a", encoding="utf-8") as f:
        for msg in crawl_batch(BATCHES, API_URL):
            row = message_row(msg)
            f.write(json.dumps(row, ensure_ascii=False) + "\n")
            f.flush()
