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
> - User time measured on Intel Core i5-1130G7; encoder is single-threaded.

|Name|compression (avg/mean/std)|User time (avg)|Payload byte size (avg)|N|Example|
|---|---|---|---|---|---|
|total|0.5425 / 0.6000 / 0.1992|0.0016|287.9633|11894||
|ru_wiki_32|0.6924 / 0.7020 / 0.0674|0.0020|541.1162|1102|одна богатая и знатная дама госпожа шереметева утр...|
|ru_wiki_256|0.6084 / 0.6364 / 0.1204|0.0015|570.1422|1104|Одна богатая и знатная дама, госпожа Шереметева, у...|
|en_wiki_32|0.4530 / 0.4656 / 0.1513|0.0029|362.2817|1104|gates started to publish articles on the macrofung...|
|en_wiki_128|0.4125 / 0.4288 / 0.1218|0.0024|376.7407|1107|Gates started to publish articles on the macrofung...|
|en_coding_32|0.4920 / 0.5012 / 0.0651|0.0427|7040.9500|60|package modelsimport     iadedafeefbaeccbfbef gith...|
|en_coding_128|0.3561 / 0.3676 / 0.0735|0.0472|8332.5000|60|package models

import (
    i878a80d2330e89d26896...|
|ru_meshcoretel_32|0.6557 / 0.6842 / 0.1386|0.0004|52.4957|2683|люди который час|
|ru_meshcoretel_256|0.5254 / 0.5870 / 0.2356|0.0004|54.8867|2815|люди, который час?|
|cyr_meshtastic_512|0.6185 / 0.6243 / 0.0909|0.0005|114.3906|635|я для домашних собирал сеть по маленькому поселку,...|
|lat_meshtastic_128|0.3182 / 0.2920 / 0.1573|0.0008|73.8922|612|Wie viele Hpfer brauchst du, um mich abzuholen?|
|lat_meshtastic_1024|0.3021 / 0.2793 / 0.1627|0.0006|76.2402|612|Wie viele Hüpfer brauchst du, um mich abzuholen?|

## Stack and acknowledgements

- **Tool's core** written in **Rust**.
- **Static dictionaries** are generated with **Python**.
- **Web-app module** is backendless and powered by WASM.
- **Frontend** (*.js, *.css, *.html) vibe-coded in **JS**. Model is instructed to integrade Rust WASM module into the user interface.
- **Datasets** used:
    - [wikipedia](https://wikipedia.org/) - articles crawled
    - [Liam Cottle's Meshtastic Map](https://meshtastic.liamcottle.net/) - API crawled
    - [Meshcoretel](https://meshcoretel.ru/) - API crawled
    - [The Stack Dataset](https://huggingface.co/datasets/bigcode/the-stack) - Hugging Face dataset

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
Public version is available at https://afmigoo.github.io/unicode_compressor/

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
cd rust && cargo test
```

## Privacy note

- In web-app mode your data never leaves your browser side, there is no backend where it could be stored.
- This is not encryption, this is encoding. Using payloads generated with this project in unencrypted channels exposes your messages.

## Planned
- `v1`
    - [ ] Add more languages

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
