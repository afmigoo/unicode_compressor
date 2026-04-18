from demo_base64 import Base64Encoder
from demo_utf8 import UTF8Encoder
from demo_base91 import Base91Encoder
from demo_decider import DeciderEncoder
from demo_base85 import Base85Encoder
import os
from string import punctuation

ALPHABET = " \n"
ALPHABET += punctuation
ALPHABET += "0123456789"
ALPHABET += "абвгдеёжзийклмнопрстуфхцчшщъыьэюя"
ALPHABET += "АБВГДЕЁЖЗИЙКЛМНОПРСТУФХЦЧШЩЪЫЬЭЮЯ"
ALPHABET += "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
ALPHABET += "abcdefghijklmnopqrstuvwxyz"


print(f"Alphabet length: {len(ALPHABET)}")

encoders = {
    'utf8': UTF8Encoder(ALPHABET),
    'utf8_optimize': UTF8Encoder(ALPHABET, optimize_preference=['cyrillic', 'english']),
    'base64': Base64Encoder(ALPHABET),
    'base64_compress': Base64Encoder(ALPHABET, compress=True),
    'base91': Base91Encoder(compress=False),
    'base91_compress': Base91Encoder(compress=True),
    'base85': Base85Encoder(compress=False),
    'base85_compress': Base85Encoder(compress=True),
}
encoders['decider'] = DeciderEncoder(ALPHABET, list(encoders.values()))

dataset = [
    """заказала аккумы, но они не скоро придут
в москве не нашла аккумов которые мне подошли бы, с таким маленьким вольтажом и габаритами только на алике есть""",

    """Будет не круто если не будет получаться стабильно сообщения получать""",

    """Мне чета хочется сделать ноду на солнечной панели""",

    """насчет внезапного 
расскажу потом продолжение про ту чувиху, которая пюдоебывала меня че я не работаю на вебке""",

    """
    for encoder_name, encoder in encoders.items():
    print('-' * os.get_terminal_size().columns)
    print(f"\{encoder_name\}:")
    for text in dataset:
        encoded = encoder.encode(text)
        print(f"\{text\} -> \{encoded\}")
        results[encoder_name].append((text, encoded))
        assert encoder.decode(encoded) == text, f"Payload corrupted: \{text\} -> \{encoded\} -> \{encoder.decode(encoded)\}"
    """,

    """
заказала аккумы, но они не скоро придут
в москве не нашла аккумов которые мне подошли бы, с таким маленьким вольтажом и габаритами только на алике есть
    """
]

results = {
    enc: []
    for enc in encoders
}
for encoder_name, encoder in encoders.items():
    print('-' * os.get_terminal_size().columns)
    print(f"{encoder_name}:")
    for text in dataset:
        encoded = encoder.encode(text)
        print(f"{text} -> {encoded}")
        results[encoder_name].append((text, encoded))
        assert encoder.decode(encoded) == text, f"Payload corrupted: {text} -> {encoded} -> {encoder.decode(encoded)}"

comp_results = {
    encoder_name: []
    for encoder_name in encoders
}
for encoder_name, results in results.items():
    size_differences = []
    for text, encoded in results:
        start_size = len(text.encode('utf-8'))
        end_size = len(encoded.encode('utf-8'))
        size_differences.append(end_size / start_size)
    comp_results[encoder_name] = sum(size_differences) / len(size_differences)
    print(f"{encoder_name}: {comp_results[encoder_name]}")
