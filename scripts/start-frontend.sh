#!/bin/bash

# 🚀 Script para iniciar el Frontend (Angular)
# Puerto: 4207

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

PROJECT_ROOT="/Users/domoblock/Documents/Proycts-dev/Vibestream"
FRONTEND_DIR="$PROJECT_ROOT/apps/frontend"
PID_FILE="$PROJECT_ROOT/.frontend.pid"
PORT=4207

cd "$FRONTEND_DIR" || exit 1

echo -e "${BLUE}🚀 Iniciando Frontend (Angular)...${NC}"

# Verificar si ya está corriendo
if [ -f "$PID_FILE" ]; then
    OLD_PID=$(cat "$PID_FILE")
    if kill -0 "$OLD_PID" 2>/dev/null; then
        echo -e "${YELLOW}⚠️  Frontend ya está corriendo (PID: $OLD_PID)${NC}"
        echo -e "${GREEN}   URL: http://localhost:$PORT${NC}"
        exit 0
    else
        rm -f "$PID_FILE"
    fi
fi

# Verificar que npm esté instalado
if ! command -v npm &> /dev/null; then
    echo -e "${RED}❌ npm no encontrado. Instala Node.js primero.${NC}"
    exit 1
fi

# Setup Env
if [ ! -d "node_modules" ]; then
    echo -e "${BLUE}📦 Instalando dependencias...${NC}"
    npm install
fi

# Iniciar el frontend
echo -e "${BLUE}🅰️  Iniciando servidor de desarrollo Angular en puerto $PORT...${NC}"
npm start -- --port $PORT > "$PROJECT_ROOT/logs/frontend.log" 2>&1 &
FRONTEND_PID=$!

# Guardar PID
echo $FRONTEND_PID > "$PID_FILE"

# Esperar a que inicie
echo -e "${BLUE}⏳ Esperando a que el frontend esté listo...${NC}"
sleep 5

# Verificar que sigue corriendo
if kill -0 "$FRONTEND_PID" 2>/dev/null; then
    echo -e "${GREEN}✅ Frontend iniciado correctamente${NC}"
    echo -e "${GREEN}   PID: $FRONTEND_PID${NC}"
    echo -e "${GREEN}   Puerto: $PORT${NC}"
    echo -e "${BLUE}   URL: http://localhost:$PORT${NC}"
    echo ""
    echo -e "${YELLOW}💡 Logs: tail -f $PROJECT_ROOT/logs/frontend.log${NC}"
    echo -e "${YELLOW}💡 Detener: ./scripts/stop-frontend.sh${NC}"
else
    echo -e "${RED}❌ Error iniciando el frontend${NC}"
    echo -e "${YELLOW}   Revisa los logs: cat $PROJECT_ROOT/logs/frontend.log${NC}"
    rm -f "$PID_FILE"
    exit 1
fi
