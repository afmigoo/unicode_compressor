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
