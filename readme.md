# Unicode compresser

## Description

This is a web application that allows you to compress utf-8 strings into utf-8 strings using different algorithms. It was motivated by meshtastic/meshcore having tiny bytes limit for the utf-8 payload.


## Algorithms (names are not final)

- `utf8` - just maps each alphabet character to a utf-8 character one to one
- `utf8_optimize` - like `utf8`, but puts frequent letters first, so they use less bytes
- `base64` - maps each alphabet character into raw bytes, then encodes them into base64
- `base64_compress` - like `base64`, but compresses the raw bytes using zlib
- `base91` - maps each alphabet character into raw bytes, then encodes them into base91
- `base91_compress` - like `base91`, but compresses the raw bytes using zlib
- `base85` - maps each alphabet character into raw bytes, then encodes them into base85
- `base85_compress` - like `base85`, but compresses the raw bytes using zlib
- `decider` - tries all the above algorithms and chooses the best one based on the size of the encoded string

## Stack and acknowledgements

- **Prototype** ([prototype/](prototype/)) written in **Python by hand**
- **Frontend ([web/](web/))** vibe-coded in **JS**. Model was instructed to mimick Python code as closely as possible. The result was tested by hand and by unit tests

## How to run

```bash
docker compose up --build
```
Then go to http://localhost:80/v1/

## How to test

```bash
docker run --rm \
    -v $(pwd)/web/v1:/app -w /app \
    node:22-alpine \
    node smoke_node.mjs
```
