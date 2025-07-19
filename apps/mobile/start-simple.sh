#!/bin/bash

echo "Iniciando Expo sin observación de archivos..."

# Configurar límites
ulimit -n 10240

# Ejecutar Expo sin watch
EXPO_NO_WATCHMAN=true \
METRO_MAX_WORKERS=1 \
npx expo start --clear --no-dev --minify --offline

echo "Expo iniciado en modo offline" 