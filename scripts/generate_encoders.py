"""
Generate rust/src/encoders/instances.rs

Lock file must exist for this script to run. It can be created with:
  python3 scripts/lock_encoders.py

usage: python3 scripts/generate_encoders.py
"""
from dict_generator.rustgen import write
from dict_generator.rustgen.encoders import build_encoders
from dict_generator.rustgen.const import INSTANCES_RS, LOCK_FILE

if __name__ == '__main__':
  encoders = build_encoders()
  write.write_rust_encoders(encoders, INSTANCES_RS, LOCK_FILE)
