#!/usr/bin/env python3
"""
Assign adaptive encoder prefix characters and write instances.lock.

Existing lock entries are immutable; only new encoders receive unoccupied prefixes.
Use --force to discard the current lock and reassign every encoder from scratch.

usage: python3 scripts/lock_encoders.py [--force]
"""
import argparse

from dict_generator import get_printable_utf8
from dict_generator.rustgen.encoders import build_encoders
from dict_generator.rustgen.const import LOCK_FILE
from dict_generator.rustgen.lock import update_lock_file

if __name__ == '__main__':
  parser = argparse.ArgumentParser(
    description='Assign adaptive encoder prefix characters to instances.lock.',
  )
  parser.add_argument(
    '--force',
    action='store_true',
    help='Overwrite the lock file and reassign all encoders (ignores existing entries).',
  )
  args = parser.parse_args()

  encoders = build_encoders()
  adaptive_chars = get_printable_utf8(1)[1:]

  _, new_assignments = update_lock_file(
    LOCK_FILE, adaptive_chars, encoders, force=args.force,
  )

  if args.force:
    print(f'Overwrote {LOCK_FILE}:')
    for name, prefix in sorted(new_assignments.items()):
      print(f'  {name} -> {prefix!r}')
  elif new_assignments:
    print(f'Updated {LOCK_FILE}:')
    for name, prefix in sorted(new_assignments.items()):
      print(f'  {name} -> {prefix!r}')
  else:
    print(f'Lock file valid, no changes: {LOCK_FILE}')
