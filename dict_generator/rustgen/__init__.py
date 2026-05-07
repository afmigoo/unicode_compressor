from pathlib import Path

def rust_str_lit(s: str) -> str:
    escaped = (
        s.replace("\\", "\\\\")
        .replace("\"", "\\\"")
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
        .replace("\0", "\\0")
    )
    return f"\"{escaped}\""

def write(tokens: dict, file: str | Path):
  token_max_chars = max(len(tkn) for tkn in tokens.keys())
  
  with open(file, 'w', encoding='utf-8') as f:
    f.write("use phf::{phf_map, Map};\n\n")
    f.write(f"pub const TOKEN_MAX_CHARS: u8 = {token_max_chars};\n\n")
    
    # token to unicode
    f.write(f"pub static TOKEN2UNICODE: Map<&'static str, &'static str> = phf_map! {{\n")
    for token, v in tokens.items():
        f.write(f"  {rust_str_lit(token)} => {rust_str_lit(v)},\n")
    f.write("};\n\n")
    
    # unicode to token 
    f.write(f"pub static UNICODE2TOKEN: Map<&'static str, &'static str> = phf_map! {{\n")
    for token, v in tokens.items():
        f.write(f"  {rust_str_lit(v)} => {rust_str_lit(token)},\n")
    f.write("};\n")

