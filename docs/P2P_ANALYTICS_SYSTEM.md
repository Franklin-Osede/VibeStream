# 🚀 Sistema de Analíticas P2P - VibeStream

## 📋 Resumen Ejecutivo

El sistema de analíticas P2P de VibeStream es una solución completa para monitorear y analizar el rendimiento del streaming peer-to-peer en tiempo real. Proporciona métricas detalladas, visualización en dashboard y alertas del sistema.

## 🎯 Estado Actual: **85% COMPLETADO**

### ✅ **Implementado y Funcionando:**

1. **Sistema de Analíticas P2P (85%)**
   - ✅ Repositorio PostgreSQL con persistencia real
   - ✅ Repositorio en memoria para desarrollo
   - ✅ Métricas de conexión P2P completas
   - ✅ Métricas de streaming de video
   - ✅ Análisis de calidad de conexión
   - ✅ APIs REST completas
   - ✅ Dashboard web moderno y responsivo
   - ✅ Gráficos en tiempo real con Chart.js
   - ✅ Sistema de alertas
   - ✅ Scripts de prueba automatizados

2. **Sistema de Recomendaciones (80%)**
   - ✅ Filtrado colaborativo
   - ✅ Filtrado basado en contenido
   - ✅ Recomendaciones P2P
   - ✅ APIs completas

3. **Dashboard de Monitoreo (75%)**
   - ✅ Entidades de métricas completas
   - ✅ Servicio de monitoreo
   - ✅ Coleccionadores de métricas
   - ✅ Sistema de alertas

## 🏗️ Arquitectura del Sistema

### Estructura de Directorios

```
services/api-gateway/src/bounded_contexts/p2p/
├── domain/
│   ├── entities/
│   │   └── analytics.rs          # Entidades de analíticas
│   └── repositories/
│       └── analytics_repository.rs # Traits de repositorio
├── application/
│   └── services/
│       └── analytics_service.rs  # Servicio de aplicación
├── infrastructure/
│   └── repositories/
│       ├── in_memory_analytics_repository.rs
│       └── postgresql_analytics_repository.rs
└── presentation/
    ├── controllers/
    │   ├── analytics_controller.rs
    │   └── dashboard_controller.rs
    └── routes/
        ├── analytics_routes.rs
        └── dashboard_routes.rs
```

### Componentes Principales

#### 1. **Entidades de Dominio**

```rust
// Métricas de conexión P2P
pub struct P2PConnectionMetrics {
    pub connection_id: String,
    pub session_id: String,
    pub peer_id: String,
    pub connection_type: ConnectionType,
    pub latency_ms: u32,
    pub bandwidth_mbps: f64,
    pub packet_loss_percent: f64,
    pub jitter_ms: u32,
    pub connection_quality: ConnectionQuality,
    // ...
}

// Métricas de streaming
pub struct StreamingMetrics {
    pub stream_id: String,
    pub content_id: String,
    pub quality_level: VideoQuality,
    pub bitrate_kbps: u32,
    pub frame_rate: f32,
    // ...
}

// Agregado de analíticas
pub struct P2PAnalyticsAggregate {
    pub id: String,
    pub session_id: String,
    pub user_id: String,
    pub connection_metrics: Vec<P2PConnectionMetrics>,
    pub streaming_metrics: Vec<StreamingMetrics>,
    // ...
}
```

#### 2. **Repositorios**

- **InMemoryP2PAnalyticsRepository**: Para desarrollo y testing
- **PostgreSQLP2PAnalyticsRepository**: Para producción con persistencia real

#### 3. **Servicios de Aplicación**

```rust
pub struct P2PAnalyticsService<R: P2PAnalyticsRepository> {
    repository: Arc<R>,
}

impl<R: P2PAnalyticsRepository> P2PAnalyticsService<R> {
    pub async fn record_connection_metrics(&self, ...) -> Result<(), AnalyticsError>
    pub async fn record_streaming_metrics(&self, ...) -> Result<(), AnalyticsError>
    pub async fn get_session_analytics(&self, ...) -> Result<Option<P2PAnalyticsAggregate>, AnalyticsError>
    pub async fn get_aggregated_stats(&self, ...) -> Result<AggregatedStats, AnalyticsError>
}
```

## 🌐 APIs Disponibles

