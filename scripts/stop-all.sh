#!/bin/bash

# 🛑 Script para detener Backend y Frontend

# Colores
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

PROJECT_ROOT="/Users/domoblock/Documents/Proycts-dev/Vibestream"

cd "$PROJECT_ROOT" || exit 1

echo -e "${BLUE}╔════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   🛑 Deteniendo VibeStream Completo   ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════╝${NC}"
echo ""

# Detener Backend
echo -e "${YELLOW}[1/2] Deteniendo Backend...${NC}"
./scripts/stop-backend.sh

echo ""

# Detener Frontend
echo -e "${YELLOW}[2/2] Deteniendo Frontend...${NC}"
./scripts/stop-frontend.sh

echo ""
echo -e "${GREEN}✅ Todos los servicios detenidos${NC}"
echo ""
