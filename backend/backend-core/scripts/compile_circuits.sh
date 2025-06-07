#!/bin/bash

# Función para verificar si el último comando se ejecutó correctamente
check_error() {
    if [ $? -ne 0 ]; then
        echo "❌ Error: $1"
        exit 1
    else
        echo "✅ $2"
    fi
}

# Ir al directorio raíz del backend
cd "$(dirname "$0")/.."

# Asegurarse de que el directorio build existe
mkdir -p build/circuits
check_error "No se pudo crear el directorio build" "Directorio build creado"

# Compilar el circuito de prueba
echo "🔨 Compilando circuito de prueba..."
circom circuits/test.circom --r1cs --wasm --sym --c -o build/circuits
check_error "Fallo en la compilación del circuito" "Circuito compilado correctamente"

# Verificar que se generó el archivo r1cs
if [ ! -f "build/circuits/proof_of_listen.r1cs" ]; then
    echo "❌ Error: No se generó el archivo r1cs"
    exit 1
fi

# Generar la trusted setup
echo "🔑 Generando trusted setup..."
echo "Desarrollo1234" | snarkjs powersoftau new bn128 12 build/circuits/pot12_0000.ptau
check_error "Fallo en la generación del primer ptau" "Primer ptau generado"

echo "Segunda fase" | snarkjs powersoftau contribute build/circuits/pot12_0000.ptau build/circuits/pot12_0001.ptau --name="First contribution" -e="random1234"
check_error "Fallo en la contribución del ptau" "Contribución ptau completada"

snarkjs powersoftau prepare phase2 build/circuits/pot12_0001.ptau build/circuits/pot12_final.ptau
check_error "Fallo en la preparación de phase2" "Phase2 preparada"

# Generar los archivos de prueba
echo "📝 Generando archivos de prueba..."
snarkjs groth16 setup build/circuits/proof_of_listen.r1cs build/circuits/pot12_final.ptau build/circuits/proof_of_listen_0000.zkey
check_error "Fallo en el setup de groth16" "Setup groth16 completado"

echo "Contribución final" | snarkjs zkey contribute build/circuits/proof_of_listen_0000.zkey build/circuits/proof_of_listen_final.zkey --name="1st Contributor" -e="random5678"
check_error "Fallo en la contribución final" "Contribución final completada"

snarkjs zkey export verificationkey build/circuits/proof_of_listen_final.zkey build/circuits/verification_key.json
check_error "Fallo en la exportación de la clave de verificación" "Clave de verificación exportada"

echo "🎉 ¡Circuitos compilados y setup completado!" 