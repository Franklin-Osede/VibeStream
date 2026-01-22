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
PORT=3007

cd "$PROJECT_ROOT" || exit 1

echo -e "${BLUE}🚀 Iniciando Vibestream Backend...${NC}"

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

# Iniciar backend (api-gateway)
# Verificar si cargo está instalado, si no, intentar usar docker-compose o avisar
if command -v cargo >/dev/null 2>&1; then
    echo -e "${BLUE}📦 Ejecutando con Cargo...${NC}"
    cd services/api-gateway || exit 1
    # Asegurar que se usen las variables de entorno correctas o el .env
    SERVER_PORT=3007 cargo run --bin api-gateway-unified > /dev/null 2>&1 &
    SERVER_PID=$!
    cd ../..
else
     echo -e "${YELLOW}⚠️ Cargo no encontrado. Intentando Docker Compose...${NC}"
     docker-compose up -d api-gateway
     # Nota: Esto no nos da el PID fácilmente para el archivo .pid, pero es un fallback.
     # Por simplicidad en este script, asumiremos entorno de desarrollo con Rust instalado.
     echo -e "${RED}❌ Rust/Cargo no encontrado. Por favor instala Rust o usa docker-compose manualmente.${NC}"
     exit 1
fi

# Guardar PID
echo $SERVER_PID > "$PID_FILE"

# Esperar un momento para verificar que el servidor inició
sleep 5

# Verificar que el proceso sigue corriendo
if kill -0 "$SERVER_PID" 2>/dev/null; then
    echo -e "${GREEN}✅ Backend iniciado correctamente${NC}"
    echo -e "${GREEN}   PID: $SERVER_PID${NC}"
    echo -e "${GREEN}   Puerto: $PORT${NC}"
    echo -e "${BLUE}   API URL: http://localhost:$PORT${NC}"
    echo ""
    echo -e "${BLUE}🚀 Para iniciar el Frontend:${NC}"
    echo -e "   cd apps/frontend && npm run dev"
    echo ""
    echo -e "${YELLOW}💡 Para detener el backend: ./scripts/dev-stop.sh${NC}"
else
    echo -e "${RED}❌ Error iniciando el servidor (Revisa logs/ o salida de cargo)${NC}"
    rm -f "$PID_FILE"
    exit 1
fi
