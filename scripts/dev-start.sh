#!/bin/bash

# Script de inicio simplificado para gestión de memoria
# Instala dependencias si no existen y levanta el servidor

# Colores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
RED='\033[0;31m'
NC='\033[0m' # No Color

PROJECT_ROOT="/Users/domoblock/Documents/Proycts-dev/Vibestream"
PID_FILE="$PROJECT_ROOT/.pid"
PORT=3000

cd "$PROJECT_ROOT" || exit 1

echo -e "${BLUE}🚀 Iniciando Vibestream...${NC}"

# Verificar si el servidor ya está corriendo
if [ -f "$PID_FILE" ]; then
    OLD_PID=$(cat "$PID_FILE")
    if kill -0 "$OLD_PID" 2>/dev/null; then
        echo -e "${YELLOW}⚠️  El servidor ya está corriendo (PID: $OLD_PID)${NC}"
        echo -e "${YELLOW}   Usa ./scripts/dev-stop.sh para detenerlo primero${NC}"
        exit 1
    else
        # PID file existe pero el proceso no, limpiar
        rm -f "$PID_FILE"
    fi
fi

# Verificar si node_modules existe
if [ ! -d "node_modules" ]; then
    echo -e "${YELLOW}📦 Instalando dependencias...${NC}"
    npm install
    if [ $? -ne 0 ]; then
        echo -e "${RED}❌ Error instalando dependencias${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ Dependencias instaladas${NC}"
else
    echo -e "${GREEN}✅ Dependencias ya instaladas${NC}"
fi

# Iniciar servidor
echo -e "${BLUE}🔄 Iniciando servidor en puerto $PORT...${NC}"
npm run dev > /dev/null 2>&1 &
SERVER_PID=$!

# Guardar PID
echo $SERVER_PID > "$PID_FILE"

# Esperar un momento para verificar que el servidor inició
sleep 2

# Verificar que el proceso sigue corriendo
if kill -0 "$SERVER_PID" 2>/dev/null; then
    echo -e "${GREEN}✅ Servidor iniciado correctamente${NC}"
    echo -e "${GREEN}   PID: $SERVER_PID${NC}"
    echo -e "${GREEN}   Puerto: $PORT${NC}"
    echo -e "${BLUE}   URL: http://localhost:$PORT${NC}"
    echo ""
    echo -e "${YELLOW}💡 Para detener: ./scripts/dev-stop.sh${NC}"
else
    echo -e "${RED}❌ Error iniciando el servidor${NC}"
    rm -f "$PID_FILE"
    exit 1
fi
