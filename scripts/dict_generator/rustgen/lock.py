from pathlib import Path
from json import load, dump

def validate_lock(lock_contents: dict, encoders: list[dict]) -> None:
  if len(lock_contents.values()) != len(set(lock_contents.values())):
    raise RuntimeError('Lock file has duplicate prefix values')

  lock_prefixes_unused = set(lock_contents.values())
  lock_encoders_unused = set(lock_contents.keys())

  for encoder in encoders:
    name = encoder['name']
    if name not in lock_contents:
      raise RuntimeError(
        f"Encoder '{name}' missing from lock; run scripts/lock_encoders.py"
      )
    prefix = lock_contents[name]
    lock_prefixes_unused.discard(prefix)
    lock_encoders_unused.discard(name)

  if len(lock_encoders_unused) > 0:
    raise RuntimeError(f'Lock file has encoders that are missing: {lock_encoders_unused}')
  if len(lock_prefixes_unused) > 0:
    raise RuntimeError(f'Lock file has prefixes that are missing: {lock_prefixes_unused}')


def load_lock_file(lock_file: str | Path, encoders: list[dict]) -> dict:
  lock_file = Path(lock_file)
  if not lock_file.exists():
    raise RuntimeError('Lock file not found; run scripts/lock_encoders.py')
  with open(lock_file, 'r', encoding='utf-8') as f:
    lock_contents = load(f)
  validate_lock(lock_contents, encoders)
  return lock_contents


def update_lock_file(
  lock_file: str | Path,
  adaptive_chars: list[str],
  encoders: list[dict],
  force: bool = False,
) -> tuple[dict, dict[str, str]]:
  lock_file = Path(lock_file)
  if force:
    lock_file.unlink(missing_ok=True)

  old_lock: dict = {}
  if lock_file.exists():
    with open(lock_file, 'r', encoding='utf-8') as f:
      old_lock = load(f)

  avail_prefixes = list(set(adaptive_chars).difference(old_lock.values()))
  avail_prefixes.sort(reverse=True)

  result = dict(old_lock)
  new_assignments: dict[str, str] = {}

  for encoder in encoders:
    name = encoder['name']
    if name in old_lock:
      continue
    if not avail_prefixes:
      raise RuntimeError(
        f'No available prefix characters left for encoder {name}'
      )
    prefix = avail_prefixes.pop()
    result[name] = prefix
    new_assignments[name] = prefix

  validate_lock(result, encoders)

  if new_assignments or not lock_file.exists():
    lock_file.parent.mkdir(parents=True, exist_ok=True)
    with open(lock_file, 'w', encoding='utf-8') as f:
      dump(result, f, indent=2, ensure_ascii=False)

  return result, new_assignments
