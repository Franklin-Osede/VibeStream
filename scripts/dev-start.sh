#!/bin/bash

# VibeStream Development Startup Script
echo "🚀 Iniciando VibeStream en modo desarrollo..."

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para verificar si un comando existe
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Verificar dependencias
echo -e "${BLUE}📋 Verificando dependencias...${NC}"

if ! command_exists redis-server; then
    echo -e "${RED}❌ Redis no está instalado. Instalando...${NC}"
    if command_exists brew; then
        brew install redis
    else
        echo -e "${RED}❌ Por favor instala Redis manualmente${NC}"
        exit 1
    fi
fi

if ! command_exists cargo; then
    echo -e "${RED}❌ Rust/Cargo no está instalado${NC}"
    exit 1
fi

# Iniciar Redis en background
echo -e "${YELLOW}🔄 Iniciando Redis...${NC}"
redis-server --daemonize yes --port 6379
sleep 2

# Verificar que Redis esté corriendo
if redis-cli ping > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Redis iniciado correctamente${NC}"
else
    echo -e "${RED}❌ Error iniciando Redis${NC}"
    exit 1
fi

# Compilar todos los servicios
echo -e "${BLUE}🔨 Compilando servicios...${NC}"
cargo build --workspace

if [ $? -ne 0 ]; then
    echo -e "${RED}❌ Error compilando servicios${NC}"
    exit 1
fi

echo -e "${GREEN}✅ Todos los servicios compilados correctamente${NC}"

# Crear directorio para logs
mkdir -p logs

# Función para iniciar un servicio
start_service() {
    local service_name=$1
    local service_path=$2
    local port=$3
    
    echo -e "${YELLOW}🔄 Iniciando $service_name...${NC}"
    
    if [ -n "$port" ]; then
        RUST_LOG=info cargo run --bin $service_name > logs/$service_name.log 2>&1 &
        echo $! > logs/$service_name.pid
        sleep 2
        echo -e "${GREEN}✅ $service_name iniciado en puerto $port (PID: $(cat logs/$service_name.pid))${NC}"
    else
        RUST_LOG=info cargo run --bin $service_name > logs/$service_name.log 2>&1 &
        echo $! > logs/$service_name.pid
        sleep 2
        echo -e "${GREEN}✅ $service_name iniciado (PID: $(cat logs/$service_name.pid))${NC}"
    fi
}

# Iniciar servicios
echo -e "${BLUE}🚀 Iniciando servicios...${NC}"

start_service "ethereum-service" "services/ethereum" ""
start_service "zk-service" "services/zk-service" ""
start_service "api-gateway" "services/api-gateway" "3000"

echo ""
echo -e "${GREEN}🎉 ¡VibeStream iniciado exitosamente!${NC}"
echo ""
echo -e "${BLUE}📊 Estado de servicios:${NC}"
echo -e "  • Redis: ${GREEN}✅ Corriendo en puerto 6379${NC}"
echo -e "  • API Gateway: ${GREEN}✅ Corriendo en puerto 3000${NC}"
echo -e "  • Ethereum Service: ${GREEN}✅ Corriendo${NC}"
echo -e "  • ZK Service: ${GREEN}✅ Corriendo${NC}"
echo ""
echo -e "${YELLOW}📝 Logs disponibles en:${NC}"
echo -e "  • logs/api-gateway.log"
echo -e "  • logs/ethereum-service.log"
echo -e "  • logs/zk-service.log"
echo ""
echo -e "${BLUE}🌐 API Gateway disponible en: http://localhost:3000${NC}"
echo -e "${BLUE}🔍 Health check: curl http://localhost:3000/health${NC}"
echo ""
echo -e "${YELLOW}⏹️  Para detener todos los servicios: ./scripts/dev-stop.sh${NC}" 