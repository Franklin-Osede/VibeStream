# Script de inicialización de Vault para Windows

Write-Host "Esperando a que Vault esté disponible..." -ForegroundColor Yellow
Start-Sleep -Seconds 5

# Función para ejecutar comandos de Vault a través de Docker
function Invoke-VaultCommand {
  param (
    [string]$command
  )
  docker exec compose-vault-1 sh -c "VAULT_TOKEN=dev-only-token VAULT_ADDR=http://127.0.0.1:8200 $command"
}

Write-Host "Habilitando motor de secretos KV v2..." -ForegroundColor Green
Invoke-VaultCommand "vault secrets enable -version=2 kv"

# Crear política para la aplicación Vibestream
$policy = @"
# Política para la aplicación Vibestream
path "kv/data/vibestream/*" {
  capabilities = ["read", "list"]
}

path "kv/metadata/vibestream/*" {
  capabilities = ["list"]
}
"@

Write-Host "Creando política para Vibestream..." -ForegroundColor Green
$policy | Out-File -Encoding ASCII -FilePath ".\infra\docker\vault\policy.hcl"
docker cp ".\infra\docker\vault\policy.hcl" compose-vault-1:/policy.hcl
Invoke-VaultCommand "vault policy write vibestream-app /policy.hcl"

Write-Host "Almacenando secretos iniciales..." -ForegroundColor Green

# Secretos de base de datos
Invoke-VaultCommand 'vault kv put kv/vibestream/database username="vibestream" password="dev_password_123_change_in_production" database="vibestream" host="postgres" port="5432"'

# Secretos para JWT
Invoke-VaultCommand 'vault kv put kv/vibestream/jwt secret="jwt_secret_key_dev_only_change_in_production" expiration="24h"'

# Secretos para Web3/Smart Contracts
Invoke-VaultCommand 'vault kv put kv/vibestream/web3 private_key="0xdev_private_key_change_in_production" rpc_url="https://eth-sepolia.g.alchemy.com/v2/your-api-key" chain_id="11155111"'

# Secretos para Redis
Invoke-VaultCommand 'vault kv put kv/vibestream/redis host="redis" port="6379"'

Write-Host "Verificando secretos almacenados..." -ForegroundColor Green
Invoke-VaultCommand "vault kv list kv/vibestream/"

Write-Host "¡Inicialización de Vault completada!" -ForegroundColor Green
Write-Host "Token de desarrollo: dev-only-token" -ForegroundColor Yellow
Write-Host "URL de Vault: http://127.0.0.1:8200" -ForegroundColor Yellow 