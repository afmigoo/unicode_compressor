# Unicode-to-unicode compression tool

## Table of contents
- [Table of contents](#table-of-contents)
- [Description](#description)
- [Metrics](#metrics)
- [Stack and acknowledgements](#stack-and-acknowledgements)
- [How to use](#how-to-use)
    - [CLI tool](#cli-tool)
    - [Web-app](#web-app)
- [Privacy note](#privacy-note)

## Description

This is a tool that allows you to compress utf-8 strings into utf-8 strings using static dicttionaries compression algorithms. It is implemented in Rust and is available as a CLI tool and a web application powered by WASM.

It was motivated by meshtastic/meshcore having tiny bytes limit for the utf-8 payload.

## Metrics

> - Compression=`0.6` means payload size was reduced by 60%
> - Name format: `<lang>_<dataset>_<alphabet>`; so `en_wiki_punct_64` means English Wikipedia dataset with `punct_64`. alphabet

|Name|compression (avg/mean/std)|User time (avg)|Payload byte size (avg)|N|Example|
|---|---|---|---|---|---|
|ru_wiki_all|0.5259 / 0.5415 / 0.1286|0.0006|576.3641|1104|Одна богатая и знатная дама, госпожа Шереметева, у...|
|ru_wiki_32|0.6354 / 0.6438 / 0.0653|0.0009|539.6869|1102|одна богатая и знатная дама госпожа шереметева утр...|
|ru_wiki_punct_64|0.6071 / 0.6232 / 0.0996|0.0010|561.2219|1104|одна богатая и знатная дама, госпожа шереметева, у...|
|ru_wiki_alpha_64|0.6305 / 0.6390 / 0.0658|0.0009|539.7205|1102|Одна богатая и знатная дама госпожа Шереметева утр...|
|ru_wiki_128|0.6041 / 0.6206 / 0.0999|0.0009|561.7111|1104|Одна богатая и знатная дама, госпожа Шереметева, у...|
|en_wiki_all|0.3516 / 0.3618 / 0.1256|0.0008|379.2629|1107|Gates started to publish articles on the macrofung...|
|en_wiki_32|0.3751 / 0.3920 / 0.1476|0.0009|360.9447|1104|gates started to publish articles on the macrofung...|
|en_wiki_punct_64|0.3723 / 0.3846 / 0.0979|0.0009|376.3550|1107|gates started to publish articles on the macrofung...|
|en_wiki_alpha_64|0.3797 / 0.3956 / 0.1601|0.0009|361.8659|1104|Gates started to publish articles on the macrofung...|
|en_wiki_128|0.3751 / 0.3850 / 0.1153|0.0009|376.5176|1107|Gates started to publish articles on the macrofung...|
|ru_meshcoretel_all|0.5367 / 0.5897 / 0.2060|0.0004|58.7890|583|у меня t-deck|
|ru_meshcoretel_32|0.6016 / 0.6323 / 0.1476|0.0005|55.7101|552|у меня|
|ru_meshcoretel_punct_64|0.5449 / 0.6111 / 0.2776|0.0004|57.0549|565|у меня -|
|ru_meshcoretel_alpha_64|0.5992 / 0.6267 / 0.1558|0.0004|55.6329|553|у меня|
|ru_meshcoretel_128|0.5514 / 0.6067 / 0.2677|0.0004|57.1451|565|у меня -|

## Stack and acknowledgements

- **Tool's core** written in **Rust** by hand.
- **Static dictionaries** are generated with **Python** scripts written by hand.
- **Web-app module** is backendless and is powered by WASM.
- **Frontend** (*.js, *.css, *.html) vibe-coded in **JS**. Model is instructed to integrade Rust WASM module into the user interface.
- **Datasets** used
    - [wikipedia](https://wikipedia.org/) crawled for training `bpe_wiki`, `bpe_wiki_ru` and `bpe_wiki_en` dictionaries.
    - [Meshcoretel](https://meshcoretel.ru/) messages taken from #public Meshcore channel (Moscow region) for training `bpe_meshcoretel_ru` dictionary.
    - [The Stack Dataset](https://huggingface.co/datasets/bigcode/the-stack) for training `bpe_coding` dictionary.

## How to use

### CLI tool

```bash
# Build
cd rust && cargo build --release
# Use
./target/release/unipress --help
```

### Web-app
#### Public version
Public version is available at https://zip.cyanshark.org/

#### Self-hosted version

```bash
# Build (optional)
docker build . -f docker/Dockerfile -t ghcr.io/afmigoo/unicode_compressor:latest
# Run
docker compose up
# Go to http://localhost:80/
```
## How to test

```bash
# v0
docker run --rm \
    -v $(pwd)/web/v1:/app -w /app \
    node:22-alpine \
    node smoke_node.mjs
# v1-preview ... TODO
```

## Privacy note

- In web-app mode your data never leaves your browser side, there is no backend where it could be stored.
- This is not encryption, this is encoding. Using payloads generated with this project in unencrypted channels exposes your messages.

## Planned
- `v1`
    - [ ] Create reference dataset for performance measurement
    - [ ] Rewrite greedy encoding of static dict encoder
    - [x] Rewrite core in Rust for compatability between cli and web.
    - [x] Refactor encoders to support diferent alphabets. Now global hard-coded alphabet is shared between all encoders

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
