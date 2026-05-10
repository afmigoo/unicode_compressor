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

Try it out at https://zip.cyanshark.org/

## Metrics

> - Compression=`0.6` means payload size was reduced by 60%
> - Name format: `<lang>_<dataset>_<alphabet>`; so `en_wiki_punct_64` means English Wikipedia dataset with `punct_64`. alphabet

|Name|compression (avg/mean/std)|User time (avg)|Payload byte size (avg)|N|Example|
|---|---|---|---|---|---|
|ru_wiki_all|0.5741 / 0.6038 / 0.1368|0.0014|575.2056|1104|Одна богатая и знатная дама, госпожа Шереметева, у...|
|total|0.5469 / 0.5904 / 0.1795|0.0027|378.4663|13947||
|ru_wiki_32|0.6931 / 0.7025 / 0.0667|0.0077|541.1162|1102|одна богатая и знатная дама госпожа шереметева утр...|
|ru_wiki_punct_64|0.6439 / 0.6608 / 0.1023|0.0044|560.8315|1104|одна богатая и знатная дама, госпожа шереметева, у...|
|ru_wiki_alpha_64|0.6717 / 0.6828 / 0.0697|0.0039|543.3557|1102|Одна богатая и знатная дама госпожа Шереметева утр...|
|ru_wiki_128|0.6290 / 0.6500 / 0.1060|0.0019|563.5045|1104|Одна богатая и знатная дама, госпожа Шереметева, у...|
|en_wiki_all|0.3925 / 0.4108 / 0.1336|0.0016|378.8753|1107|Gates started to publish articles on the macrofung...|
|en_wiki_32|0.4585 / 0.4700 / 0.1517|0.0039|362.2817|1104|gates started to publish articles on the macrofung...|
|en_wiki_punct_64|0.4371 / 0.4454 / 0.1060|0.0027|376.5727|1107|gates started to publish articles on the macrofung...|
|en_wiki_alpha_64|0.4369 / 0.4564 / 0.1633|0.0026|363.1304|1104|Gates started to publish articles on the macrofung...|
|en_wiki_128|0.4137 / 0.4293 / 0.1227|0.0019|376.7407|1107|Gates started to publish articles on the macrofung...|
|ru_meshcoretel_all|0.5430 / 0.6053 / 0.2241|0.0005|55.3411|598|Инет рухнул шоль?|
|ru_meshcoretel_32|0.6518 / 0.6814 / 0.1484|0.0010|51.1281|570|инет рухнул шоль|
|ru_meshcoretel_punct_64|0.5816 / 0.6400 / 0.2696|0.0007|52.6753|582|инет рухнул шоль?|
|ru_meshcoretel_alpha_64|0.6296 / 0.6582 / 0.1512|0.0007|51.2509|570|Инет рухнул шоль|
|ru_meshcoretel_128|0.5607 / 0.6228 / 0.2670|0.0005|52.9089|582|Инет рухнул шоль?|

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
# v1-preview
cd rust && cargo test --release
```

## Privacy note

- In web-app mode your data never leaves your browser side, there is no backend where it could be stored.
- This is not encryption, this is encoding. Using payloads generated with this project in unencrypted channels exposes your messages.

## Planned
- `v1`
    - [x] Create reference dataset for performance measurement
    - [x] Rewrite greedy encoding of static dict encoder
    - [x] Rewrite core in Rust for compatability between cli and web.
    - [x] Refactor encoders to support diferent alphabets. Now global hard-coded alphabet is shared between all encoders

## Algorithms (names are not final)

- `adaptive` (default, recommended) - tries all the algorithms and chooses the best one based on the size of the encoded string. Costs one extra utf-8 character to mark which algorithm was used
- encoder variation parameters:
    - `map` vs `token`
        - `map` encoders simply map unicode characters one-to-one. This may compress data, if payload contains multi-byte characters and they are mapped to ASCII range. Not the best choice, but a decent fallback.
        - `token` encoders encode chunks of characters (tokens) instead of individual characters similar to LLM tokenization algorithms, but simpler
    - transport: `bin` vs `utf8`
        - `bin` encoders pack data into binary n-bit tokens, then encode binary data into base91
        - `utf8` encoders encode data straight into utf-8 characters
    - alphabet: each encoder has an optional alphabet, which limits the set of characters that can be encoded
    - dataset: each `token` encoder is trained on a specific dataset

By trying all the variations, `adaptive` encoder can find the best encoding for the given payload with a small cost of 1 byte overhead.
