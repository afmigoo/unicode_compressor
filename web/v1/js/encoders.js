/**
 * Ports logic from python/demo_*.py — browser is authoritative for wire format.
 */
import { zlibSync, unzlibSync } from "../vendor/fflate-browser.js";

/** Python 3 string.punctuation */
const PYTHON_PUNCTUATION = "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~";

export const ALPHABET =
  " \n" +
  PYTHON_PUNCTUATION +
  "0123456789" +
  "абвгдеёжзийклмнопрстуфхцчшщъыьэюя" +
  "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ" +
  "ABCDEFGHIJKLMNOPQRSTUVWXYZ" +
  "abcdefghijklmnopqrstuvwxyz";

const B85_ALPHABET =
  "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz!#$%&()*+-;<=>?@^_`{|}~";

const BASE91_ALPHABET = [
  "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
  "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
  "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
  "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
  "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "!", "#", "$",
  "%", "&", "(", ")", "*", "+", ",", ".", "/", ":", ";", "<", "=",
  ">", "?", "@", "[", "]", "^", "_", "`", "{", "|", "}", "~", '"',
];

const BASE91_DECODE = Object.fromEntries(
  BASE91_ALPHABET.map((ch, i) => [ch, i]),
);

export function bytesToBase64(bytes) {
  let binary = "";
  const chunk = 0x8000;
  for (let i = 0; i < bytes.length; i += chunk) {
    binary += String.fromCharCode.apply(
      null,
      bytes.subarray(i, Math.min(i + chunk, bytes.length)),
    );
  }
  return btoa(binary);
}

export function base64ToBytes(b64) {
  const binary = atob(b64);
  const out = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) {
    out[i] = binary.charCodeAt(i);
  }
  return out;
}

function b85BuildTables() {
  const chars = [...B85_ALPHABET].map((c) => c);
  const chars2 = [];
  for (let a = 0; a < 85; a++) {
    for (let b = 0; b < 85; b++) {
      chars2.push(chars[a] + chars[b]);
    }
  }
  return { chars, chars2 };
}

const _b85 = b85BuildTables();

/** CPython b85encode(b, pad=False) — no z/y shortcuts (foldnuls=False). */
export function b85encode(bytes, pad = false) {
  let b = bytes;
  /** Python (-len)%4 */
  const padding = (4 - (b.length % 4)) % 4;
  if (padding) {
    const nb = new Uint8Array(b.length + padding);
    nb.set(b);
    b = nb;
  }
  const view = new DataView(b.buffer, b.byteOffset, b.length);
  const n = b.length / 4;
  const chunks = [];
  const { chars, chars2 } = _b85;
  for (let i = 0; i < n; i++) {
    const word = view.getUint32(i * 4, false);
    chunks.push(
      chars2[Math.floor(word / 614125)] +
        chars2[Math.floor(word / 85) % 7225] +
        chars[word % 85],
    );
  }
  if (padding && !pad) {
    let last = chunks[chunks.length - 1];
    last = last.slice(0, last.length - padding);
    chunks[chunks.length - 1] = last;
  }
  return chunks.join("");
}

/** CPython b85decode — pad with ~ to multiple of 5, strip trailing bytes. */
export function b85decode(str) {
  const dec = new Array(256).fill(null);
  for (let i = 0; i < B85_ALPHABET.length; i++) {
    dec[B85_ALPHABET.charCodeAt(i)] = i;
  }
  const enc = new TextEncoder().encode(str);
  /** Python (-len)%5 */
  let padding = (5 - (enc.length % 5)) % 5;
  const padded = new Uint8Array(enc.length + padding);
  padded.set(enc);
  padded.fill(126, enc.length); // '~'

  const out = [];
  for (let i = 0; i < padded.length; i += 5) {
    let acc = 0;
    for (let j = 0; j < 5; j++) {
      const v = dec[padded[i + j]];
      if (v == null) {
        throw new Error(`bad base85 character at position ${i + j}`);
      }
      acc = acc * 85 + v;
    }
    out.push((acc >>> 24) & 0xff, (acc >>> 16) & 0xff, (acc >>> 8) & 0xff, acc & 0xff);
  }
  const full = Uint8Array.from(out);
  if (padding) {
    return full.slice(0, full.length - padding);
  }
  return full;
}

export function base91Encode(bindata) {
  let b = 0;
  let n = 0;
  let out = "";
  for (let count = 0; count < bindata.length; count++) {
    b |= bindata[count] << n;
    n += 8;
    if (n > 13) {
      let v = b & 8191;
      if (v > 88) {
        b >>= 13;
        n -= 13;
      } else {
        v = b & 16383;
        b >>= 14;
        n -= 14;
      }
      out += BASE91_ALPHABET[v % 91] + BASE91_ALPHABET[Math.floor(v / 91)];
    }
  }
  if (n) {
    out += BASE91_ALPHABET[b % 91];
    if (n > 7 || b > 90) {
      out += BASE91_ALPHABET[Math.floor(b / 91)];
    }
  }
  return out;
}

