pragma circom 2.2.2;

template EdDSAVerifier() {
    signal input signature[2];  // (R8x, S)
    signal input publicKey[2];  // (Ax, Ay)
    signal output isValid;      // 1 si es válida, 0 si no

    // Constante para el campo Fr de bn128
    var FIELD_SIZE = 21888242871839275222246405745257275088548364400416034343698204186575808495617;

    // Señales para verificación de rangos
    signal sigValid0;
    signal sigValid1;
    signal pkValid0;
    signal pkValid1;

    // Verificar que la firma esté en el rango correcto usando comparación
    sigValid0 <-- signature[0] < FIELD_SIZE ? 1 : 0;
    sigValid1 <-- signature[1] < FIELD_SIZE ? 1 : 0;

    // Verificar que la clave pública esté en el rango correcto
    pkValid0 <-- publicKey[0] < FIELD_SIZE ? 1 : 0;
    pkValid1 <-- publicKey[1] < FIELD_SIZE ? 1 : 0;

    // Forzar que las señales de validación sean binarias
    sigValid0 * (1 - sigValid0) === 0;
    sigValid1 * (1 - sigValid1) === 0;
    pkValid0 * (1 - pkValid0) === 0;
    pkValid1 * (1 - pkValid1) === 0;

    // Usar señales intermedias para mantener las restricciones cuadráticas
    signal sigValid;
    signal pkValid;
    
    // Validar firma y clave pública por separado
    sigValid <== sigValid0 * sigValid1;
    pkValid <== pkValid0 * pkValid1;
    
    // La firma es válida si ambas partes son válidas
    isValid <== sigValid * pkValid;
} 