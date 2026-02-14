import os
import sys
from pathlib import Path

# Add python/ directory to sys.path so `from output import ...` works
sys.path.insert(0, str(Path(__file__).parent.parent))

# Load .env file from project root
env_path = Path(__file__).parent.parent.parent / ".env"
if env_path.exists():
    for line in env_path.read_text().splitlines():
        line = line.strip()
        if line and not line.startswith("#") and "=" in line:
            key, _, value = line.partition("=")
            os.environ.setdefault(key.strip(), value.strip())
