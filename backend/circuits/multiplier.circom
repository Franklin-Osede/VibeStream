pragma circom 2.2.2;

/*
    Este es un circuito simple que demuestra que c es el resultado
    de multiplicar a por b, sin revelar los valores de a y b.
*/

template Multiplier() {
    // Declaración de señales privadas de entrada
    signal input a;
    signal input b;
    
    // Declaración de señal pública de salida
    signal output c;
    
    // Restricción que define la multiplicación
    c <== a * b;
}

component main = Multiplier(); 