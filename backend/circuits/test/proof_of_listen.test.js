const chai = require("chai");
const path = require("path");
const wasm_tester = require("circom_tester").wasm;
const F1Field = require("ffjavascript").F1Field;
const Scalar = require("ffjavascript").Scalar;

const assert = chai.assert;

describe("Proof of Listen Circuit", function() {
    let circuit;
    const p = Scalar.fromString("21888242871839275222246405745257275088548364400416034343698204186575808495617");
    const Fr = new F1Field(p);

    // Increase timeout as compilation might take time
    this.timeout(100000);

    before(async function () {
        circuit = await wasm_tester(path.join(__dirname, "../proof_of_listen.circom"));
    });

    it("Should verify valid listening session with direct values", async function() {
        const input = {
            startTime: "10",
            currentTime: "20",
            endTime: "30",
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"]
        };

        console.log("Debug - Input values:", input);
        
        const witness = await circuit.calculateWitness(input);
        await circuit.checkConstraints(witness);
        
        // Log all witness values for debugging
        console.log("Debug - Witness array:");
        witness.forEach((value, index) => {
            console.log(`witness[${index}] = ${value.toString()}`);
        });
        
        // Verify each output signal - note the order
        const verifiedSongHash = witness[1];  // First output
        const validPlaytime = witness[2];     // Second output
        
        console.log("Debug - Output signals:");
        console.log("verifiedSongHash:", verifiedSongHash.toString());
        console.log("validPlaytime:", validPlaytime.toString());
        console.log("Expected songHash:", input.songHash);
        
        // Assert the outputs
        assert.equal(
            verifiedSongHash.toString(),
            input.songHash,
            "verifiedSongHash should match input songHash"
        );
        assert.equal(
            validPlaytime.toString(),
            "1",
            "validPlaytime should be 1"
        );
    });

    it("Should verify valid listening session with field elements", async function() {
        const input = {
            startTime: Fr.e("10"),
            currentTime: Fr.e("20"),
            endTime: Fr.e("30"),
            songHash: Fr.e("42"),
            userSignature: [Fr.e("123"), Fr.e("456")],
            userPublicKey: [Fr.e("111"), Fr.e("222")]
        };

        console.log("Debug - Field element input:", {
            startTime: Fr.toString(input.startTime),
            currentTime: Fr.toString(input.currentTime),
            endTime: Fr.toString(input.endTime),
            songHash: Fr.toString(input.songHash),
            userSignature: input.userSignature.map(x => Fr.toString(x)),
            userPublicKey: input.userPublicKey.map(x => Fr.toString(x))
        });
        
        const witness = await circuit.calculateWitness(input);
        await circuit.checkConstraints(witness);
        
        // Log witness values in field element form - note the order
        const verifiedSongHash = Fr.e(witness[1]);  // First output
        const validPlaytime = Fr.e(witness[2]);     // Second output
        
        console.log("Debug - Field element outputs:");
        console.log("verifiedSongHash:", Fr.toString(verifiedSongHash));
        console.log("validPlaytime:", Fr.toString(validPlaytime));
        console.log("Expected songHash:", Fr.toString(input.songHash));
        
        // Assert using field element comparison
        assert(
            Fr.eq(verifiedSongHash, input.songHash),
            "verifiedSongHash should match input songHash"
        );
        assert(
            Fr.eq(validPlaytime, Fr.e(1)),
            "validPlaytime should be 1"
        );
    });

    // Tests para casos inválidos
    it("Should reject when currentTime is before startTime", async function() {
        const input = {
            startTime: "20",      // currentTime < startTime
            currentTime: "10",
            endTime: "30",
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"]
        };

        const witness = await circuit.calculateWitness(input);
        const validPlaytime = witness[2];
        
        assert.equal(validPlaytime.toString(), "0", "validPlaytime should be 0 when currentTime < startTime");
    });

    it("Should reject when currentTime is after endTime", async function() {
        const input = {
            startTime: "10",
            currentTime: "40",    // currentTime > endTime
            endTime: "30",
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"]
        };

        const witness = await circuit.calculateWitness(input);
        const validPlaytime = witness[2];
        
        assert.equal(validPlaytime.toString(), "0", "validPlaytime should be 0 when currentTime > endTime");
    });

    it("Should verify song hash even when time is invalid", async function() {
        const input = {
            startTime: "30",      // Invalid time range
            currentTime: "10",
            endTime: "20",
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"]
        };

        const witness = await circuit.calculateWitness(input);
        const verifiedSongHash = witness[1];
        const validPlaytime = witness[2];
        
        assert.equal(verifiedSongHash.toString(), input.songHash, "verifiedSongHash should match even with invalid time");
        assert.equal(validPlaytime.toString(), "0", "validPlaytime should be 0 with invalid time");
    });

    it("Should handle edge cases in time validation", async function() {
        const input = {
            startTime: "20",
            currentTime: "20",    // currentTime equals startTime
            endTime: "20",        // endTime equals currentTime
            songHash: "42",
            userSignature: ["123", "456"],
            userPublicKey: ["111", "222"]
        };

        const witness = await circuit.calculateWitness(input);
        const validPlaytime = witness[2];
        
        assert.equal(validPlaytime.toString(), "1", "validPlaytime should be 1 when all times are equal");
    });
}); 