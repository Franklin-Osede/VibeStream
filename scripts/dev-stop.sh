#!/bin/bash

# Script de detención simplificado para gestión de memoria
# Detiene el servidor y elimina node_modules para ahorrar RAM

# Colores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

PROJECT_ROOT="/Users/domoblock/Documents/Proycts-dev/Vibestream"
PID_FILE="$PROJECT_ROOT/.pid"

cd "$PROJECT_ROOT" || exit 1

echo -e "${BLUE}🛑 Deteniendo Vibestream...${NC}"

# Detener servidor si está corriendo
if [ -f "$PID_FILE" ]; then
    PID=$(cat "$PID_FILE")
    
    if kill -0 "$PID" 2>/dev/null; then
        echo -e "${YELLOW}🔄 Deteniendo servidor (PID: $PID)...${NC}"
        kill "$PID" 2>/dev/null
        
        # Esperar a que termine
        sleep 2
        
        # Verificar si aún está corriendo
        if kill -0 "$PID" 2>/dev/null; then
            echo -e "${YELLOW}⚠️  Forzando detención...${NC}"
            kill -9 "$PID" 2>/dev/null
            sleep 1
        fi
        
        echo -e "${GREEN}✅ Servidor detenido${NC}"
    else
        echo -e "${YELLOW}⚠️  El servidor ya estaba detenido${NC}"
    fi
    
    # Eliminar archivo PID
    rm -f "$PID_FILE"
else
    echo -e "${YELLOW}⚠️  No se encontró archivo PID${NC}"
fi

# Eliminar node_modules para ahorrar RAM
if [ -d "node_modules" ]; then
    echo -e "${YELLOW}🧹 Eliminando node_modules para liberar RAM...${NC}"
    rm -rf node_modules
    echo -e "${GREEN}✅ node_modules eliminado${NC}"
else
    echo -e "${YELLOW}⚠️  node_modules no existe${NC}"
fi

echo ""
echo -e "${GREEN}✅ Proceso completado${NC}"
echo -e "${BLUE}💡 RAM liberada. Para reiniciar: ./scripts/dev-start.sh${NC}"
