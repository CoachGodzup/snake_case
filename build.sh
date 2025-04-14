#!/bin/bash

# Genera il changelog
./generate_changelog.sh

# Determina la piattaforma corrente
PLATFORM=$(uname)
ARCHITECTURE=$(uname -m)

# Ottieni la versione dall'ultimo tag Git (o usa "dev" se non ci sono tag)
VERSION=$(git describe --tags --abbrev=0 2>/dev/null || echo "dev")
# Rimuovi il "v" iniziale se presente
VERSION=${VERSION#v}

echo "Compilazione versione $VERSION per piattaforma: $PLATFORM su architettura: $ARCHITECTURE"

# Compila per la piattaforma nativa
cargo build --release

# Crea una directory per i binari
mkdir -p releases

# Copia e rinomina il binario con informazioni sulla piattaforma e versione
if [ "$PLATFORM" = "Darwin" ]; then
    # macOS
    cp target/release/snake_case "releases/snake_case_${VERSION}_macos_$ARCHITECTURE"
    echo "Binario creato: releases/snake_case_${VERSION}_macos_$ARCHITECTURE"
elif [ "$PLATFORM" = "Linux" ]; then
    # Linux
    cp target/release/snake_case "releases/snake_case_${VERSION}_linux_$ARCHITECTURE"
    echo "Binario creato: releases/snake_case_${VERSION}_linux_$ARCHITECTURE"
elif [ "$PLATFORM" = "MINGW"* ] || [ "$PLATFORM" = "MSYS"* ] || [ "$PLATFORM" = "CYGWIN"* ]; then
    # Windows
    cp target/release/snake_case.exe "releases/snake_case_${VERSION}_windows_$ARCHITECTURE.exe"
    echo "Binario creato: releases/snake_case_${VERSION}_windows_$ARCHITECTURE.exe"
else
    echo "Piattaforma non riconosciuta: $PLATFORM"
    exit 1
fi

echo "Build completata per $PLATFORM ($ARCHITECTURE), versione $VERSION!"
echo "Per compilare per altre piattaforme, esegui questo script su ogni sistema operativo di destinazione." 