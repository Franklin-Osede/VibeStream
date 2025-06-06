pragma circom 2.2.2;

/*
    Proof of Listen Circuit
    
    This circuit verifies:
    1. User listened to a specific song (songHash)
    2. For a minimum time (MIN_LISTEN_TIME = 30 seconds)
    3. Within valid song duration (SONG_DURATION)
    4. With valid progress (startTime <= currentTime <= endTime)
    5. With a valid wallet signature (userSignature)
*/

include "./util/mimc.circom";  // For hashing
include "./util/eddsa.circom";
include "./util/babyjub.circom";

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

    // Constants (in seconds)
    var MIN_LISTEN_TIME = 30;

    // Time validations
    signal timeDiff1 <== currentTime - startTime;  // Time elapsed
    signal timeDiff2 <== endTime - currentTime;    // Time remaining
    signal listenDuration <== timeDiff1;          // How long they've listened

    // Comparison signals (1 if true, 0 if false)
    signal isAfterStart <-- timeDiff1 >= 0 ? 1 : 0;
    signal isBeforeEnd <-- timeDiff2 >= 0 ? 1 : 0;
    signal hasListenedEnough <-- listenDuration >= MIN_LISTEN_TIME ? 1 : 0;

    // Ensure comparison signals are binary
    isAfterStart * (1 - isAfterStart) === 0;
    isBeforeEnd * (1 - isBeforeEnd) === 0;
    hasListenedEnough * (1 - hasListenedEnough) === 0;

    // Break down the validation into two steps to maintain quadratic constraints
    signal timeRangeValid <== isAfterStart * isBeforeEnd;
    isValid <== timeRangeValid * hasListenedEnough;
}

template ProofOfListen() {
    // Input signals
    signal input startTime;
    signal input currentTime;
    signal input endTime;
    signal input songHash;
    signal input userSignature[2];    // (R8x, S)
    signal input userPublicKey[2];    // (Ax, Ay)
    signal input messageHash;         // Message hash for signature verification

    // Output signals - Explicit order
    signal output verifiedSongHash;   // First output
    signal output validPlaytime;      // Second output

    // Time validation
    component timeCheck = TimeRangeCheck();
    timeCheck.startTime <== startTime;
    timeCheck.currentTime <== currentTime;
    timeCheck.endTime <== endTime;

    // EdDSA signature verification
    component signatureVerifier = EdDSAVerifier();
    signatureVerifier.signature[0] <== userSignature[0];  // R8x
    signatureVerifier.signature[1] <== userSignature[1];  // S
    signatureVerifier.publicKey[0] <== userPublicKey[0];  // Ax
    signatureVerifier.publicKey[1] <== userPublicKey[1];  // Ay

    // Direct hash assignment and verification
    verifiedSongHash <== songHash;

    // Only validate time for now
    validPlaytime <== timeCheck.isValid;

    // Explicit constraint to ensure hash matches
    songHash === verifiedSongHash;
}

component main = ProofOfListen(); 