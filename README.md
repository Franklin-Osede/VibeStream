# VibeStream 🌊

**Plataforma de streaming de pagos blockchain con arquitectura de microservicios**

## 🏗️ Arquitectura

VibeStream utiliza una arquitectura de microservicios moderna que resuelve conflictos de dependencias y permite escalabilidad independiente:

```
┌─────────────────┐    ┌─────────────────┐
│   Mobile App    │    │    Web App      │
└─────────┬───────┘    └─────────┬───────┘
          │                      │
          └──────────┬───────────┘
                     │
         ┌───────────▼───────────┐
         │    API Gateway        │
         │   (tokio 1.25+)       │
         └───────────┬───────────┘
                     │
         ┌───────────▼───────────┐
         │    Redis Queue        │
         └─┬─────────┬─────────┬─┘
           │         │         │
    ┌──────▼──┐ ┌────▼────┐ ┌──▼──────┐
    │Ethereum │ │ Solana  │ │   ZK    │
    │Service  │ │Service  │ │Service  │
    │(tokio   │ │(tokio   │ │(tokio   │
    │ 1.18+)  │ │ 1.14)   │ │ 1.25+)  │
    └─────────┘ └─────────┘ └─────────┘
```

## 🚀 Inicio Rápido

### Prerrequisitos
- Rust 1.70+
- Redis
- Node.js (para apps frontend)

### Desarrollo Local

```bash
# Iniciar todos los servicios
./scripts/dev-start.sh

# Detener todos los servicios
./scripts/dev-stop.sh
```

### Verificar Estado
```bash
# Health check del API Gateway
curl http://localhost:3000/health

# Ver logs
tail -f logs/api-gateway.log
tail -f logs/ethereum-service.log
tail -f logs/zk-service.log
```

## 📁 Estructura del Proyecto

```
VibeStream/
├── shared/
│   └── types/              # Tipos compartidos entre servicios
│       ├── src/
│       │   ├── blockchain.rs
│       │   ├── messages.rs
│       │   └── errors.rs
│       └── Cargo.toml
├── services/
│   ├── api-gateway/        # API REST principal (puerto 3000)
│   │   ├── src/
│   │   │   ├── handlers.rs
│   │   │   ├── services.rs
│   │   │   └── main.rs
│   │   └── tests/
│   ├── ethereum/           # Servicio Ethereum
│   │   └── src/
│   ├── solana/            # Servicio Solana (workspace independiente)
│   │   └── src/
│   └── zk-service/        # Servicio Zero-Knowledge
│       └── src/
├── apps/
│   ├── mobile/            # App móvil (React Native)
│   └── web/               # App web (Next.js)
├── scripts/
│   ├── dev-start.sh       # Script de inicio
│   └── dev-stop.sh        # Script de parada
└── logs/                  # Logs de servicios
```

## 🔧 Servicios

### API Gateway (Puerto 3000)
- **Tecnología**: Axum + tokio 1.25+
- **Función**: Punto de entrada único, enrutamiento de requests
- **Endpoints**:
  - `GET /health` - Health check
  - `POST /api/v1/transactions` - Procesar transacciones
  - `GET /api/v1/balance/:address` - Consultar balance
  - `POST /api/v1/streams` - Crear stream de pagos

### Ethereum Service
- **Tecnología**: Ethers.rs + tokio 1.18+
- **Función**: Operaciones en blockchain Ethereum
- **Capacidades**: Transacciones, balances, smart contracts

### Solana Service
- **Tecnología**: Solana SDK + tokio 1.14
- **Función**: Operaciones en blockchain Solana
- **Capacidades**: Transacciones, balances, programas

### ZK Service
- **Tecnología**: Arkworks + tokio 1.25+
- **Función**: Pruebas zero-knowledge
- **Capacidades**: Pruebas de solvencia, privacidad

## 📡 Comunicación Entre Servicios

Los servicios se comunican a través de **Redis** usando un patrón de colas de mensajes:

