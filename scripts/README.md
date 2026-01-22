# 🚀 Scripts de Desarrollo - VibeStream

Scripts para gestionar el inicio y detención de los servicios de VibeStream.

## 📋 Configuración de Puertos

| Servicio                  | Puerto | URL                   |
| ------------------------- | ------ | --------------------- |
| **Backend (API Gateway)** | `3007` | http://localhost:3007 |
| **Frontend (Angular)**    | `4200` | http://localhost:4200 |

## 🎯 Scripts Disponibles

### Iniciar Servicios

```bash
# Iniciar TODO (Backend + Frontend)
./scripts/start-all.sh

# Iniciar solo Backend
./scripts/start-backend.sh

# Iniciar solo Frontend
./scripts/start-frontend.sh
```

### Detener Servicios

```bash
# Detener TODO
./scripts/stop-all.sh

# Detener solo Backend
./scripts/stop-backend.sh

# Detener solo Frontend
./scripts/stop-frontend.sh
```

## 📊 Monitoreo

### Ver Logs en Tiempo Real

```bash
# Logs del Backend
tail -f logs/backend.log

# Logs del Frontend
tail -f logs/frontend.log
```

### URLs Útiles

**Backend:**

- API: http://localhost:3007
- Health Check: http://localhost:3007/health
- Swagger UI: http://localhost:3007/swagger-ui
- ReDoc: http://localhost:3007/redoc

**Frontend:**

- Aplicación: http://localhost:4200

## 🔧 Características de los Scripts

### ✅ Gestión de Procesos

- Detecta si los servicios ya están corriendo
- Guarda PIDs para control de procesos
- Detención segura con fallback a `kill -9`

### 📝 Logging

- Logs separados por servicio en `logs/`
- Salida colorizada para mejor legibilidad
- Información de estado clara

### 🛡️ Validaciones

- Verifica que las herramientas necesarias estén instaladas (cargo, npm)
- Instala dependencias automáticamente si faltan
- Manejo de errores robusto

## 🔄 Cambiar Puertos

### Backend

Edita: `services/api-gateway/.env`

```env
SERVER_PORT=3007  # Cambia este valor
```

### Frontend

Edita: `apps/frontend/package.json`

```json
{
  "scripts": {
    "start": "ng serve --port 4200"
  }
}
```

## 📦 Requisitos

- **Rust/Cargo** (para el backend)
- **Node.js/npm** (para el frontend)
- **PostgreSQL** (puerto 5435)
- **Redis** (puerto 6382)

## 🐛 Troubleshooting

### El backend no inicia

```bash
# Ver logs detallados
cat logs/backend.log

# Verificar que PostgreSQL y Redis estén corriendo
docker-compose up -d postgres redis
```

### El frontend no inicia

```bash
# Ver logs detallados
cat logs/frontend.log

# Reinstalar dependencias
cd apps/frontend
rm -rf node_modules
npm install
```

### Puerto ya en uso

```bash
# Encontrar proceso usando el puerto
lsof -i :3007  # Backend
lsof -i :4200  # Frontend

# Matar el proceso
kill -9 <PID>
```
