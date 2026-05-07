from datasets import load_dataset

def raw(file: str | Path):
  ds = load_dataset('json', data_files=str(file))['train']
  for s in ds:
    yield s['text']

def alphabet_filtered(file: str | Path, alphabet: list[str]):
  alphabet_map = set(alphabet)
  for s in raw(file):
    filtered_s = ''
    for ch in s:
      if ch in alphabet_map:
        filtered_s += ch
      elif ch.lower() in alphabet_map:
        filtered_s += ch.lower()
      elif ch.upper() in alphabet_map:
        filtered_s += ch.upper()
    if len(filtered_s) > 0:
      yield filtered_s

