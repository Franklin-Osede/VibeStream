#!/bin/bash
# =============================================================================
# VIBESTREAM - Script de Setup para Desarrollo
# =============================================================================
# 
# Este script configura el entorno de desarrollo:
# 1. Inicia PostgreSQL y Redis con Docker
# 2. Ejecuta migraciones
# 3. Genera JWT_SECRET
# 4. Crea archivo .env
#
# Uso: ./scripts/setup-dev.sh

set -e  # Salir si hay algún error

echo "🚀 VibeStream - Setup de Desarrollo"
echo "===================================="
echo ""

# Colores para output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# =============================================================================
# 1. Verificar Docker
# =============================================================================
echo "📦 Verificando Docker..."
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker no está instalado. Por favor instala Docker primero.${NC}"
    exit 1
fi

if ! docker ps &> /dev/null; then
    echo -e "${RED}❌ Docker no está corriendo. Por favor inicia Docker.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Docker está disponible${NC}"
echo ""

# =============================================================================
# 2. Iniciar PostgreSQL y Redis
# =============================================================================
echo "🐘 Iniciando PostgreSQL y Redis..."
cd "$(dirname "$0")/.." || exit 1

if docker-compose ps | grep -q "postgres.*Up"; then
    echo -e "${YELLOW}⚠️  PostgreSQL ya está corriendo${NC}"
else
    docker-compose up -d postgres redis
    echo -e "${GREEN}✅ PostgreSQL y Redis iniciados${NC}"
    
    # Esperar a que PostgreSQL esté listo
    echo "⏳ Esperando a que PostgreSQL esté listo..."
    max_attempts=30
    attempt=0
    while [ $attempt -lt $max_attempts ]; do
        if docker-compose exec -T postgres pg_isready -U vibestream &> /dev/null; then
            echo -e "${GREEN}✅ PostgreSQL está listo${NC}"
            break
        fi
        attempt=$((attempt + 1))
        sleep 1
    done
    
    if [ $attempt -eq $max_attempts ]; then
        echo -e "${RED}❌ PostgreSQL no está respondiendo después de $max_attempts intentos${NC}"
        exit 1
    fi
fi
echo ""

# =============================================================================
# 3. Ejecutar Migraciones
# =============================================================================
echo "📊 Ejecutando migraciones..."
cd services/api-gateway || exit 1

# Verificar si sqlx está instalado
if ! command -v sqlx &> /dev/null; then
    echo -e "${YELLOW}⚠️  sqlx-cli no está instalado. Instalando...${NC}"
    cargo install sqlx-cli --no-default-features --features postgres
fi

# Configurar DATABASE_URL
export DATABASE_URL="postgresql://vibestream:vibestream@localhost:5433/vibestream"

# Ejecutar migraciones
if sqlx migrate run; then
    echo -e "${GREEN}✅ Migraciones ejecutadas correctamente${NC}"
else
    echo -e "${RED}❌ Error al ejecutar migraciones${NC}"
    exit 1
fi
echo ""

# =============================================================================
# 4. Generar JWT_SECRET
# =============================================================================
echo "🔐 Generando JWT_SECRET..."
JWT_SECRET=$(openssl rand -base64 32 2>/dev/null || head -c 32 /dev/urandom | base64)

if [ -z "$JWT_SECRET" ]; then
    echo -e "${YELLOW}⚠️  No se pudo generar JWT_SECRET automáticamente${NC}"
    JWT_SECRET="your_super_secret_jwt_key_change_in_production_$(date +%s)"
fi
echo -e "${GREEN}✅ JWT_SECRET generado${NC}"
echo ""

# =============================================================================
# 5. Crear archivo .env
# =============================================================================
echo "📝 Creando archivo .env..."
ENV_FILE="services/api-gateway/.env"

if [ -f "$ENV_FILE" ]; then
    echo -e "${YELLOW}⚠️  El archivo .env ya existe. ¿Deseas sobrescribirlo? (y/N)${NC}"
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        echo "📝 Manteniendo archivo .env existente"
        echo ""
        echo "Para actualizar JWT_SECRET manualmente, edita: $ENV_FILE"
        exit 0
    fi
fi

cat > "$ENV_FILE" << EOF
# Database Configuration
DATABASE_URL=postgresql://vibestream:vibestream@localhost:5433/vibestream
TEST_DATABASE_URL=postgresql://vibestream:vibestream@localhost:5433/vibestream_test

# Redis Configuration
REDIS_URL=redis://localhost:6379

# JWT Configuration
# ⚠️ REQUIRED: JWT_SECRET must be set for security
JWT_SECRET=$JWT_SECRET
JWT_ACCESS_TOKEN_EXPIRY=3600
JWT_REFRESH_TOKEN_EXPIRY=2592000

# Server Configuration
SERVER_HOST=0.0.0.0
SERVER_PORT=3000

# Environment
ENVIRONMENT=development
RUST_LOG=info

# Optional: External Services (for future use)
# STRIPE_SECRET_KEY=sk_test_...
# IPFS_GATEWAY=https://ipfs.io/ipfs/
# ETHEREUM_RPC_URL=https://mainnet.infura.io/v3/your_project_id
# SOLANA_RPC_URL=https://api.mainnet-beta.solana.com
EOF

echo -e "${GREEN}✅ Archivo .env creado en: $ENV_FILE${NC}"
echo ""

# =============================================================================
# 6. Crear base de datos de test (opcional)
# =============================================================================
echo "🧪 Creando base de datos de test..."
TEST_DB_URL="postgresql://vibestream:vibestream@localhost:5433/vibestream_test"

if docker-compose exec -T postgres psql -U vibestream -tc "SELECT 1 FROM pg_database WHERE datname = 'vibestream_test'" | grep -q 1; then
    echo -e "${YELLOW}⚠️  Base de datos de test ya existe${NC}"
else
    docker-compose exec -T postgres psql -U vibestream -c "CREATE DATABASE vibestream_test;" || true
    echo -e "${GREEN}✅ Base de datos de test creada${NC}"
fi
echo ""

# =============================================================================
# 7. Resumen
# =============================================================================
echo "===================================="
echo -e "${GREEN}✅ Setup completado exitosamente!${NC}"
echo "===================================="
echo ""
echo "📋 Resumen:"
echo "  ✅ PostgreSQL corriendo en puerto 5433"
echo "  ✅ Redis corriendo en puerto 6379"
echo "  ✅ Migraciones ejecutadas"
echo "  ✅ JWT_SECRET configurado"
echo "  ✅ Archivo .env creado"
echo ""
echo "🚀 Próximos pasos:"
echo "  1. cd services/api-gateway"
echo "  2. cargo run --bin api-gateway-unified"
echo ""
echo "📖 Documentación:"
echo "  - API Contract: API_CONTRACT.md"
echo "  - Análisis Backend: ANALISIS_EXHAUSTIVO_BACKEND_COMPLETO.md"
echo ""
echo "🔗 Endpoints disponibles:"
echo "  - API: http://localhost:3000/api/v1"
echo "  - Health: http://localhost:3000/health"
echo "  - Swagger UI: http://localhost:3000/swagger-ui"
echo "  - ReDoc: http://localhost:3000/redoc"
echo ""
