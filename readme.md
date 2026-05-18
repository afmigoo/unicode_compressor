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

Unicode-to-unicode compression built around **various pre-built static dictionaries**, not one universal model. Each dictionary (its alphabet, tokens) is baked into the binary at compilation time.

At runtime the default **adaptive** encoder evaluates every variant that can handle the input and **keeps the shortest encoded string**, prefixing one Unicode character so the decoder knows which table was used—paying a tiny fixed overhead for the freedom to pick among N static layouts. In the worst-case scenario, when no variant produced a net-positive compression, encoded payload will have a 1 byte overhead.

Implemented in Rust (CLI + WASM web demo). Motivated by tight UTF-8 payload limits on meshtastic/meshcore.

**UTF-8 transport** was intentionally picked over binary: compressed output is meant to be **ordinary text** you can paste into chat, forums, or any UTF-8 channel without binary-safe tooling.

Try it out at https://afmigoo.github.io/unicode_compressor/

## Metrics

> - Compression=`0.6` means payload size was reduced by 60%
> - Name format: `<lang>_<dataset>_<alphabet>`; so `en_wiki_punct_64` means English Wikipedia dataset with `punct_64`.

|Name|compression (avg/mean/std)|User time (avg)|Payload byte size (avg)|N|Example|
|---|---|---|---|---|---|
|ru_wiki_32|0.6924 / 0.7020 / 0.0672|0.0025|541.1162|1102|одна богатая и знатная дама госпожа шереметева утр...|
|total|0.5581 / 0.6000 / 0.1747|0.0022|467.4670|6578||
|ru_wiki_256|0.6050 / 0.6364 / 0.1378|0.0014|570.1422|1104|Одна богатая и знатная дама, госпожа Шереметева, у...|
|en_wiki_32|0.4529 / 0.4656 / 0.1513|0.0024|362.2817|1104|gates started to publish articles on the macrofung...|
|en_wiki_128|0.4120 / 0.4288 / 0.1232|0.0018|376.7407|1107|Gates started to publish articles on the macrofung...|
|ru_meshcoretel_32|0.6620 / 0.6860 / 0.1372|0.0004|52.0407|1007|ну она долбит да|
|ru_meshcoretel_256|0.5478 / 0.5938 / 0.2003|0.0005|55.5397|1034|ну она долбит, да :))|
|en_coding_32|0.4920 / 0.5012 / 0.0651|0.0386|7040.9500|60|package modelsimport     iadedafeefbaeccbfbef gith...|
|en_coding_128|0.3561 / 0.3676 / 0.0735|0.0424|8332.5000|60|package models

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
    - [ ] Make dicts with different sizes of vocabulary use a single modular dict
        - Subtract dictionaries of smaller sizes from the larger ones to reduce the size of binary. For an example, instead of having `dict_64 {1, ... 64}` and `dict_128 {1, ... 128}` we can have `dict_64 {1, ... 64}` and `dict_128 {65, ... 128}`, since dict_128 always contains dict_64.

## Algorithms (names are not final)

- `adaptive` (default, recommended) - tries all the algorithms and chooses the best one based on the size of the encoded string. Costs one extra utf-8 character to mark which algorithm was used
- encoder variation parameters:
    - `map` vs `token`
        - `map` encoders simply map unicode characters one-to-one. This may compress data if payload contains mostly multi-byte characters. Not the best choice, but a decent fallback.
        - `token` encoders encode chunks of characters (tokens) instead of individual characters. Similar to LLM tokenization algorithms, but simpler
    - transport: `bin` vs `utf8`
        - `bin` encoders pack data into binary n-bit tokens, then encode binary data into base91. This process saves space by sub-byte packing but then inflates when passed through base91 encoding.
        - `utf8` encoders encode data straight into utf-8 characters
    - alphabet: each encoder has an optional alphabet, which limits the set of characters that can be encoded
    - dataset: each `token` encoder is trained on a specific dataset

By trying all the variations, `adaptive` encoder can find the best encoding for the given payload with a small cost of 1 byte overhead.
