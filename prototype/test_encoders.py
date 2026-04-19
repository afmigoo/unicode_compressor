import unittest
from string import punctuation
from random import choices

from demo_base64 import Base64Encoder
from demo_base85 import Base85Encoder
from demo_base91 import Base91Encoder
from demo_decider import DeciderEncoder
from demo_utf8 import UTF8Encoder

ALPHABET = " \n"
ALPHABET += punctuation
ALPHABET += "0123456789"
ALPHABET += "абвгдеёжзийклмнопрстуфхцчшщъыьэюя"
ALPHABET += "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"
ALPHABET += "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
ALPHABET += "abcdefghijklmnopqrstuvwxyz"


def _build_encoders():
    encoders = {
        "utf8": UTF8Encoder(ALPHABET),
        "utf8_optimize": UTF8Encoder(ALPHABET, optimize_preference=["cyrillic", "english"]),
        "base64": Base64Encoder(ALPHABET),
        "base64_compress": Base64Encoder(ALPHABET, compress=True),
        "base91": Base91Encoder(ALPHABET, compress=False),
        "base91_compress": Base91Encoder(ALPHABET, compress=True),
        "base85": Base85Encoder(ALPHABET, compress=False),
        "base85_compress": Base85Encoder(ALPHABET, compress=True),
    }
    encoders["decider"] = DeciderEncoder(ALPHABET, list(encoders.values()))
    return encoders


ENCODERS = _build_encoders()

# Payloads must use only ALPHABET so utf8/base64/decider paths stay valid.
_ALPHABET_CASES = [
    "",
    "a",
    "hello 123",
    "лтфлстзлфытьсзфь",
    "заказала аккумы\nно они не скоро",
    "Test: !@# with mixed АБВ and xyz.",
    ALPHABET,
    ALPHABET * 10
]

_ALPHABET_CASES.extend(
    [''.join(choices(ALPHABET, k=n)) for n in range(1000)]
)


class TestEncodersRoundTrip(unittest.TestCase):
    def test_round_trip(self):
        for name, encoder in ENCODERS.items():
            with self.subTest(encoder=name):
                for text in _ALPHABET_CASES:
                    with self.subTest(text=text[:40] + ("…" if len(text) > 40 else "")):
                        self.assertEqual(encoder.decode(encoder.encode(text)), text)


if __name__ == "__main__":
    unittest.main()
