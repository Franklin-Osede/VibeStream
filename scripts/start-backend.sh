#!/bin/bash

# 🚀 Script para iniciar el Backend (API Gateway Unificado)
# Puerto: 3007

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

PROJECT_ROOT="/Users/domoblock/Documents/Proycts-dev/Vibestream"
BACKEND_DIR="$PROJECT_ROOT/services/api-gateway"
PID_FILE="$PROJECT_ROOT/.backend.pid"
# IMPORTANT: This export is needed if the Rust code reads it.
export PORT=3007

cd "$BACKEND_DIR" || exit 1

echo -e "${BLUE}🚀 Iniciando Backend (API Gateway Unificado)...${NC}"

# Verificar si ya está corriendo
if [ -f "$PID_FILE" ]; then
    OLD_PID=$(cat "$PID_FILE")
    if kill -0 "$OLD_PID" 2>/dev/null; then
        echo -e "${YELLOW}⚠️  Backend ya está corriendo (PID: $OLD_PID)${NC}"
        echo -e "${GREEN}   URL: http://localhost:$PORT${NC}"
        exit 0
    else
        rm -f "$PID_FILE"
    fi
fi

# Verificar que cargo esté instalado
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Cargo no encontrado. Instala Rust primero.${NC}"
    exit 1
fi

# Iniciar el backend
echo -e "${BLUE}📦 Compilando y ejecutando api-gateway-unified en puerto $PORT...${NC}"
# Usamos el binario unificado recomendado
cargo run --bin api-gateway-unified > "$PROJECT_ROOT/logs/backend.log" 2>&1 &
BACKEND_PID=$!

# Guardar PID
echo $BACKEND_PID > "$PID_FILE"

# Esperar a que inicie
echo -e "${BLUE}⏳ Esperando a que el backend esté listo...${NC}"
sleep 5

# Verificar que sigue corriendo
if kill -0 "$BACKEND_PID" 2>/dev/null; then
    echo -e "${GREEN}✅ Backend iniciado correctamente${NC}"
    echo -e "${GREEN}   PID: $BACKEND_PID${NC}"
    echo -e "${GREEN}   Puerto: $PORT${NC}"
    echo -e "${BLUE}   API URL: http://localhost:$PORT${NC}"
    echo -e "${BLUE}   Health: http://localhost:$PORT/health${NC}"
    echo -e "${BLUE}   Swagger: http://localhost:$PORT/swagger-ui${NC}"
    echo ""
    echo -e "${YELLOW}💡 Logs: tail -f $PROJECT_ROOT/logs/backend.log${NC}"
    echo -e "${YELLOW}💡 Detener: ./scripts/stop-backend.sh${NC}"
else
    echo -e "${RED}❌ Error iniciando el backend${NC}"
    echo -e "${YELLOW}   Revisa los logs: cat $PROJECT_ROOT/logs/backend.log${NC}"
    rm -f "$PID_FILE"
    exit 1
fi
