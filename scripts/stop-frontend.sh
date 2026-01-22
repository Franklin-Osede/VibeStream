#!/bin/bash

# 🛑 Script para detener el Frontend

# Colores
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

PROJECT_ROOT="/Users/domoblock/Documents/Proycts-dev/Vibestream"
PID_FILE="$PROJECT_ROOT/.frontend.pid"

if [ ! -f "$PID_FILE" ]; then
    echo -e "${YELLOW}⚠️  Frontend no está corriendo (no se encontró archivo PID)${NC}"
    exit 0
fi

PID=$(cat "$PID_FILE")

if kill -0 "$PID" 2>/dev/null; then
    echo -e "${YELLOW}🛑 Deteniendo Frontend (PID: $PID)...${NC}"
    kill "$PID"
    sleep 2
    
    # Verificar si se detuvo
    if kill -0 "$PID" 2>/dev/null; then
        echo -e "${RED}⚠️  Forzando detención...${NC}"
        kill -9 "$PID"
    fi
    
    echo -e "${GREEN}✅ Frontend detenido${NC}"
else
    echo -e "${YELLOW}⚠️  Proceso no encontrado (limpiando archivo PID)${NC}"
fi

rm -f "$PID_FILE"
