#!/bin/sh

# Esperar a que Vault esté listo
sleep 5

# Configurar variables de entorno para el cliente Vault
export VAULT_ADDR='http://127.0.0.1:8200'
export VAULT_TOKEN='dev-only-token'

# Habilitar el motor de secretos KV version 2
vault secrets enable -version=2 kv

# Crear política para la aplicación
vault policy write vibestream-app - <<EOF
path "kv/data/vibestream/*" {
  capabilities = ["read"]
}
EOF

# Almacenar secretos iniciales
vault kv put kv/vibestream/database \
    username="vibestream" \
    password="dev_password_123_change_in_production"

vault kv put kv/vibestream/jwt \
    secret="dev_jwt_secret_key_change_in_production"

vault kv put kv/vibestream/web3 \
    private_key="dev_private_key_change_in_production"

echo "Vault initialized with basic secrets and policies" 