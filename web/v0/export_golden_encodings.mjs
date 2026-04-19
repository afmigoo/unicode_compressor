/**
 * Encode golden plaintext fixtures with every encoder from buildEncoders()
 * and write JSON for manual review / regression snapshots. Frontend-only (Node + web encoders).
 *
 * Usage (from web/v1): node export_golden_encodings.mjs
 */
import { mkdirSync, writeFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

import { ALPHABET, buildEncoders, utf8ByteLength } from "./js/encoders.js";

const __dirname = dirname(fileURLToPath(import.meta.url));
const OUT = join(__dirname, "fixtures", "golden_encodings.json");

/** Short placeholders — replace plaintext with real golden cases; chars must stay in ALPHABET. */
const GOLDEN_CASES = [
  { id: "gs_01", note: "placeholder", plaintext: ALPHABET },
  { id: "gs_02", note: "placeholder", plaintext: ALPHABET.repeat(10) },
  { id: "gs_03", note: "placeholder", plaintext: "brown fox jumps over the lazy dog" },
  { id: "gs_04", note: "placeholder", plaintext: "съешь ещё этих мягких французских булок да выпей чаю" },
  { id: "gs_05", note: "placeholder", plaintext: "hello world" },
  { id: "gs_06", note: "placeholder", plaintext: "привет мир" },
  { id: "gs_07", note: "placeholder", plaintext: "SIGNALIS is a 2022 indie survival horror video game developed by German duo rose-engine (consisting of Yuri Stern and Barbara Wittmann), and published by Humble Games, Balor Games and Playism. It takes place in a sci-fi retro-futuristic dystopian setting where the player follows Elster, a model of sentient android known in-universe as a Replika, as she explores the abandoned underground facility S23-Sierpinski. Its gameplay is primarily inspired by classic survival horror games Silent Hill and Resident Evil, while its art direction takes from creators like Hideaki Anno, Stanley Kubrick and David Lynch. The game involves solving various puzzles and engaging in combat when necessary to progress. It has a total of four known endings." },
  { id: "gs_08", note: "placeholder", plaintext: "Лягушки - общеупотребительное название группы животных из отряда бесхвостых земноводных[1]. В широком смысле термин \"лягушка\" относится ко всем представителям отряда бесхвостых, обладающим увеличенными задними конечностями, которые позволяют им прыгать[2][3]. В узком смысле это название применяется по отношению к представителям семейства настоящих лягушек. Личинки лягушек называются головастиками." },
];

function assertAlphabetOnly(plaintext, caseId) {
  for (const ch of plaintext) {
    if (!ALPHABET.includes(ch)) {
      throw new Error(
        `${caseId}: character not in ALPHABET: ${JSON.stringify(ch)} — encoders only accept ALPHABET text.`,
      );
    }
  }
}

const encoders = buildEncoders();
const encoderNames = Object.keys(encoders);

const cases = GOLDEN_CASES.map(({ id, note, plaintext }) => {
  assertAlphabetOnly(plaintext, id);
  const encoded = {};
  const outUtf8Bytes = {};
  for (const name of encoderNames) {
    const s = encoders[name].encode(plaintext);
    encoded[name] = s;
    outUtf8Bytes[name] = utf8ByteLength(s);
  }
  return {
    id,
    note,
    plaintext,
    plaintextUtf8Bytes: utf8ByteLength(plaintext),
    encoded,
    encodedUtf8Bytes: outUtf8Bytes,
  };
});

const payload = {
  meta: {
    source: "web/v1/export_golden_encodings.mjs",
    encoderNames,
  },
  cases,
};

mkdirSync(dirname(OUT), { recursive: true });
writeFileSync(OUT, JSON.stringify(payload, null, 2) + "\n", "utf8");
console.log(`wrote ${OUT}`);
