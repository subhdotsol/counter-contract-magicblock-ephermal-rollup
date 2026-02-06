#!/bin/bash
# Script to patch edition2024 crates in cargo registry

REGISTRY_DIR="$HOME/.cargo/registry/src/index.crates.io-6f17d22bba15001f"

echo "Patching edition2024 crates in cargo registry..."

# Find all Cargo.toml files with edition = "2024" and patch them
find "$REGISTRY_DIR" -name "Cargo.toml" -type f | while read -r file; do
    if grep -q 'edition = "2024"' "$file"; then
        echo "Patching: $file"
        sed -i.bak 's/edition = "2024"/edition = "2021"/' "$file"
    fi
done

echo "Done! All edition2024 references have been changed to edition2021"
