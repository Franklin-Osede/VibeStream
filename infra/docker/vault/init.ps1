# Esperar a que Vault esté listo
Start-Sleep -Seconds 5

# Configurar variables de entorno para el cliente Vault
$env:VAULT_ADDR = 'http://127.0.0.1:8200'
$env:VAULT_TOKEN = 'dev-only-token'

# Habilitar el motor de secretos KV version 2
vault secrets enable -version=2 kv

# Crear política para la aplicación
$policy = @"
path "kv/data/vibestream/*" {
  capabilities = ["read"]
}
"@

$policy | vault policy write vibestream-app -

# Almacenar secretos iniciales
vault kv put kv/vibestream/database `
  username="vibestream" `
  password="dev_password_123_change_in_production"

vault kv put kv/vibestream/jwt `
  secret="dev_jwt_secret_key_change_in_production"

vault kv put kv/vibestream/web3 `
  private_key="dev_private_key_change_in_production"

Write-Host "Vault initialized with basic secrets and policies" 