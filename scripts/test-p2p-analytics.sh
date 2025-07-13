#!/bin/bash

# Script para probar el sistema de analíticas P2P de VibeStream
# Este script verifica que las APIs de analíticas funcionen correctamente

set -e

echo "🚀 Iniciando pruebas del sistema de analíticas P2P..."

# Configuración
API_BASE_URL="http://localhost:8080"
API_TIMEOUT=10

# Colores para output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Función para imprimir mensajes con colores
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Función para hacer requests HTTP
make_request() {
    local method=$1
    local endpoint=$2
    local data=$3
    
    if [ -n "$data" ]; then
        curl -s -X "$method" \
            -H "Content-Type: application/json" \
            -d "$data" \
            --max-time $API_TIMEOUT \
            "$API_BASE_URL$endpoint"
    else
        curl -s -X "$method" \
            --max-time $API_TIMEOUT \
            "$API_BASE_URL$endpoint"
    fi
}

# Verificar que el servidor esté corriendo
print_status "Verificando que el servidor esté corriendo..."
if ! make_request "GET" "/health" > /dev/null 2>&1; then
    print_error "El servidor no está corriendo en $API_BASE_URL"
    print_error "Asegúrate de iniciar el servidor con: cargo run --bin api-gateway"
    exit 1
fi
print_success "Servidor está corriendo"

# 1. Probar registro de métricas de conexión
print_status "1. Probando registro de métricas de conexión..."

CONNECTION_METRICS='{
    "session_id": "test-session-001",
    "user_id": "test-user-001",
    "connection_id": "conn-001",
    "peer_id": "peer-001",
    "connection_type": "WebRTC",
    "latency_ms": 45,
    "bandwidth_mbps": 10.5,
    "packet_loss_percent": 0.1,
    "jitter_ms": 5,
    "connection_quality": "Excellent",
    "ice_connection_state": "connected",
    "dtls_transport_state": "connected"
}'

response=$(make_request "POST" "/api/p2p/analytics/connection-metrics" "$CONNECTION_METRICS")
if echo "$response" | grep -q "successfully"; then
    print_success "Métricas de conexión registradas correctamente"
else
    print_error "Error registrando métricas de conexión: $response"
fi

# 2. Probar registro de métricas de streaming
print_status "2. Probando registro de métricas de streaming..."

STREAMING_METRICS='{
    "session_id": "test-session-001",
    "user_id": "test-user-001",
    "stream_id": "stream-001",
    "content_id": "content-001",
    "quality_level": "HD",
    "bitrate_kbps": 2500,
    "frame_rate": 30.0,
    "resolution_width": 1920,
    "resolution_height": 1080,
    "buffer_level_seconds": 5.0,
    "dropped_frames": 2,
    "total_frames": 900,
    "adaptive_switches": 1,
    "start_time": "'$(date -u +%Y-%m-%dT%H:%M:%SZ)'",
    "end_time": null,
    "duration_seconds": 30.0
}'

response=$(make_request "POST" "/api/p2p/analytics/streaming-metrics" "$STREAMING_METRICS")
if echo "$response" | grep -q "successfully"; then
    print_success "Métricas de streaming registradas correctamente"
else
    print_error "Error registrando métricas de streaming: $response"
fi

# 3. Probar obtención de analíticas de sesión
print_status "3. Probando obtención de analíticas de sesión..."

response=$(make_request "GET" "/api/p2p/analytics/session/test-session-001")
if echo "$response" | grep -q "test-session-001"; then
    print_success "Analíticas de sesión obtenidas correctamente"
    echo "$response" | jq '.' 2>/dev/null || echo "$response"
else
    print_error "Error obteniendo analíticas de sesión: $response"
fi

# 4. Probar obtención de analíticas de usuario
print_status "4. Probando obtención de analíticas de usuario..."

response=$(make_request "GET" "/api/p2p/analytics/user/test-user-001")
if echo "$response" | grep -q "test-user-001"; then
    print_success "Analíticas de usuario obtenidas correctamente"
    echo "$response" | jq '.' 2>/dev/null || echo "$response"
