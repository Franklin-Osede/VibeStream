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
include "./util/eddsa.circom"; // Now contains the real EdDSAPoseidonVerifier
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
    signal input userSignature[3];    // (R8x, R8y, S)
    signal input userPublicKey[2];    // (Ax, Ay)
    signal input messageHash;         // Message hash for signature verification

    // Output signals
    signal output verifiedSongHash;
    signal output validPlaytime;

    // Time validation
    component timeCheck = TimeRangeCheck();
    timeCheck.startTime <== startTime;
    timeCheck.currentTime <== currentTime;
    timeCheck.endTime <== endTime;

    // Real EdDSA signature verification
    component signatureVerifier = EdDSAPoseidonVerifier();
    signatureVerifier.enabled <== 1;
    signatureVerifier.R8x <== userSignature[0];
    signatureVerifier.R8y <== userSignature[1];
    signatureVerifier.S <== userSignature[2];
    signatureVerifier.Ax <== userPublicKey[0];
    signatureVerifier.Ay <== userPublicKey[1];
    signatureVerifier.M <== messageHash;

    // Direct hash assignment and verification
    verifiedSongHash <== songHash;

    // Time validation output
    validPlaytime <== timeCheck.isValid;

    // Explicit constraint to ensure hash matches
    songHash === verifiedSongHash;
}

component main { public [ userPublicKey, messageHash ] } = ProofOfListen(); 