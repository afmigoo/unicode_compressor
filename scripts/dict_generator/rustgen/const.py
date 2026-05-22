from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parent.parent.parent.parent
LOCK_FILE = REPO_ROOT / 'rust/src/encoders/instances.lock'
INSTANCES_RS = REPO_ROOT / 'rust/src/encoders/instances.rs'
