import requests
from pathlib import Path
from datetime import datetime
from time import sleep

BATCHES = 60
BATCH_SIZE = 100

URL = "https://meshcoretel.ru/api/channels/{channel_id}/messages?region_code={region_code}&limit={limit}".format(
    channel_id=27, # public
    region_code="MOW",
    limit=BATCH_SIZE,
)

output_dir = Path(__file__).parent.parent / "corpus/raw/meshcoretel"
output_dir.mkdir(exist_ok=True)

def crawl_batch(batches: int):
    _url = URL
    for i in range(batches):
        print(f"Crawling batch {i + 1} of {batches} - {_url}")
        resp = requests.get(_url)
        if not resp.ok:
            raise Exception(resp.text)
        next_before = resp.json()['next_before']
        for msg in resp.json()['messages']:
            yield msg['message']
        _url = URL + "&before=" + str(next_before)
        sleep(2)

with open(output_dir / "ru.txt", "w", encoding="utf-8") as f:
    for message in crawl_batch(BATCHES):
        f.write(message + "\n")
        f.flush()
