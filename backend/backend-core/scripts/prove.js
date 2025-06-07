const snarkjs = require("snarkjs");

async function run() {
    // Los argumentos son: node prove.js <input_json> <wasm_path> <zkey_path>
    if (process.argv.length < 5) {
        console.error("Uso: node prove.js <input_json> <wasm_path> <zkey_path>");
        process.exit(1);
    }

    const input = JSON.parse(process.argv[2]);
    const wasmPath = process.argv[3];
    const zkeyPath = process.argv[4];

    try {
        const { proof, publicSignals } = await snarkjs.groth16.fullProve(
            input,
            wasmPath,
            zkeyPath
        );

        // Devolver el resultado como JSON al stdout para que Rust lo capture
        console.log(JSON.stringify({ proof, publicSignals }));
    } catch (error) {
        console.error("Error al generar la prueba:", error);
        process.exit(1);
    }
}

run(); 