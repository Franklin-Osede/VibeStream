#!/bin/bash

# 🚀 VibeStream - Inicio Rápido
echo "🚀 Iniciando VibeStream completo..."

# Dar permisos de ejecución a los scripts
chmod +x scripts/dev-start.sh
chmod +x scripts/dev-stop.sh

# Ejecutar script principal
./scripts/dev-start.sh

echo ""
echo "✨ ¡Todo listo! Tu plataforma VibeStream está corriendo"
echo ""
echo "🔗 Links útiles:"
echo "  • API: http://localhost:3002"
echo "  • Health: http://localhost:3002/health"
echo ""
echo "📱 Para la app móvil:"
echo "  npm run ios     # iOS Simulator"
echo "  npm run android # Android"
echo "  npx expo start  # Expo Dev" 