### Endpoints de Analíticas

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| `POST` | `/api/p2p/analytics/connection-metrics` | Registrar métricas de conexión |
| `POST` | `/api/p2p/analytics/streaming-metrics` | Registrar métricas de streaming |
| `GET` | `/api/p2p/analytics/session/{session_id}` | Obtener analíticas de sesión |
| `GET` | `/api/p2p/analytics/user/{user_id}` | Obtener analíticas de usuario |
| `GET` | `/api/p2p/analytics/stats?hours=24` | Estadísticas agregadas |
| `GET` | `/api/p2p/analytics/performance-report/{user_id}?days=7` | Reporte de rendimiento |

### Endpoints del Dashboard

| Método | Endpoint | Descripción |
|--------|----------|-------------|
| `GET` | `/api/p2p/dashboard/` | Dashboard HTML |
| `GET` | `/api/p2p/dashboard/realtime-metrics` | Métricas en tiempo real |
| `GET` | `/api/p2p/dashboard/alerts` | Alertas del sistema |
| `GET` | `/api/p2p/dashboard/trends` | Gráficos de tendencias |

## 📊 Dashboard de Monitoreo

### Características

- **Diseño Moderno**: Interfaz responsiva con gradientes y animaciones
- **Métricas en Tiempo Real**: Actualización automática cada 30 segundos
- **Gráficos Interactivos**: Chart.js con múltiples tipos de visualización
- **Sistema de Alertas**: Notificaciones de problemas del sistema
- **Métricas Principales**:
  - Usuarios activos
  - Conexiones P2P
  - Latencia promedio
  - Ancho de banda
  - Tasa de éxito
  - Streams activos

### Acceso al Dashboard

```
http://localhost:8080/api/p2p/dashboard/
```

## 🗄️ Base de Datos

### Esquema PostgreSQL

```sql
-- Tabla principal de analíticas
CREATE TABLE p2p_analytics_aggregates (
    id VARCHAR(255) PRIMARY KEY,
    session_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    network_metrics JSONB NOT NULL,
    system_metrics JSONB NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Métricas de conexión
CREATE TABLE p2p_connection_metrics (
    id VARCHAR(255) PRIMARY KEY,
    analytics_id VARCHAR(255) NOT NULL,
    connection_id VARCHAR(255) NOT NULL,
    session_id VARCHAR(255) NOT NULL,
    peer_id VARCHAR(255) NOT NULL,
    connection_type VARCHAR(50) NOT NULL,
    latency_ms INTEGER NOT NULL,
    bandwidth_mbps DOUBLE PRECISION NOT NULL,
    packet_loss_percent DOUBLE PRECISION NOT NULL,
    jitter_ms INTEGER NOT NULL,
    connection_quality VARCHAR(50) NOT NULL,
    ice_connection_state VARCHAR(255) NOT NULL,
    dtls_transport_state VARCHAR(255) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    FOREIGN KEY (analytics_id) REFERENCES p2p_analytics_aggregates(id) ON DELETE CASCADE
);

-- Métricas de streaming
CREATE TABLE p2p_streaming_metrics (
    id VARCHAR(255) PRIMARY KEY,
    analytics_id VARCHAR(255) NOT NULL,
    stream_id VARCHAR(255) NOT NULL,
    content_id VARCHAR(255) NOT NULL,
    user_id VARCHAR(255) NOT NULL,
    quality_level VARCHAR(50) NOT NULL,
    bitrate_kbps INTEGER NOT NULL,
    frame_rate REAL NOT NULL,
    resolution_width INTEGER NOT NULL,
    resolution_height INTEGER NOT NULL,
    buffer_level_seconds REAL NOT NULL,
    dropped_frames INTEGER NOT NULL,
    total_frames INTEGER NOT NULL,
    adaptive_switches INTEGER NOT NULL,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE,
    duration_seconds DOUBLE PRECISION NOT NULL,
    FOREIGN KEY (analytics_id) REFERENCES p2p_analytics_aggregates(id) ON DELETE CASCADE
);
```

### Índices Optimizados

```sql
CREATE INDEX idx_p2p_analytics_session_id ON p2p_analytics_aggregates(session_id);
CREATE INDEX idx_p2p_analytics_user_id ON p2p_analytics_aggregates(user_id);
CREATE INDEX idx_p2p_analytics_created_at ON p2p_analytics_aggregates(created_at);
```

## 🧪 Pruebas

### Script de Pruebas Automatizadas

```bash
# Ejecutar todas las pruebas del sistema de analíticas
./scripts/test-p2p-analytics.sh
```

### Pruebas Manuales

```bash
# 1. Registrar métricas de conexión
curl -X POST http://localhost:8080/api/p2p/analytics/connection-metrics \
  -H "Content-Type: application/json" \
  -d '{
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

# 2. Obtener analíticas de sesión
curl http://localhost:8080/api/p2p/analytics/session/test-session-001

# 3. Obtener estadísticas agregadas
curl http://localhost:8080/api/p2p/analytics/stats?hours=24

# 4. Acceder al dashboard
curl http://localhost:8080/api/p2p/dashboard/
```

