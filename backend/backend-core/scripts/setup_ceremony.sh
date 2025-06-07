#!/bin/bash
set -e # Exit immediately if a command exits with a non-zero status.

# --- Go to project root ---
# This makes the script robust and runnable from anywhere. It finds its own location
# and navigates to the project root directory.
cd "$(dirname "$0")/../.."

# --- 1. Configuration (Paths from Project Root) ---
CIRCUIT_NAME="proof_of_listen"
CIRCUITS_DIR="./backend/circuits" # Corrected path based on file search
BACKEND_DIR="./backend"
ARTIFACTS_DIR="${BACKEND_DIR}/zk_artifacts" # Artifacts will be in backend/zk_artifacts
PTAU_POWER=15 # Increased power to support circuit complexity

# --- 2. Clean and Create Artifacts Directory ---
rm -rf $ARTIFACTS_DIR
mkdir -p $ARTIFACTS_DIR
echo "Cleaned and created artifacts directory at ${ARTIFACTS_DIR}"

# --- 3. Compile the Circuit ---
echo "--- Compiling Circuit: ${CIRCUIT_NAME}.circom ---"
# Paths are now relative to the project root
echo "DEBUG: Current directory before circom: $(pwd)"
echo "DEBUG: Circuit path argument: ${CIRCUITS_DIR}/${CIRCUIT_NAME}.circom"
circom "${CIRCUITS_DIR}/${CIRCUIT_NAME}.circom" --r1cs --wasm --sym -l "./backend/node_modules/circomlib/circuits" -o "$ARTIFACTS_DIR"

# --- 4. Powers of Tau (Phase 1) ---
echo "--- Ceremony Phase 1: Powers of Tau ---"
if [ -f "${ARTIFACTS_DIR}/powers_of_tau_final.ptau" ]; then
    echo "powers_of_tau_final.ptau already exists. Skipping Phase 1."
else
    echo "Starting new powers of tau ceremony..."
    snarkjs powersoftau new bn128 $PTAU_POWER "${ARTIFACTS_DIR}/pot_${PTAU_POWER}_0000.ptau" -v
    
    echo "Contributing to the ceremony (a random contribution for development purposes)..."
    snarkjs powersoftau contribute "${ARTIFACTS_DIR}/pot_${PTAU_POWER}_0000.ptau" "${ARTIFACTS_DIR}/pot_${PTAU_POWER}_0001.ptau" --name="Test Contribution" -v -e="$(openssl rand -base64 20)"
    
    echo "Finalizing Powers of Tau..."
    snarkjs powersoftau prepare phase2 "${ARTIFACTS_DIR}/pot_${PTAU_POWER}_0001.ptau" "${ARTIFACTS_DIR}/powers_of_tau_final.ptau" -v
fi

# --- 5. Circuit-Specific Setup (Phase 2) ---
echo "--- Ceremony Phase 2: Circuit-Specific Setup ---"

echo "Generating .zkey file from .r1cs and .ptau..."
snarkjs groth16 setup "${ARTIFACTS_DIR}/${CIRCUIT_NAME}.r1cs" "${ARTIFACTS_DIR}/powers_of_tau_final.ptau" "${ARTIFACTS_DIR}/${CIRCUIT_NAME}_0000.zkey"

echo "Contributing to the circuit-specific ceremony (VibeStream's contribution)..."
snarkjs zkey contribute "${ARTIFACTS_DIR}/${CIRCUIT_NAME}_0000.zkey" "${ARTIFACTS_DIR}/${CIRCUIT_NAME}.zkey" --name="VibeStream Setup Contribution" -v -e="$(openssl rand -base64 20)"

# --- 6. Export Verification Key ---
echo "--- Exporting Verification Key ---"
snarkjs zkey export verificationkey "${ARTIFACTS_DIR}/${CIRCUIT_NAME}.zkey" "${ARTIFACTS_DIR}/verification_key.json"

echo "--- Setup Complete! ---"
echo "The following artifacts have been generated in ${ARTIFACTS_DIR}:"
ls -l "$ARTIFACTS_DIR"

echo -e "\nNext steps:"
echo "1. Use '${ARTIFACTS_DIR}/${CIRCUIT_NAME}.zkey' and '${ARTIFACTS_DIR}/${CIRCUIT_NAME}.wasm' in your Rust backend (ProofOfListenService)."
echo "2. Use '${ARTIFACTS_DIR}/verification_key.json' in your verifier (smart contract or backend service)." 