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

    const SONG_DURATION = 210;
    const MIN_LISTEN_TIME = 30;
    
    // --- Helper function to generate valid inputs for all tests ---
    async function generateValidInputs() {
        const privateKey = crypto.randomBytes(32);
        const publicKey = eddsa.prv2pub(privateKey);
        
        // The data that will be signed
        const songHash = "42";
        const nonce = F.e(Date.now()); // A unique value for each signature
        const message = poseidon([songHash, F.toObject(publicKey[0]), F.toObject(nonce)]);

        const signature = eddsa.signPoseidon(privateKey, message);

        const bufferToString = (buf) => eddsa.F.toObject(buf).toString();

        return {
            startTime: "1000",
            currentTime: "1050",
            endTime: (1000 + SONG_DURATION).toString(),
            songHash: songHash,
            userSignature: [
                bufferToString(signature.R8[0]),
                bufferToString(signature.R8[1]),
                signature.S.toString()
            ],
            userPublicKey: [
                bufferToString(publicKey[0]),
                bufferToString(publicKey[1])
            ],
            nonce: bufferToString(nonce)
        };
    }

    this.timeout(200000);

    before(async function () {
        poseidon = await circomlibjs.buildPoseidon();
        eddsa = await circomlibjs.buildEddsa();
        F = eddsa.F;
        circuit = await wasm_tester(
            path.join(__dirname, "../proof_of_listen.circom"),
            { include: [path.join(__dirname, "../../node_modules/circomlib/circuits")] }
        );
    });

    context("when verifying time constraints", function() {
        it("Should verify a valid listening session", async function() {
            const input = await generateValidInputs();
            input.currentTime = (parseInt(input.startTime) + 120).toString(); // 2 mins

            const witness = await circuit.calculateWitness(input);
            await circuit.checkConstraints(witness);
            const validPlaytime = witness[2];
            assert.equal(validPlaytime.toString(), "1", "Playtime should be valid");
        });

        it("Should reject when listening time is too short", async function() {
            const input = await generateValidInputs();
            input.currentTime = (parseInt(input.startTime) + 15).toString(); // Too short

            const witness = await circuit.calculateWitness(input);
            const validPlaytime = witness[2];
            assert.equal(validPlaytime.toString(), "0", "Playtime should be invalid when listen time is too short");
        });

        it("Should reject when currentTime exceeds song duration", async function() {
            const input = await generateValidInputs();
            input.currentTime = (parseInt(input.endTime) + 60).toString(); // Exceeded

            const witness = await circuit.calculateWitness(input);
            const validPlaytime = witness[2];
            assert.equal(validPlaytime.toString(), "0", "Playtime should be invalid when exceeding song duration");
        });

        it("Should handle exact boundary times", async function() {
            const input = await generateValidInputs();
            input.currentTime = (parseInt(input.startTime) + MIN_LISTEN_TIME).toString(); // Boundary

            const witness = await circuit.calculateWitness(input);
            await circuit.checkConstraints(witness);
            const validPlaytime = witness[2];
            assert.equal(validPlaytime.toString(), "1", "Playtime should be valid at the boundary");
        });
    });

    context("when verifying EdDSA signature", function() {
        it("Should successfully pass with a valid signature", async function() {
            const input = await generateValidInputs();
            const witness = await circuit.calculateWitness(input);
            await circuit.checkConstraints(witness);
            const validPlaytime = witness[2];
            assert.equal(validPlaytime.toString(), "1", "Playtime should be valid with a correct signature");
        });

        it("Should fail when the signature is invalid", async function() {
            const input = await generateValidInputs();
            // Corrupt the signature
            input.userSignature[2] = (BigInt(input.userSignature[2]) + 1n).toString();

            try {
                await circuit.calculateWitness(input);
                assert.fail("Should have thrown an error for invalid signature");
            } catch (error) {
                assert.include(error.message, "Error: Assert Failed", "Should fail with a constraint assertion error");
            }
        });

        it("Should fail when the nonce is incorrect (replay attack)", async function() {
            const input = await generateValidInputs();
            // Change the nonce, simulating a replay attack where the signature is valid
            // but the nonce doesn't match the one expected by the verifier for this session.
            input.nonce = (BigInt(input.nonce) + 1n).toString();

            try {
                await circuit.calculateWitness(input);
                assert.fail("Should have thrown an error for incorrect nonce");
            } catch (error) {
                assert.include(error.message, "Error: Assert Failed", "Should fail with a constraint assertion error");
            }
        });
    });
}); 