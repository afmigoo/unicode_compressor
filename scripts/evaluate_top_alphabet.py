from collections import Counter

from dict_generator.dataset import DATASETS
from dict_generator import alphabet

PRINT_TOP_N = 255
MIN_LETTERS = {
    'ru': alphabet.ru.RU_32,
    'en': alphabet.en.EN_32,
}

if __name__ == '__main__':
  for ds in DATASETS:
    print(f"Evaluating {ds}...")
    top_alphabet = Counter()
    for payload in ds.raw('train'):
        top_alphabet.update(payload)

    top_letters = [x for x, _ in top_alphabet.most_common(PRINT_TOP_N)]
    missing_min_letters = set(MIN_LETTERS[ds.lang]).difference(set(top_letters))

    # assert len(missing_min_letters) == 0, \
    #     f"Some letters are not in the min letters: {missing_min_letters}"

    print("missing_min_letters:", missing_min_letters)
    print(sorted(top_letters))
