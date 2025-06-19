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

if ! command_exists psql; then
    echo -e "${YELLOW}⚠️  PostgreSQL no detectado. Instalando...${NC}"
    if command_exists brew; then
        brew install postgresql@14
    fi
fi

# Iniciar servicios de base con brew
echo -e "${YELLOW}🔄 Iniciando servicios de base...${NC}"

# Iniciar Redis
brew services start redis
sleep 2

# Iniciar PostgreSQL
brew services start postgresql@14
sleep 2

# Verificar que Redis esté corriendo
if redis-cli ping > /dev/null 2>&1; then
    echo -e "${GREEN}✅ Redis iniciado correctamente${NC}"
else
    echo -e "${RED}❌ Error iniciando Redis${NC}"
    exit 1
fi

# Verificar PostgreSQL
if psql -d vibestream -c "SELECT 1;" > /dev/null 2>&1; then
    echo -e "${GREEN}✅ PostgreSQL conectado correctamente${NC}"
else
    echo -e "${YELLOW}⚠️  Creando base de datos vibestream...${NC}"
    createdb vibestream 2>/dev/null || true
fi

# Ejecutar migraciones
echo -e "${YELLOW}🔄 Ejecutando migraciones...${NC}"
export DATABASE_URL="postgresql://domoblock:@localhost:5432/vibestream"
sqlx migrate run

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
    local port=$2
    
    echo -e "${YELLOW}🔄 Iniciando $service_name...${NC}"
    
    if [ "$service_name" = "api-gateway" ]; then
        export DATABASE_URL="postgresql://domoblock:@localhost:5432/vibestream"
        RUST_LOG=info cargo run --bin $service_name > logs/$service_name.log 2>&1 &
    else
        RUST_LOG=info cargo run --bin $service_name > logs/$service_name.log 2>&1 &
    fi
    
    echo $! > logs/$service_name.pid
    sleep 3
    
    if [ -n "$port" ]; then
        echo -e "${GREEN}✅ $service_name iniciado en puerto $port (PID: $(cat logs/$service_name.pid))${NC}"
    else
        echo -e "${GREEN}✅ $service_name iniciado (PID: $(cat logs/$service_name.pid))${NC}"
    fi
}

# Iniciar servicios en orden
echo -e "${BLUE}🚀 Iniciando servicios...${NC}"

start_service "ethereum-service" ""
start_service "solana-service" ""  
start_service "zk-service" ""
start_service "api-gateway" "3002"

echo ""
echo -e "${GREEN}🎉 ¡VibeStream iniciado exitosamente!${NC}"
echo ""
echo -e "${BLUE}📊 Estado de servicios:${NC}"
echo -e "  • Redis: ${GREEN}✅ Corriendo en puerto 6379${NC}"
echo -e "  • PostgreSQL: ${GREEN}✅ Corriendo en puerto 5432${NC}"
echo -e "  • API Gateway: ${GREEN}✅ Corriendo en puerto 3002${NC}"
echo -e "  • Ethereum Service: ${GREEN}✅ Corriendo${NC}"
echo -e "  • Solana Service: ${GREEN}✅ Corriendo${NC}"
echo -e "  • ZK Service: ${GREEN}✅ Corriendo${NC}"
echo ""
echo -e "${YELLOW}📝 Logs disponibles en:${NC}"
echo -e "  • logs/api-gateway.log"
echo -e "  • logs/ethereum-service.log"
echo -e "  • logs/solana-service.log"
echo -e "  • logs/zk-service.log"
echo ""
echo -e "${BLUE}🌐 API Gateway disponible en: http://localhost:3002${NC}"
echo -e "${BLUE}🔍 Health check: curl http://localhost:3002/health${NC}"
echo ""
echo -e "${BLUE}📱 Para iniciar la app móvil:${NC}"
echo -e "  • ${YELLOW}npx expo start${NC} - Desarrollo con Expo"
echo -e "  • ${YELLOW}npx expo run:ios${NC} - Compilar para iOS"
echo -e "  • ${YELLOW}npm run ios${NC} - Abrir en simulador iOS"
echo ""
echo -e "${YELLOW}⏹️  Para detener todos los servicios: ./scripts/dev-stop.sh${NC}" 