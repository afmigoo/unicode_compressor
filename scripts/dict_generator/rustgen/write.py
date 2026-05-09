from pathlib import Path
from typing import Literal
from jinja2 import Template

def rust_str_lit(s: str, quoted: bool = True) -> str:
  escaped = (
    s.replace("\\", "\\\\")
    .replace("\"", "\\\"")
    .replace("\n", "\\n")
    .replace("\r", "\\r")
    .replace("\t", "\\t")
    .replace("\0", "\\0")
  )
  return f"\"{escaped}\"" if quoted else escaped

def write_rust_struct(
  tokens: dict, file: str | Path, 
  encoded_type: Literal['bin', 'utf8'], 
  encoder_type: Literal['map', 'token'],
):
  token_max_chars = max(len(tkn) for tkn in tokens.keys())
  is_quoted = encoded_type == 'utf8'
  rust_type = {'bin': 'u16', 'utf8': 'str'}[encoded_type]
  type_suffix = {'bin': 'u16', 'utf8': ''}[encoded_type]
  encoder_type = {'map': 'MapEncoder', 'token': 'TokenEncoder'}[encoder_type]

  token2encoded = {
    rust_str_lit(k, is_quoted): f"{rust_str_lit(v, is_quoted)}{type_suffix}" 
    for k, v in tokens.items()
  }
  
  with open(file, 'w', encoding='utf-8') as f:
    template = Template(
      open(Path(__file__).parent / 'encoder.j2', 'r', encoding='utf-8').read(),
      trim_blocks=True,
      lstrip_blocks=True,
    )
    f.write(template.render(
      token2encoded=token2encoded, 
      encoded_type=rust_type,
      encoder_struct=encoder_type, 
      token_max_chars=token_max_chars
    ))
