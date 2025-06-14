# Configuración del API Gateway

## Variables de Entorno Requeridas

### Base de Datos PostgreSQL
```bash
export DATABASE_URL="postgresql://usuario:password@localhost:5432/vibestream"
```

### Redis
```bash
export REDIS_URL="redis://127.0.0.1:6379"
```

### Puerto del Servidor (Opcional)
```bash
export PORT="3000"
```

## Configuración de Desarrollo

Para desarrollo local, puedes configurar las variables así:

```bash
# En tu terminal, ejecuta:
export DATABASE_URL="postgresql://vibestream:dev_password_123_change_in_production@localhost:5432/vibestream"
export REDIS_URL="redis://127.0.0.1:6379"
export RUST_LOG="api_gateway=debug,tower_http=debug"

# Luego ejecuta el servidor:
cargo run
```

## Configuración de Producción

En producción, configura estas variables en tu sistema de despliegue:

- **Docker**: usando `-e` o `--env-file`
- **Kubernetes**: usando ConfigMaps y Secrets
- **Cloud Services**: usando sus sistemas de configuración

## Archivo .env (Crear manualmente)

Si quieres usar un archivo `.env` local, créalo manualmente:

```bash
# Crear archivo .env manualmente (NO lo subas a Git)
touch .env
echo "DATABASE_URL=postgresql://vibestream:dev_password_123_change_in_production@localhost:5432/vibestream" >> .env
echo "REDIS_URL=redis://127.0.0.1:6379" >> .env
```

**⚠️ IMPORTANTE**: Nunca subas archivos `.env` a Git. Ya está en `.gitignore`. 