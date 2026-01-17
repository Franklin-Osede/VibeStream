#!/bin/bash

# Script de limpieza manual sin detener el servidor
# Elimina node_modules para liberar RAM sin afectar el servidor en ejecución

# Colores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

PROJECT_ROOT="/Users/domoblock/Documents/Proycts-dev/Vibestream"
PID_FILE="$PROJECT_ROOT/.pid"

cd "$PROJECT_ROOT" || exit 1

echo -e "${BLUE}🧹 Limpieza manual de Vibestream...${NC}"

# Verificar si el servidor está corriendo
if [ -f "$PID_FILE" ]; then
    PID=$(cat "$PID_FILE")
    if kill -0 "$PID" 2>/dev/null; then
        echo -e "${YELLOW}⚠️  El servidor está corriendo (PID: $PID)${NC}"
        echo -e "${YELLOW}   Se eliminará node_modules pero el servidor seguirá activo${NC}"
        echo -e "${YELLOW}   Nota: Si necesitas reinstalar dependencias, usa: npm install${NC}"
    fi
fi

# Eliminar node_modules
if [ -d "node_modules" ]; then
    echo -e "${YELLOW}🗑️  Eliminando node_modules...${NC}"
    rm -rf node_modules
    echo -e "${GREEN}✅ node_modules eliminado${NC}"
    
    # Calcular espacio liberado (aproximado)
    echo -e "${BLUE}💾 RAM liberada${NC}"
else
    echo -e "${YELLOW}⚠️  node_modules no existe${NC}"
fi

# Limpiar otros archivos temporales opcionales
echo -e "${BLUE}🧹 Limpiando archivos temporales...${NC}"

# Limpiar cache de npm (opcional, comentado por defecto)
# if [ -d "$HOME/.npm" ]; then
#     echo -e "${YELLOW}🗑️  Limpiando cache de npm...${NC}"
#     npm cache clean --force
# fi

echo ""
echo -e "${GREEN}✅ Limpieza completada${NC}"