export function base91Decode(encodedStr) {
  let v = -1;
  let b = 0;
  let n = 0;
  const out = [];
  for (const strletter of encodedStr) {
    if (!(strletter in BASE91_DECODE)) continue;
    const c = BASE91_DECODE[strletter];
    if (v < 0) {
      v = c;
    } else {
      v += c * 91;
      b |= v << n;
      n += (v & 8191) > 88 ? 13 : 14;
      while (true) {
        out.push(b & 255);
        b >>= 8;
        n -= 8;
        if (!(n > 7)) break;
      }
      v = -1;
    }
  }
  if (v + 1) {
    out.push((b | (v << n)) & 255);
  }
  return Uint8Array.from(out);
}

export class UTF8Encoder {
  static PRINTABLE_RANGES = [
    [32, 126],
    [161, 255],
  ];

  static MAX_CHARS = UTF8Encoder.PRINTABLE_RANGES.reduce(
    (s, [a, b]) => s + (b - a + 1),
    0,
  );

  static ALPH_FREQ = {
    cyrillic: {
      О: 11.18,
      Е: 8.75,
      А: 7.64,
      И: 7.09,
      Н: 6.78,
      Т: 6.09,
      С: 4.97,
      Л: 4.96,
      В: 4.38,
      Р: 4.23,
      К: 3.3,
      М: 3.17,
      Д: 3.09,
      П: 2.47,
      Ы: 2.36,
      У: 2.22,
      Б: 2.01,
      Я: 1.96,
      Ь: 1.84,
      Г: 1.72,
      З: 1.48,
      Ч: 1.4,
      Й: 1.21,
      Ж: 1.01,
      Х: 0.95,
      Ш: 0.72,
      Ю: 0.47,
      Ц: 0.39,
      Э: 0.36,
      Щ: 0.3,
      Ф: 0.21,
      Ё: 0.2,
      Ъ: 0.02,
    },
    english: {
      E: 12.02,
      T: 9.1,
      A: 8.12,
      O: 7.68,
      I: 7.31,
      N: 6.95,
      S: 6.28,
      R: 6.02,
      H: 5.92,
      D: 4.32,
      L: 3.98,
      U: 2.88,
      C: 2.71,
      M: 2.61,
      F: 2.3,
      Y: 2.11,
      W: 2.09,
      G: 2.03,
      P: 1.82,
      B: 1.49,
      V: 1.11,
      K: 0.69,
      X: 0.17,
      Q: 0.11,
      J: 0.1,
      Z: 0.07,
    },
  };

  constructor(alphabet, optimizePreference = ["cyrillic", "english"]) {
    let a = alphabet;
    if (optimizePreference && optimizePreference.length) {
      a = UTF8Encoder.optimizeAlphabet(a, optimizePreference);
    }
    this.alphabet = a;
    this.charToIndex = UTF8Encoder.mapAlphabet(a);
    this.indexToChar = Object.fromEntries(
      Object.entries(this.charToIndex).map(([k, v]) => [v, k]),
    );
  }

  encode(text) {
    let out = "";
    for (const ch of text) {
      if (!(ch in this.charToIndex)) {
        throw new Error(`Character not in alphabet: ${ch}`);
      }
      out += this.charToIndex[ch];
    }
    return out;
  }

  decode(text) {
    let out = "";
    for (const ch of text) {
      if (!(ch in this.indexToChar)) {
        throw new Error(`Unknown mapped character: ${ch}`);
      }
      out += this.indexToChar[ch];
    }
    return out;
  }

  static mapAlphabet(alphabet) {
    if (alphabet.length > UTF8Encoder.MAX_CHARS) {
      throw new Error(`Alphabet is bigger than ${UTF8Encoder.MAX_CHARS} characters`);
    }
    const ranges = UTF8Encoder.PRINTABLE_RANGES.map(([lo, hi]) => [lo, hi]);
    const mapping = {};
    for (const ch of alphabet) {
      const r = ranges[0];
      mapping[ch] = String.fromCharCode(r[0]);
      r[0]++;
      if (r[0] > r[1]) {
        ranges.shift();
      }
    }
    return mapping;
  }

  static optimizeAlphabet(alphabet, optimizePreference) {
    const weights = {};
    optimizePreference.forEach((lang, i) => {
      const k = 10000 ** -i;
      const freq = UTF8Encoder.ALPH_FREQ[lang];
      for (const [ch, w] of Object.entries(freq)) {
        weights[ch] = w * k;
        weights[ch.toLowerCase()] = w * 100 * k;
      }
    });
    return [...alphabet]
      .sort((x, y) => (weights[y] ?? 0) - (weights[x] ?? 0))
      .join("");
  }
}