```rust
// Ejemplo de mensaje
{
  "id": "uuid-v4",
  "timestamp": "2024-01-01T00:00:00Z",
  "payload": {
    "ProcessTransaction": {
      "blockchain": "Ethereum",
      "from": "0x123...",
      "to": "0x456...",
      "amount": 1000
    }
  }
}
```

### Colas Redis
- `ethereum_queue` - Mensajes para Ethereum Service
- `solana_queue` - Mensajes para Solana Service  
- `zk_queue` - Mensajes para ZK Service
- `response_queue` - Respuestas de servicios

## 🧪 Testing

```bash
# Ejecutar todas las pruebas
cargo test --workspace

# Pruebas específicas del API Gateway
cargo test -p api-gateway

# Pruebas de integración
cargo test --test integration_tests
```

### Tipos de Pruebas
- ✅ **Unitarias**: Lógica de cada servicio
- ✅ **Integración**: Comunicación entre servicios
- ✅ **Serialización**: Mensajes y tipos compartidos
- ✅ **Conexión**: Redis y bases de datos

## 🔒 Seguridad

- **Validación**: Todos los inputs son validados
- **Rate Limiting**: Protección contra spam
- **Autenticación**: JWT tokens
- **Encriptación**: TLS en todas las comunicaciones
- **Zero-Knowledge**: Privacidad en transacciones

## 📊 Monitoreo

### Métricas Disponibles
- Tiempo de respuesta por servicio
- Throughput de transacciones
- Longitud de colas Redis
- Uso de memoria y CPU
- Errores por servicio

### Logs
Los logs se almacenan en `logs/` con formato estructurado:
```
[2024-01-01T12:00:00Z INFO api_gateway] Request processed: tx_hash=0x123...
```

## 🚀 Despliegue

### Docker
```bash
# Construir imágenes
docker-compose build

# Iniciar servicios
docker-compose up -d
```

### Kubernetes
```bash
# Aplicar manifiestos
kubectl apply -f k8s/
```

## 🛠️ Desarrollo

### Agregar Nuevo Servicio

1. **Crear directorio**: `services/nuevo-servicio/`
2. **Cargo.toml**: Definir dependencias
3. **Agregar al workspace**: Actualizar `Cargo.toml` raíz
4. **Implementar**: Usar `vibestream-types` para comunicación
5. **Pruebas**: Agregar tests de integración

### Modificar Tipos Compartidos

1. **Editar**: `shared/types/src/`
2. **Compilar**: `cargo check --workspace`
3. **Actualizar servicios**: Según cambios de API
4. **Pruebas**: Verificar compatibilidad

## 🐛 Resolución de Problemas

### Servicio No Inicia
```bash
# Verificar logs
tail -f logs/servicio.log

# Verificar puertos
lsof -i :3000

# Reiniciar Redis
redis-cli shutdown
redis-server --daemonize yes
```

### Conflictos de Dependencias
- Cada servicio maneja sus propias dependencias
- Solana Service tiene workspace independiente
- Usar `cargo tree` para diagnosticar

### Performance
```bash
# Monitorear Redis
redis-cli monitor

# Métricas de sistema
htop
```

## 📈 Roadmap

- [ ] **Frontend Apps**: React Native + Next.js
- [ ] **Monitoring**: Prometheus + Grafana
- [ ] **Load Balancing**: Nginx + múltiples instancias
- [ ] **Database**: PostgreSQL para persistencia
- [ ] **CI/CD**: GitHub Actions
- [ ] **Documentation**: OpenAPI specs

## 🤝 Contribuir

1. Fork el repositorio
2. Crear branch: `git checkout -b feature/nueva-funcionalidad`
3. Commit: `git commit -m 'Agregar nueva funcionalidad'`
4. Push: `git push origin feature/nueva-funcionalidad`
5. Pull Request

## 📄 Licencia

MIT License - ver [LICENSE](LICENSE) para detalles.

---

**VibeStream** - Streaming de pagos blockchain del futuro 🌊✨

