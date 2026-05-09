from dataclasses import dataclass
import subprocess
import resource
from pathlib import Path
from typing import Union

UNIPRESS_BINARY = Path(__file__).parent.parent.parent / 'rust/target/release/unipress'

@dataclass
class EncodingResult:
  payload: str
  encoded_payload: str
  compression_rate: float
  user_time: float

@dataclass
class DecodingResult:
  payload: str
  decoded_payload: str
  user_time: float

def _call_unipress(payload: str, encoder: str, decode: bool = False) -> tuple[str, str, float, float]:
  decode_flag = ['--decode'] if decode else []
  before = resource.getrusage(resource.RUSAGE_CHILDREN)
  try:
    result = subprocess.run(
      [UNIPRESS_BINARY, "--encoder", encoder] + decode_flag,
      input=payload,
      text=True,
      capture_output=True,
      check=True,
    )
    stdout = result.stdout.strip()
  except subprocess.CalledProcessError as e:
    raise Exception(f"Failed to call unipress: {e.stderr}\npayload: {payload} (len={len(payload)})")
  after = resource.getrusage(resource.RUSAGE_CHILDREN)
  user_time = after.ru_utime - before.ru_utime
  result_len, payload_len = len(stdout.encode('utf-8')), len(payload.encode('utf-8'))
  compression_rate = (payload_len - result_len) / payload_len if payload_len > 0 else 0
  return payload, stdout, compression_rate, user_time

def encode(payload: str, encoder: str) -> EncodingResult:
  payload, encoded_payload, compression_rate, user_time = _call_unipress(payload, encoder)
  return EncodingResult(payload, encoded_payload, compression_rate, user_time)

def decode(payload: str, encoder: str) -> DecodingResult:
  payload, decoded_payload, _, user_time = _call_unipress(payload, encoder, decode=True)
  return DecodingResult(payload, decoded_payload, user_time)
