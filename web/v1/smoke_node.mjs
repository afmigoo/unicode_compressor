import { b85encode, b85decode, base91Encode, base91Decode, buildEncoders, ALPHABET } from "./js/encoders.js";
import { randomBytes, randomInt } from "crypto";

for (let n = 0; n < 30; n++) {
  const u = new Uint8Array(randomBytes(n));
  const s = b85encode(u);
  const d = b85decode(s);
  if (Buffer.from(d).compare(Buffer.from(u)) !== 0) {
    console.error("b85 fail", n);
    process.exit(1);
  }
}
for (let n = 0; n < 30; n++) {
  const u = new Uint8Array(randomBytes(n));
  const s = base91Encode(u);
  const d = base91Decode(s);
  if (Buffer.from(d).compare(Buffer.from(u)) !== 0) {
    console.error("b91 fail", n);
    process.exit(1);
  }
}

const cases = [
  "",
  "a",
  "hello 123",
  "лтфлстзлфытьсзфь",
  "съешь ещё этих мягких французских булок, да выпей чаю",
  "brown fox jumps over the lazy dog",
  "Test: !@# with mixed АБВ and xyz.",
];

/** Same as prototype/test_encoders.py: choices(ALPHABET, k=n) for n in range(1000) */
const alphabetChars = [...ALPHABET];
for (let n = 0; n < 1000; n++) {
  let s = "";
  for (let i = 0; i < n; i++) {
    s += alphabetChars[randomInt(alphabetChars.length)];
  }
  cases.push(s);
}

const encoders = buildEncoders();
for (const [name, enc] of Object.entries(encoders)) {
  for (const text of cases) {
    for (const ch of text) {
      if (!ALPHABET.includes(ch)) {
        throw new Error(`bad fixture char ${ch}`);
      }
    }
    const got = enc.decode(enc.encode(text));
    if (got !== text) {
      console.error("round-trip fail", name, JSON.stringify(text), JSON.stringify(got));
      process.exit(1);
    }
  }
}
console.log("smoke ok");
