#!/bin/bash
# Nettoie les artefacts de build standard
cargo clean

# Nettoie les artefacts de build Linux (créés par build_linux.sh)
rm -rf target_linux

echo "Nettoyage terminé."
