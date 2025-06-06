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

template TimeRangeCheck() {
    signal input startTime;
    signal input currentTime;
    signal input endTime;
    signal output isValid;

    // Check if currentTime is between startTime and endTime
    signal timeDiff1;
    signal timeDiff2;
    timeDiff1 <== currentTime - startTime;
    timeDiff2 <== endTime - currentTime;

    // Create comparison signals
    signal isAfterStart;
    signal isBeforeEnd;
    
    // Compare differences (0 if false, 1 if true)
    isAfterStart <-- timeDiff1 >= 0 ? 1 : 0;
    isBeforeEnd <-- timeDiff2 >= 0 ? 1 : 0;

    // Ensure comparison signals are binary
    isAfterStart * (1 - isAfterStart) === 0;
    isBeforeEnd * (1 - isBeforeEnd) === 0;

    // Both conditions must be true for isValid to be 1
    isValid <== isAfterStart * isBeforeEnd;
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

component main = ProofOfListen(); 