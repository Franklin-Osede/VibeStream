#!/bin/bash

# Configurar límites para evitar EMFILE
ulimit -n 10240

# Variables de entorno para optimizar Metro
export EXPO_USE_WATCHMAN=true
export EXPO_NO_WATCHMAN=false
export METRO_MAX_WORKERS=2

# Limpiar cache de Metro
npx expo start --clear

echo "Expo iniciado con configuraciones optimizadas" 