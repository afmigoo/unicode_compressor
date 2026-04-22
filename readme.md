# Unicode-to-unicode compression tool

## Description

This is a web application that allows you to compress utf-8 strings into utf-8 strings using different algorithms. It was motivated by meshtastic/meshcore having tiny bytes limit for the utf-8 payload.

## Stack and acknowledgements

- **All but frontend** written in **Python by hand**
- **Frontend ([web/](web/))** vibe-coded in **JS**. Model is instructed to mimick Python code as closely as possible and is supervised. The result is tested by hand and by smoke tests.
- **Datasets** used
    - [wikipedia](https://wikipedia.org/) crawled for training `bpe_wiki`, `bpe_wiki_ru` and `bpe_wiki_en` dictionaries.
    - [Meshcoretel](https://meshcoretel.ru/) messages taken from #public Meshcore channel (Moscow region) for training `bpe_meshcoretel_ru` dictionary.
    - [The Stack Dataset](https://huggingface.co/datasets/bigcode/the-stack) for training `bpe_coding` dictionary.

## How to use

### General

Just serve directory `web/` (or `web/vX` for specific version) with any webserver. There is no backend

### Docker

#### Build (optional)
```bash
docker build . -f docker/Dockerfile -t ghcr.io/afmigoo/unicode_compressor:latest
```

#### Run
```bash
docker compose up
```
Then go to http://localhost:80/v0/

## How to test

```bash
docker run --rm \
    -v $(pwd)/web/v1:/app -w /app \
    node:22-alpine \
    node smoke_node.mjs
```

## Privacy note

- It is a frontend-only web-application, meaning your data never leaves your browser.
- This is not encryption, this is encoding. Using payloads generated with this project in unencrypted channels exposes your messages.

## Planned
- `v1`
    - [ ] Create reference dataset for performance measurement
    - [ ] Rewrite greedy encoding of static dict encoder
    - [ ] Refactor encoders to support diferent alphabets. Now global hard-coded alphabet is shared between all encoders

## Algorithms (names are not final)

- `decider` (default, recommended) - tries all the algorithms from below and chooses the best one based on the size of the encoded string. Costs one extra utf-8 character to mark which algorithm was used
- `utf8` - just maps each alphabet character to a utf-8 character one to one
- `utf8_optimize` - like `utf8`, but puts frequent letters first, so they use less bytes
- `base64` - maps each alphabet character into raw bytes, then encodes them into base64
- `base64_compress` - like `base64`, but compresses the raw bytes using zlib
- `base91` - maps each alphabet character into raw bytes, then encodes them into base91
- `base91_compress` - like `base91`, but compresses the raw bytes using zlib
- `base85` - maps each alphabet character into raw bytes, then encodes them into base85
- `base85_compress` - like `base85`, but compresses the raw bytes using zlib
- `bpe_wiki` - pretrained BPE dictionary with python. Frontend encodes/decodes using said dictionary (greedy encoding). Trained on wikipedia articles in Russian and English.
- `bpe_wiki_ru` - like `bpe_wiki`, but trained Russian articles.
- `bpe_wiki_en` - like `bpe_wiki`, but trained English articles.
- `bpe_meshcoretel_ru` - like `bpe_wiki`, but trained with messages taken from #public Meshcore channel (Moscow region).
- `bpe_coding` - like `bpe_wiki`, but trained on a tiny subset of [The Stack Dataset](https://huggingface.co/datasets/bigcode/the-stack).
