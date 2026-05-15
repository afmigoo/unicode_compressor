from pathlib import Path
from typing import Literal
from jinja2 import Template

def rust_str_lit(s: str, quoted: bool = True) -> str:
  escaped = (
    s.replace("\\", "\\\\")
    .replace("'", "\\'")
    .replace("\"", "\\\"")
    .replace("\n", "\\n")
    .replace("\r", "\\r")
    .replace("\t", "\\t")
    .replace("\0", "\\0")
  )
  return f"\"{escaped}\"" if quoted else escaped

def write_rust_dict(
  tokens: dict,
  file: str | Path, 
):
  token_max_chars = max(len(tkn) for tkn in tokens.keys())

  token2encoded = {
    rust_str_lit(str(k), True): rust_str_lit(str(v), False)
    for k, v in tokens.items()
  }
  
  with open(file, 'w', encoding='utf-8') as f:
    template = Template(
      open(Path(__file__).parent / 'dict.j2', 'r', encoding='utf-8').read(),
      trim_blocks=True,
      lstrip_blocks=True,
    )
    f.write(template.render(
      token2encoded=token2encoded, 
      token_max_chars=token_max_chars
    ))

def write_rust_encoders(
  encoders: list[dict],
  file: str | Path,
  adaptive_chars: list[str],
):
  with open(file, 'w', encoding='utf-8') as f:
    template = Template(
      open(Path(__file__).parent / 'instances.j2', 'r', encoding='utf-8').read(),
      trim_blocks=True,
      lstrip_blocks=True,
    )
    f.write(template.render(
      encoders=encoders,
      adaptive_chars=[rust_str_lit(ch, False) for ch in adaptive_chars],
      token_max_chars=16,
    ))

def write_rust_char_dict(
  char2int: dict,
  file: str | Path,
):
  with open(file, 'w', encoding='utf-8') as f:
    template = Template(
      open(Path(__file__).parent / 'int2utf.j2', 'r', encoding='utf-8').read(),
      trim_blocks=True,
      lstrip_blocks=True,
    )
    escaped_char2int = {
      rust_str_lit(str(k), True): v
      for k, v in char2int.items()
    }
    f.write(template.render(char2int=escaped_char2int))
