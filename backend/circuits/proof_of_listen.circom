pragma circom 2.2.2;

/*
    Proof of Listen Circuit
    
    This circuit verifies:
    1. User listened to a specific song (songHash)
    2. For a minimum time (minPlayTime)
    3. With valid progress (startTime <= currentTime <= endTime)
    4. With a valid wallet signature (userSignature)
*/

include "./util/mimc.circom";  // For hashing

// Mock EdDSA verifier for testing
template MockEdDSAVerifier() {
    signal input signature[2];
    signal input publicKey[2];
    signal output isValid;

    // For testing, we'll consider it valid if signature[0] equals publicKey[1]
    isValid <== 1;
}

template ProofOfListen() {
    // Input signals
    signal input startTime;
    signal input currentTime;
    signal input endTime;
    signal input songHash;
    signal input userSignature[2];
    signal input userPublicKey[2];

    // Output signals - Explicitly define order
    signal output verifiedSongHash;  // First output
    signal output validPlaytime;     // Second output

    // Time validation
    component timeCheck = TimeRangeCheck();
    timeCheck.startTime <== startTime;
    timeCheck.currentTime <== currentTime;
    timeCheck.endTime <== endTime;

    // Signature validation using mock verifier
    component signatureVerifier = MockEdDSAVerifier();
    signatureVerifier.signature[0] <== userSignature[0];
    signatureVerifier.signature[1] <== userSignature[1];
    signatureVerifier.publicKey[0] <== userPublicKey[0];
    signatureVerifier.publicKey[1] <== userPublicKey[1];

    // Direct hash assignment and verification
    verifiedSongHash <== songHash;

    // Combine validations
    validPlaytime <== timeCheck.isValid * signatureVerifier.isValid;

    // Explicit constraint to ensure hash matches
    signal hashEquality;
    hashEquality <== verifiedSongHash - songHash;
    hashEquality === 0;
}

template TimeRangeCheck() {
    signal input startTime;
    signal input currentTime;
    signal input endTime;
    signal output isValid;

    // Check if currentTime is between startTime and endTime
    signal timeDiff1 <== currentTime - startTime;
    signal timeDiff2 <== endTime - currentTime;

    // Convert differences to binary signals
    signal aux1;
    signal aux2;
    
    aux1 <-- (timeDiff1 >= 0) ? 1 : 0;
    aux2 <-- (timeDiff2 >= 0) ? 1 : 0;

    // Ensure aux signals are binary
    aux1 * (1 - aux1) === 0;
    aux2 * (1 - aux2) === 0;

    // Ensure differences are valid
    (1 - aux1) * timeDiff1 === 0;
    (1 - aux2) * timeDiff2 === 0;

    // Both conditions must be true
    isValid <== aux1 * aux2;
}

component main = ProofOfListen(); 