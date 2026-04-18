import { b85encode, b85decode, base91Encode, base91Decode, buildEncoders, ALPHABET } from "./js/encoders.js";
import { randomBytes } from "crypto";

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
  "заказала аккумы\nно они не скоро",
  "Test: !@# with mixed АБВ and xyz.",
];
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
