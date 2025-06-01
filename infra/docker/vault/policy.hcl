# Pol??tica para la aplicaci??n Vibestream
path "kv/data/vibestream/*" {
  capabilities = ["read", "list"]
}

path "kv/metadata/vibestream/*" {
  capabilities = ["list"]
}