export class Base64Encoder {
  constructor(alphabet, compress = false) {
    this.alphabet = alphabet;
    this.compress = compress;
    this.charToIndex = Object.fromEntries(
      [...alphabet].map((ch, i) => [ch, i]),
    );
    this.indexToChar = Object.fromEntries(
      [...alphabet].map((ch, i) => [i, ch]),
    );
  }

  encode(text) {
    const payload = new Uint8Array(text.length);
    let i = 0;
    for (const ch of text) {
      if (!(ch in this.charToIndex)) {
        throw new Error(`Character not in alphabet: ${JSON.stringify(ch)}`);
      }
      payload[i++] = this.charToIndex[ch];
    }
    let bytes = payload;
    if (this.compress) {
      bytes = zlibSync(bytes, { level: 9 });
    }
    return bytesToBase64(bytes);
  }

  decode(encoded) {
    let payload = base64ToBytes(encoded.replace(/\s/g, ""));
    if (this.compress) {
      payload = unzlibSync(payload);
    }
    let out = "";
    for (let j = 0; j < payload.length; j++) {
      const b = payload[j];
      if (!(b in this.indexToChar)) {
        throw new Error(`Byte out of alphabet range: ${b}`);
      }
      out += this.indexToChar[b];
    }
    return out;
  }
}

export class Base91Encoder {
  constructor(compress = false) {
    this.compress = compress;
  }

  encode(text) {
    const te = new TextEncoder();
    let payload = te.encode(text);
    if (this.compress) {
      payload = zlibSync(payload, { level: 9 });
    }
    return base91Encode(payload);
  }

  decode(encoded) {
    let payload = base91Decode(encoded);
    if (this.compress) {
      payload = unzlibSync(payload);
    }
    return new TextDecoder().decode(payload);
  }
}

export class Base85Encoder {
  constructor(compress = false) {
    this.compress = compress;
  }

  encode(text) {
    const te = new TextEncoder();
    let payload = te.encode(text);
    if (this.compress) {
      payload = zlibSync(payload, { level: 9 });
    }
    return b85encode(payload, false);
  }

  decode(encoded) {
    let payload = b85decode(encoded);
    if (this.compress) {
      payload = unzlibSync(payload);
    }
    return new TextDecoder().decode(payload);
  }
}

const MARKER_CHARS = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

export class DeciderEncoder {
  /**
   * @param {string} _alphabet unused (parity with Python)
   * @param {object[]} encoders child encoders in fixed order
   */
  constructor(_alphabet, encoders) {
    if (encoders.length > MARKER_CHARS.length) {
      throw new Error("Too many encoders");
    }
    this.encoders = encoders;
    this.encoderToMarker = new Map();
    this.markerToEncoder = new Map();
    encoders.forEach((enc, idx) => {
      const m = MARKER_CHARS[idx];
      this.encoderToMarker.set(enc, m);
      this.markerToEncoder.set(m, enc);
    });
  }

  encode(text) {
    const te = new TextEncoder();
    const candidates = this.encoders.map((encoder) => {
      const encodedText = encoder.encode(text);
      return {
        encoder,
        markerChar: this.encoderToMarker.get(encoder),
        encoded: encodedText,
        size: te.encode(encodedText).length,
      };
    });
    candidates.sort((a, b) => a.size - b.size);
    const best = candidates[0];
    return best.markerChar + best.encoded;
  }

  decode(encoded) {
    const markerChar = encoded[0];
    const encodedText = encoded.slice(1);
    const encoder = this.markerToEncoder.get(markerChar);
    if (!encoder) {
      throw new Error(`Unknown decider marker: ${JSON.stringify(markerChar)}`);
    }
    return encoder.decode(encodedText);
  }
}

const te = new TextEncoder();

export function buildEncoders() {
  const utf8 = new UTF8Encoder(ALPHABET);
  const utf8_optimize = new UTF8Encoder(ALPHABET, ["cyrillic", "english"]);
  const base64 = new Base64Encoder(ALPHABET, false);
  const base64_compress = new Base64Encoder(ALPHABET, true);
  const base91 = new Base91Encoder(false);
  const base91_compress = new Base91Encoder(true);
  const base85 = new Base85Encoder(false);
  const base85_compress = new Base85Encoder(true);
  const decider = new DeciderEncoder(ALPHABET, [
    utf8,
    utf8_optimize,
    base64,
    base64_compress,
    base91,
    base91_compress,
    base85,
    base85_compress,
  ]);
  return {
    decider,
    utf8,
    utf8_optimize,
    base64,
    base64_compress,
    base91,
    base91_compress,
    base85,
    base85_compress,
  };
}

export function utf8ByteLength(s) {
  return te.encode(s).length;
}
