#!/bin/bash

# Ottieni la data corrente
CURRENT_DATE=$(date +"%Y-%m-%d")

# Ottieni l'ultimo tag o usa l'inizio del repository
LAST_TAG=$(git describe --tags --abbrev=0 2>/dev/null || echo "inizio")

# Crea un file temporaneo per il nuovo changelog
echo "# Changelog" > CHANGELOG.md.new
echo "" >> CHANGELOG.md.new
echo "## [$CURRENT_DATE]" >> CHANGELOG.md.new
echo "" >> CHANGELOG.md.new

# Ottieni i commit tra l'ultimo tag e HEAD
git log $LAST_TAG..HEAD --pretty=format:"- %s (%h) - %an" >> CHANGELOG.md.new

echo "" >> CHANGELOG.md.new
echo "---" >> CHANGELOG.md.new
echo "" >> CHANGELOG.md.new

# Se esiste un CHANGELOG.md precedente, mantieni solo la parte dopo la prima sezione
if [ -f CHANGELOG.md ]; then
    # Conta quante righe saltare (fino alla prima linea con "---")
    LINES_TO_SKIP=$(grep -n "^---" CHANGELOG.md | head -1 | cut -d: -f1)
    
    if [ ! -z "$LINES_TO_SKIP" ]; then
        # Aggiungi il contenuto del changelog precedente dopo la riga con "---"
        tail -n +$((LINES_TO_SKIP + 1)) CHANGELOG.md >> CHANGELOG.md.new
    fi
fi

# Sostituisci il vecchio changelog con quello nuovo
mv CHANGELOG.md.new CHANGELOG.md

echo "Changelog generato con successo!" 