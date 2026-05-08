from pathlib import Path
from typing import Literal

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

def write(tokens: dict, file: str | Path, encoded_type: Literal['bin', 'utf8']):
  token_max_chars = max(len(tkn) for tkn in tokens.keys())
  is_quoted = encoded_type == 'utf8'
  rust_type = {'bin': 'u16', 'utf8': 'str'}[encoded_type]
  type_suffix = {'bin': 'u16', 'utf8': ''}[encoded_type]
  var_name = {'bin': 'INT', 'utf8': 'UNICODE'}[encoded_type]
  
  with open(file, 'w', encoding='utf-8') as f:
    f.write("use phf::{phf_map, Map};\n\n")
    f.write(f"pub const TOKEN_MAX_CHARS: u8 = {token_max_chars};\n\n")
    
    # token to unicode
    f.write(f"pub static TOKEN2{var_name}: Map<&'static str, &'static {rust_type}> = phf_map! {{\n")
    for token, v in tokens.items():
        f.write(f"  {rust_str_lit(str(token))} => {rust_str_lit(str(v), is_quoted)}{type_suffix},\n")
    f.write("};\n\n")
    
    # unicode to token 
    f.write(f"pub static {var_name}2TOKEN: Map<&'static {rust_type}, &'static str> = phf_map! {{\n")
    for token, v in tokens.items():
        f.write(f"  {rust_str_lit(str(v), is_quoted)}{type_suffix} => {rust_str_lit(str(token))},\n")
    f.write("};\n")
