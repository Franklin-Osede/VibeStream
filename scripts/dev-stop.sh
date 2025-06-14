#!/bin/bash

# VibeStream Development Stop Script
echo "🛑 Deteniendo VibeStream..."

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para detener un servicio
stop_service() {
    local service_name=$1
    local pid_file="logs/$service_name.pid"
    
    if [ -f "$pid_file" ]; then
        local pid=$(cat "$pid_file")
        if kill -0 "$pid" 2>/dev/null; then
            echo -e "${YELLOW}🔄 Deteniendo $service_name (PID: $pid)...${NC}"
            kill "$pid"
            sleep 2
            
            # Verificar si el proceso aún existe
            if kill -0 "$pid" 2>/dev/null; then
                echo -e "${RED}⚠️  Forzando detención de $service_name...${NC}"
                kill -9 "$pid"
            fi
            
            echo -e "${GREEN}✅ $service_name detenido${NC}"
        else
            echo -e "${YELLOW}⚠️  $service_name ya estaba detenido${NC}"
        fi
        rm -f "$pid_file"
    else
        echo -e "${YELLOW}⚠️  No se encontró PID para $service_name${NC}"
    fi
}

# Detener servicios
echo -e "${BLUE}🛑 Deteniendo servicios...${NC}"

stop_service "api-gateway"
stop_service "ethereum-service"
stop_service "zk-service"

# Detener Redis
echo -e "${YELLOW}🔄 Deteniendo Redis...${NC}"
redis-cli shutdown 2>/dev/null || echo -e "${YELLOW}⚠️  Redis ya estaba detenido${NC}"

# Limpiar archivos temporales
echo -e "${BLUE}🧹 Limpiando archivos temporales...${NC}"
rm -f logs/*.pid

echo ""
echo -e "${GREEN}✅ ¡Todos los servicios detenidos exitosamente!${NC}"
echo -e "${BLUE}📝 Los logs se mantienen en el directorio logs/${NC}" 