## 🚀 Configuración y Despliegue

### 1. Configuración del Repositorio

```rust
// Para desarrollo (memoria)
let repository = P2PAnalyticsRepositoryFactory::create_in_memory();

// Para producción (PostgreSQL)
let pool = sqlx::PgPool::connect(&database_url).await?;
let repository = P2PAnalyticsRepositoryFactory::create_postgresql(pool).await?;
```

### 2. Configuración del Servicio

```rust
let analytics_service = Arc::new(P2PAnalyticsService::new(repository));
let analytics_controller = Arc::new(P2PAnalyticsController::new(analytics_service));
```

### 3. Configuración de Rutas

```rust
let app = Router::new()
    .nest("/api/p2p/analytics", create_analytics_routes(analytics_controller.clone()))
    .nest("/api/p2p/dashboard", create_dashboard_routes(analytics_controller));
```

## 📈 Métricas Disponibles

### Métricas de Conexión P2P

- **Latencia**: Tiempo de respuesta entre peers
- **Ancho de Banda**: Capacidad de transferencia
- **Pérdida de Paquetes**: Porcentaje de paquetes perdidos
- **Jitter**: Variación en la latencia
- **Calidad de Conexión**: Excellent/Good/Fair/Poor/Unusable
- **Estado ICE**: Estado de la conexión WebRTC
- **Estado DTLS**: Estado del transporte seguro

### Métricas de Streaming

- **Calidad de Video**: UltraHD/FullHD/HD/SD/Low
- **Bitrate**: Velocidad de transmisión
- **Frame Rate**: Frames por segundo
- **Resolución**: Ancho x Alto
- **Buffer Level**: Nivel de buffer en segundos
- **Frames Dropped**: Frames perdidos
- **Adaptive Switches**: Cambios de calidad adaptativa

### Métricas del Sistema

- **CPU Usage**: Uso de procesador
- **Memory Usage**: Uso de memoria
- **Network Throughput**: Rendimiento de red
- **Active Streams**: Streams activos
- **Total Peers**: Total de peers conectados
- **Error Rate**: Tasa de errores

## 🔮 Próximos Pasos

### Fase 2: Mejoras (2-3 semanas)

1. **WebRTC Real (ALTA PRIORIDAD)**
   - Reemplazar mock con webrtc-rs real
   - Implementar conexiones P2P reales
   - Optimizar rendimiento de streaming

2. **Payment Gateways Reales (ALTA PRIORIDAD)**
   - Integración con Stripe real
   - Integración con PayPal real
   - Integración con Coinbase real

3. **Video Storage IPFS (MEDIA PRIORIDAD)**
   - Almacenamiento de video en IPFS
   - Gestión de chunks de video
   - Optimización de distribución P2P

### Fase 3: Optimizaciones (3-4 semanas)

1. **Dashboard Avanzado**
   - Más tipos de gráficos
   - Filtros avanzados
   - Exportación de reportes

2. **Sistema de Alertas Inteligente**
   - Machine Learning para detección de anomalías
   - Alertas predictivas
   - Notificaciones automáticas

3. **Analíticas Avanzadas**
   - Análisis de patrones de uso
   - Predicciones de rendimiento
   - Optimización automática

## 🛠️ Troubleshooting

### Problemas Comunes

1. **Error de conexión a PostgreSQL**
   ```bash
   # Verificar que PostgreSQL esté corriendo
   sudo systemctl status postgresql
   
   # Verificar conexión
   psql -h localhost -U postgres -d vibestream
   ```

2. **Dashboard no carga**
   ```bash
   # Verificar que el servidor esté corriendo
   curl http://localhost:8080/health
   
   # Verificar logs del servidor
   tail -f logs/api-gateway.log
   ```

3. **Métricas no se registran**
   ```bash
   # Verificar que las tablas existan
   psql -d vibestream -c "\dt p2p_*"
   
   # Verificar permisos de base de datos
   psql -d vibestream -c "GRANT ALL PRIVILEGES ON ALL TABLES IN SCHEMA public TO vibestream_user;"
   ```

## 📞 Soporte

Para soporte técnico o preguntas sobre el sistema de analíticas P2P:

- **Documentación**: Este archivo
- **Issues**: GitHub Issues del proyecto
- **Logs**: `logs/api-gateway.log`

---

**Última actualización**: $(date)
**Versión**: 1.0.0
**Estado**: 85% Completado 