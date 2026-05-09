# Encoder design — TODO / spec summary

## Two encoder families

| Family        | Typical alphabet size | Bit width (idea) |
|---------------|------------------------|------------------|
| **Char mappers**  | Small (tens of symbols) | Often ~5–6 bits |
| **Token mappers** | Larger dictionaries     | Often ≥8 bits, can grow |

Both share:

- A **fixed alphabet** (closed set of chars or tokens).
- A **static, precompiled map** from each char/token to a **binary** codeword.

## Char mappers

Suited to **tiny** alphabets; examples (names are illustrative):

- **ENG_LC_MIN** — lowercase only (26) + a small punct set (e.g. 6 symbols → **32 symbols ≈ 5 bits**).
- **ENG_MIN_PUNCT** — `a–z`, `A–Z` (52) + ~12 punct → **~6 bits**.
- **ENG_MIN_DIG** — `a–z`, `A–Z` (52) + digits + plus/minus (and similar).
- **RU_LC_MIN**, etc. — same pattern for other languages.

## Token mappers

- **Larger** vocabularies; likely starting around **8 bits** per token, upper bound TBD.
- **Scoped** to language or text domain, e.g.:
  - `ENG`, `ENG_MESHCHAT`
  - `RU`, `RU_MESHCHAT`
  - `CODING_PYTHON`, `CODING_CPP`, …

## Pipeline after mapping

1. **Encode** → binary payload (via the precompiled map).
2. **Optional** — run a **general-purpose compressor** (configured per encoder instance).
3. **Encode for Unicode transport** (optional) — e.g. base64, base91, or another base algorithm so the result is representable as Unicode text. The CLI should expose a **flag** to skip this step and emit raw (or compressed) binary instead when desired.

Compression is **not** implicit: it is chosen **at encoder initialization**, e.g.:

- `TokenEncoder(..., compress=False)` — raw binary stream (then base step as needed).
- `TokenEncoder(..., compress=True)` — same map, then compress, then base step.

Same idea applies to char encoders once the type exists.

## Implementation notes (for later)

- [ ] Define trait / API shared by char and token encoders (alphabet, map, optional compression, base encoding).
- [ ] Precompile maps (build script or checked-in tables).
- [ ] Flesh out concrete alphabet presets (`ENG_LC_MIN`, `ENG_MESHCHAT`, …).
