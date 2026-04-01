#!/usr/bin/env python3
"""Decrypt Fernet-encrypted .key files and print NAME=VALUE pairs.

Usage: FIELD_ENCRYPTION_KEY=xxx python3 decrypt_keys.py /path/to/keys/

Called by start.sh before launching agentgateway.
Outputs lines like: VENICE_API_KEY=sk-actual-key
These are eval'd by start.sh to export as env vars.

If FIELD_ENCRYPTION_KEY is not set, reads key files as plaintext
(backwards compatible with pre-encryption key files).
"""

from __future__ import annotations

import os
import sys
from pathlib import Path

from cryptography.fernet import Fernet


def _name_to_env_var(stem: str) -> str:
    """Convert a key file stem to env var name.

    Examples:
        venice    -> VENICE_API_KEY
        openai    -> OPENAI_API_KEY
        my-model  -> MY_MODEL_API_KEY
    """
    return stem.upper().replace("-", "_").replace(".", "_") + "_API_KEY"


def main() -> None:
    keys_dir = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("keys")

    if not keys_dir.is_dir():
        return

    enc_key = os.environ.get("FIELD_ENCRYPTION_KEY", "")

    if not enc_key:
        print(
            "Warning: FIELD_ENCRYPTION_KEY not set, reading keys as plaintext",
            file=sys.stderr,
        )
        for kf in sorted(keys_dir.glob("*.key")):
            name = _name_to_env_var(kf.stem)
            print(f"export {name}={kf.read_text().strip()}")
        return

    fernet = Fernet(enc_key.encode())
    for kf in sorted(keys_dir.glob("*.key")):
        name = _name_to_env_var(kf.stem)
        try:
            decrypted = fernet.decrypt(kf.read_bytes()).decode()
            print(f"export {name}={decrypted}")
        except Exception as e:
            print(f"Warning: Failed to decrypt {kf.name}: {e}", file=sys.stderr)


if __name__ == "__main__":
    main()
