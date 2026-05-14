#!/bin/bash

set -eo pipefail

rm -f rust/src/encoders/dictionaries/*

python3 scripts/generate_static_dicts.py
python3 scripts/generate_encoders.py

cd rust && cargo build --release && cd ..

python3 scripts/test_roundtrip.py
python3 scripts/evaluate_metrics.py | tee metrics.txt
