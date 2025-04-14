#!/bin/bash

# Controllo se è stato fornito un argomento per la versione
if [ $# -ne 1 ]; then
    echo "Utilizzo: $0 <versione>"
    echo "Esempio: $0 1.0.0"
    exit 1
fi

VERSION=$1

# Controlla se la versione inizia già con 'v'
if [[ $VERSION == v* ]]; then
    TAG=$VERSION
else
    TAG="v$VERSION"
fi

# Crea il tag
git tag -a $TAG -m "Versione $VERSION"

echo "Tag $TAG creato! Usa 'git push --tags' per pubblicarlo."
echo "Ora puoi eseguire ./build.sh per compilare con la nuova versione." 