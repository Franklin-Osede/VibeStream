pragma circom 2.2.2;

// MiMC-7 con 91 rondas optimizado para el campo Fr de bn128
template Pow7() {
    signal input in;
    signal output out;
    
    // Calculamos x^7 usando multiplicaciones cuadráticas
    // x^7 = x * x^2 * x^4
    signal x2;
    signal x4;
    signal temp;
    
    // x^2
    x2 <== in * in;
    
    // x^4
    x4 <== x2 * x2;
    
    // Multiplicación intermedia
    temp <== in * x2;
    
    // x^7 = (x * x^2) * x^4
    out <== temp * x4;
}

template MiMC7() {
    signal input in[3];  // Entrada de 3 elementos
    signal output out;   // Salida hash
    
    // Constantes para MiMC7
    var nRounds = 91;
    signal round[nRounds];
    
    // Componente para elevar a la 7
    component pow7[nRounds];
    for (var i = 0; i < nRounds; i++) {
        pow7[i] = Pow7();
    }
    
    // Primera ronda
    signal firstSum;
    firstSum <== in[0] + in[1] + in[2];
    pow7[0].in <== firstSum;
    round[0] <== pow7[0].out;
    
    // Rondas intermedias
    for (var i = 1; i < nRounds; i++) {
        pow7[i].in <== round[i-1] + i;
        round[i] <== pow7[i].out;
    }
    
    // Salida final
    out <== round[nRounds-1];
} 