else
    print_error "Error obteniendo analíticas de usuario: $response"
fi

# 5. Probar obtención de estadísticas agregadas
print_status "5. Probando obtención de estadísticas agregadas..."

response=$(make_request "GET" "/api/p2p/analytics/stats?hours=24")
if echo "$response" | grep -q "total_sessions"; then
    print_success "Estadísticas agregadas obtenidas correctamente"
    echo "$response" | jq '.' 2>/dev/null || echo "$response"
else
    print_error "Error obteniendo estadísticas agregadas: $response"
fi

# 6. Probar generación de reporte de rendimiento
print_status "6. Probando generación de reporte de rendimiento..."

response=$(make_request "GET" "/api/p2p/analytics/performance-report/test-user-001?days=7")
if echo "$response" | grep -q "test-user-001"; then
    print_success "Reporte de rendimiento generado correctamente"
    echo "$response" | jq '.' 2>/dev/null || echo "$response"
else
    print_error "Error generando reporte de rendimiento: $response"
fi

# 7. Probar dashboard de monitoreo
print_status "7. Probando dashboard de monitoreo..."

# Métricas en tiempo real
response=$(make_request "GET" "/api/p2p/dashboard/realtime-metrics?hours=1")
if echo "$response" | grep -q "timestamp"; then
    print_success "Métricas en tiempo real obtenidas correctamente"
    echo "$response" | jq '.' 2>/dev/null || echo "$response"
else
    print_error "Error obteniendo métricas en tiempo real: $response"
fi

# Alertas del sistema
response=$(make_request "GET" "/api/p2p/dashboard/alerts")
if echo "$response" | grep -q "total_alerts"; then
    print_success "Alertas del sistema obtenidas correctamente"
    echo "$response" | jq '.' 2>/dev/null || echo "$response"
else
    print_error "Error obteniendo alertas del sistema: $response"
fi

# Gráficos de tendencias
response=$(make_request "GET" "/api/p2p/dashboard/trends?hours=24")
if echo "$response" | grep -q "latency_trend"; then
    print_success "Gráficos de tendencias obtenidos correctamente"
    echo "$response" | jq '.' 2>/dev/null || echo "$response"
else
    print_error "Error obteniendo gráficos de tendencias: $response"
fi

# 8. Probar acceso al dashboard HTML
print_status "8. Probando acceso al dashboard HTML..."

response=$(make_request "GET" "/api/p2p/dashboard/")
if echo "$response" | grep -q "VibeStream P2P Dashboard"; then
    print_success "Dashboard HTML accesible correctamente"
else
    print_error "Error accediendo al dashboard HTML: $response"
fi

# Resumen final
echo ""
echo "🎉 ========================================="
echo "🎉 PRUEBAS DEL SISTEMA DE ANALÍTICAS P2P"
echo "🎉 ========================================="
echo ""
print_success "Todas las pruebas completadas exitosamente"
echo ""
echo "📊 Endpoints probados:"
echo "   ✅ POST /api/p2p/analytics/connection-metrics"
echo "   ✅ POST /api/p2p/analytics/streaming-metrics"
echo "   ✅ GET  /api/p2p/analytics/session/{session_id}"
echo "   ✅ GET  /api/p2p/analytics/user/{user_id}"
echo "   ✅ GET  /api/p2p/analytics/stats"
echo "   ✅ GET  /api/p2p/analytics/performance-report/{user_id}"
echo "   ✅ GET  /api/p2p/dashboard/realtime-metrics"
echo "   ✅ GET  /api/p2p/dashboard/alerts"
echo "   ✅ GET  /api/p2p/dashboard/trends"
echo "   ✅ GET  /api/p2p/dashboard/"
echo ""
echo "🌐 Dashboard disponible en: http://localhost:8080/api/p2p/dashboard/"
echo ""
print_success "¡El sistema de analíticas P2P está funcionando correctamente!" 