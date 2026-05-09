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
|ru_wiki_all|0.5266 / 0.5420 / 0.1289|0.0009|576.3641|1104|Одна богатая и знатная дама, госпожа Шереметева, у...|
|ru_wiki_32|0.6898 / 0.6980 / 0.0650|0.0021|539.6869|1102|одна богатая и знатная дама госпожа шереметева утр...|
|ru_wiki_punct_64|0.6284 / 0.6426 / 0.0982|0.0017|561.2219|1104|одна богатая и знатная дама, госпожа шереметева, у...|
|ru_wiki_alpha_64|0.6639 / 0.6741 / 0.0664|0.0015|539.7205|1102|Одна богатая и знатная дама госпожа Шереметева утр...|
|ru_wiki_128|0.6048 / 0.6210 / 0.0997|0.0013|561.7111|1104|Одна богатая и знатная дама, госпожа Шереметева, у...|
|en_wiki_all|0.3548 / 0.3644 / 0.1274|0.0012|379.2629|1107|Gates started to publish articles on the macrofung...|
|en_wiki_32|0.4500 / 0.4611 / 0.1496|0.0016|360.9447|1104|gates started to publish articles on the macrofung...|
|en_wiki_punct_64|0.4168 / 0.4214 / 0.1010|0.0014|376.3550|1107|gates started to publish articles on the macrofung...|
|en_wiki_alpha_64|0.4250 / 0.4421 / 0.1600|0.0013|361.8659|1104|Gates started to publish articles on the macrofung...|
|en_wiki_128|0.3792 / 0.3871 / 0.1161|0.0013|376.5176|1107|Gates started to publish articles on the macrofung...|
|ru_meshcoretel_all|0.5501 / 0.6000 / 0.2052|0.0007|58.7890|583|у меня t-deck|
|ru_meshcoretel_32|0.6609 / 0.6871 / 0.1543|0.0008|55.7101|552|у меня|
|ru_meshcoretel_punct_64|0.5808 / 0.6406 / 0.2819|0.0008|57.0549|565|у меня -|
|ru_meshcoretel_alpha_64|0.6406 / 0.6667 / 0.1586|0.0008|55.6329|553|у меня|
|ru_meshcoretel_128|0.5666 / 0.6190 / 0.2701|0.0007|57.1451|565|у меня -|

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
