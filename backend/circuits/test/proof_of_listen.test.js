const chai = require("chai");
const path = require("path");
const wasm_tester = require("circom_tester").wasm;
const F1Field = require("ffjavascript").F1Field;
const Scalar = require("ffjavascript").Scalar;
const crypto = require("crypto");
const circomlibjs = require("circomlibjs");

const assert = chai.assert;

describe("Proof of Listen Circuit", function() {
    let circuit;
    let poseidon;
    let eddsa;
    let F;

    // Constants for time validation (in seconds)
    const SONG_DURATION = 210;  // 3:30 minutes
    const MIN_LISTEN_TIME = 30; // Must listen at least 30 seconds
    
    const p = Scalar.fromString("21888242871839275222246405745257275088548364400416034343698204186575808495617");
    const Fr = new F1Field(p);

    // Increase timeout as compilation might take time
    this.timeout(100000);

    // Helper function to convert buffer to field element
    function bufferToField(buff) {
        if (Buffer.isBuffer(buff)) {
            // Convert buffer to hex string without separators
            return BigInt('0x' + buff.toString('hex')).toString();
        } else if (Array.isArray(buff)) {
            // If it's an array of numbers, convert each to hex and concatenate
            const hex = buff.map(b => b.toString(16).padStart(2, '0')).join('');
            return BigInt('0x' + hex).toString();
        }
        // If it's already a string, return as is
        return buff.toString();
    }

    // Helper function to create EdDSA signature
    async function createEdDSASignature(privateKey, message) {
        try {
            const msgBuff = Buffer.from(message);
            // Use poseidon array input format
            const msgHash = await poseidon([msgBuff]);
            const signature = await eddsa.signPoseidon(privateKey, msgHash);
            
            console.log("Debug - Raw signature:", {
                R8: signature.R8,
                S: signature.S,
                hash: msgHash
            });

            return {
                R8: [bufferToField(signature.R8[0])],
                S: bufferToField(signature.S),
                hash: bufferToField(msgHash)
            };
        } catch (error) {
            console.error("Error creating signature:", error);
            throw error;
        }
    }

    // Helper function to convert field element to string
    function fieldToString(element) {
        if (typeof element === 'bigint') {
            return element.toString();
        }
        if (Array.isArray(element)) {
            return element.map(e => e.toString()).join(',');
        }
        if (Buffer.isBuffer(element)) {
            return BigInt('0x' + element.toString('hex')).toString();
        }
        return element.toString();
    }

    before(async function () {
        try {
            // First build poseidon and eddsa
            poseidon = await circomlibjs.buildPoseidon();
            eddsa = await circomlibjs.buildEddsa();
            F = eddsa.F;
            
            // Then compile the circuit
            circuit = await wasm_tester(path.join(__dirname, "../proof_of_listen.circom"));
        } catch (error) {
            console.error("Error in setup:", error);
            throw error;
        }
    });

    it("Should verify valid listening session with realistic duration", async function() {
        const startTime = 1000; // Some timestamp
        const currentTime = startTime + 120; // 2 minutes into song
        const endTime = startTime + SONG_DURATION;

        const input = {
            startTime: startTime.toString(),
            currentTime: currentTime.toString(),
            endTime: endTime.toString(),
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"],
            messageHash: "789"
        };

        console.log("Debug - Input values:", {
            ...input,
            duration: SONG_DURATION,
            timeElapsed: currentTime - startTime,
            timeRemaining: endTime - currentTime
        });
        
        const witness = await circuit.calculateWitness(input);
        await circuit.checkConstraints(witness);
        
        const verifiedSongHash = witness[1];
        const validPlaytime = witness[2];
        
        assert.equal(verifiedSongHash.toString(), input.songHash, "verifiedSongHash should match input songHash");
        assert.equal(validPlaytime.toString(), "1", "validPlaytime should be 1 for valid duration");
    });

    it("Should reject when listening time is too short", async function() {
        const startTime = 1000;
        const currentTime = startTime + 15; // Only 15 seconds (less than minimum)
        const endTime = startTime + SONG_DURATION;

        const input = {
            startTime: startTime.toString(),
            currentTime: currentTime.toString(),
            endTime: endTime.toString(),
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"],
            messageHash: "789"
        };

        console.log("Debug - Short listen duration:", {
            timeElapsed: currentTime - startTime,
            minimumRequired: MIN_LISTEN_TIME
        });

        const witness = await circuit.calculateWitness(input);
        const validPlaytime = witness[2];
        
        assert.equal(validPlaytime.toString(), "0", "validPlaytime should be 0 when listen time is too short");
    });

    it("Should reject when currentTime exceeds song duration", async function() {
        const startTime = 1000;
        const currentTime = startTime + SONG_DURATION + 60; // 1 minute after song ends
        const endTime = startTime + SONG_DURATION;

        const input = {
            startTime: startTime.toString(),
            currentTime: currentTime.toString(),
            endTime: endTime.toString(),
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"],
            messageHash: "789"
        };

        console.log("Debug - Exceeded duration:", {
            songDuration: SONG_DURATION,
            timeElapsed: currentTime - startTime,
            overTime: currentTime - endTime
        });

        const witness = await circuit.calculateWitness(input);
        const validPlaytime = witness[2];
        
        assert.equal(validPlaytime.toString(), "0", "validPlaytime should be 0 when exceeding song duration");
    });

    it("Should handle exact boundary times", async function() {
        const startTime = 1000;
        const currentTime = startTime + MIN_LISTEN_TIME; // Exactly minimum time
        const endTime = startTime + SONG_DURATION;

        const input = {
            startTime: startTime.toString(),
            currentTime: currentTime.toString(),
            endTime: endTime.toString(),
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"],
            messageHash: "789"
        };

        console.log("Debug - Boundary case:", {
            minimumRequired: MIN_LISTEN_TIME,
            actualListenTime: currentTime - startTime
        });

        const witness = await circuit.calculateWitness(input);
        const validPlaytime = witness[2];
        
        assert.equal(validPlaytime.toString(), "1", "validPlaytime should be 1 when exactly at minimum time");
    });
}); 