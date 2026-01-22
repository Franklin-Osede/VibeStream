#!/bin/bash

# 🚀 Script maestro para levantar todo el entorno
# - Backend (3007)
# - Frontend (4207)
# - MCP Server (Check)

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

PROJECT_ROOT="/Users/domoblock/Documents/Proycts-dev/Vibestream"

echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"
echo -e "${BLUE}   🚀 VIBESTREAM - INICIO INTELIGENTE DEL ENTORNO${NC}"
echo -e "${BLUE}════════════════════════════════════════════════════════════${NC}"

# 0. Limpieza previa
echo -e "${YELLOW}🧹 Limpiando procesos antiguos...${NC}"
$PROJECT_ROOT/scripts/stop-backend.sh > /dev/null 2>&1
$PROJECT_ROOT/scripts/stop-frontend.sh > /dev/null 2>&1

mkdir -p "$PROJECT_ROOT/logs"

# 1. Verificar MCP Server
echo -e "${BLUE}🔍 Verificando Project Tracker MCP...${NC}"
if [ ! -f "$PROJECT_ROOT/tools/project-tracker/build/index.js" ]; then
    echo -e "${YELLOW}⚠️  MCP Server no construido. Construyendo...${NC}"
    cd "$PROJECT_ROOT/tools/project-tracker" && npm install && npm run build
    cd "$PROJECT_ROOT"
else
     echo -e "${GREEN}✅ MCP Server listo.${NC}"
fi

# 2. Iniciar Backend (Unified @ 3007)
echo -e "\n${BLUE}👉 Iniciando Backend (Unified)...${NC}"
$PROJECT_ROOT/scripts/start-backend.sh

# 3. Iniciar Frontend (Angular @ 4207)
echo -e "\n${BLUE}👉 Iniciando Frontend (Angular)...${NC}"
$PROJECT_ROOT/scripts/start-frontend.sh

# Resumen Final
echo -e "\n${GREEN}════════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ ENTORNO LEVANTADO CON ÉXITO${NC}"
echo -e "${BLUE}   🔌 Backend:  http://localhost:3007${NC}"
echo -e "${BLUE}   💻 Frontend: http://localhost:4207${NC}"
echo -e "${BLUE}   🧠 MCP:      Instalado y localmente disponible${NC}"
echo -e "${GREEN}════════════════════════════════════════════════════════════${NC}"
