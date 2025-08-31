#!/bin/bash

# CI/CD Script para VibeStream API Gateway
# Valida compilación, calidad de código, tests y migraciones

set -e  # Exit on any error

echo "🚀 Iniciando CI/CD para VibeStream API Gateway..."

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para logging
log_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

# Verificar que estamos en el directorio correcto
if [ ! -f "Cargo.toml" ]; then
    log_error "No se encontró Cargo.toml. Ejecutar desde el directorio raíz del proyecto."
    exit 1
fi

# Configurar variables de entorno para CI
export DATABASE_URL="postgresql://vibestream:vibestream@localhost:5433/vibestream"
export REDIS_URL="redis://localhost:6379"
export RUST_LOG="info"

# Feature flags para CI (solo módulos estables)
export FEATURE_LISTEN_REWARD="true"
export FEATURE_FAN_VENTURES="true"
export FEATURE_NOTIFICATIONS="false"
export FEATURE_MUSIC="true"
export FEATURE_ANALYTICS="false"
export FEATURE_SEARCH="false"
export FEATURE_MARKET_STATS="false"
export FEATURE_ZK_INTEGRATION="false"
export FEATURE_BLOCKCHAIN_INTEGRATION="false"

log_info "Configuración de CI:"
log_info "  DATABASE_URL: $DATABASE_URL"
log_info "  REDIS_URL: $REDIS_URL"
log_info "  Feature flags habilitados: listen_reward, fan_ventures, music"

# 1. Verificar dependencias
log_info "1. Verificando dependencias..."
if ! command -v cargo &> /dev/null; then
    log_error "Cargo no está instalado"
    exit 1
fi

if ! command -v sqlx &> /dev/null; then
    log_warning "SQLx CLI no está instalado. Instalando..."
    cargo install sqlx-cli --no-default-features --features postgres
fi

log_success "Dependencias verificadas"

# 2. Verificar formato de código
log_info "2. Verificando formato de código..."
if ! cargo fmt --check; then
    log_error "El código no está formateado correctamente"
    log_info "Ejecutar: cargo fmt"
    exit 1
fi
log_success "Formato de código verificado"

# 3. Verificar compilación
log_info "3. Verificando compilación..."
if ! cargo check; then
    log_error "Error de compilación"
    exit 1
fi
log_success "Compilación exitosa"

# 4. Verificar clippy (linter)
log_info "4. Ejecutando clippy..."
if ! cargo clippy -- -D warnings; then
    log_error "Clippy encontró warnings/errors"
    exit 1
fi
log_success "Clippy sin warnings"

# 5. Verificar tests unitarios
log_info "5. Ejecutando tests unitarios..."
if ! cargo test --lib; then
    log_error "Tests unitarios fallaron"
    exit 1
fi
log_success "Tests unitarios exitosos"

# 6. Verificar tests de integración
log_info "6. Ejecutando tests de integración..."
if ! cargo test --tests; then
    log_error "Tests de integración fallaron"
    exit 1
fi
log_success "Tests de integración exitosos"

# 7. Verificar migraciones SQLx
log_info "7. Verificando migraciones SQLx..."
if ! sqlx migrate info; then
    log_error "Error al verificar migraciones"
    exit 1
fi
log_success "Migraciones verificadas"

# 8. Verificar sqlx prepare (offline mode)
log_info "8. Verificando sqlx prepare..."
if ! cargo sqlx prepare --check; then
    log_error "sqlx prepare falló - verificar queries SQL"
    exit 1
fi
log_success "sqlx prepare verificado"

# 9. Verificar seguridad con cargo audit
log_info "9. Verificando vulnerabilidades de seguridad..."
if ! cargo audit; then
    log_warning "Se encontraron vulnerabilidades de seguridad"
    log_info "Revisar: cargo audit --fix"
else
    log_success "Sin vulnerabilidades de seguridad"
fi

# 10. Verificar documentación
log_info "10. Verificando documentación..."
if ! cargo doc --no-deps; then
    log_error "Error al generar documentación"
    exit 1
fi
log_success "Documentación generada"

# 11. Verificar tamaño del binario
log_info "11. Verificando tamaño del binario..."
if ! cargo build --release; then
    log_error "Error al compilar en modo release"
    exit 1
fi

BINARY_SIZE=$(stat -f%z target/release/api-gateway 2>/dev/null || stat -c%s target/release/api-gateway 2>/dev/null || echo "0")
MAX_SIZE=$((50 * 1024 * 1024))  # 50MB

if [ "$BINARY_SIZE" -gt "$MAX_SIZE" ]; then
    log_warning "Binario muy grande: ${BINARY_SIZE} bytes (> ${MAX_SIZE} bytes)"
else
    log_success "Tamaño del binario OK: ${BINARY_SIZE} bytes"
fi

# 12. Verificar health check del sistema
log_info "12. Verificando health check del sistema..."
if ! cargo run --bin health-check 2>/dev/null; then
    log_warning "Health check no disponible (opcional)"
else
    log_success "Health check exitoso"
fi

# Resumen final
echo ""
log_success "🎉 CI/CD completado exitosamente!"
log_info "Resumen:"
log_info "  ✅ Formato de código"
log_info "  ✅ Compilación"
log_info "  ✅ Clippy (sin warnings)"
log_info "  ✅ Tests unitarios"
log_info "  ✅ Tests de integración"
log_info "  ✅ Migraciones SQLx"
log_info "  ✅ sqlx prepare"
log_info "  ✅ Documentación"
log_info "  ✅ Tamaño del binario"

echo ""
log_info "🚀 Listo para deployment!"

# Opcional: Generar reporte de cobertura
if command -v grcov &> /dev/null; then
    log_info "Generando reporte de cobertura..."
    export CARGO_INCREMENTAL=0
    export RUSTFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
    export RUSTDOCFLAGS="-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Cpanic=abort -Zpanic_abort_tests"
    
    cargo test
    grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./coverage/
    log_success "Reporte de cobertura generado en ./coverage/"
fi








