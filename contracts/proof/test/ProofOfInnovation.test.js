const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("ProofOfInnovation", function () {
  let proofOfInnovation;
  let owner;
  let addr1;
  let addr2;

  beforeEach(async function () {
    [owner, addr1, addr2] = await ethers.getSigners();

    const ProofOfInnovation = await ethers.getContractFactory("ProofOfInnovation");
    proofOfInnovation = await ProofOfInnovation.deploy();
    await proofOfInnovation.deployed();
  });

  describe("Deployment", function () {
    it("Should set the right owner", async function () {
      expect(await proofOfInnovation.owner()).to.equal(owner.address);
    });

    it("Should start unpaused", async function () {
      expect(await proofOfInnovation.paused()).to.equal(false);
    });

    it("Should start with zero innovations", async function () {
      expect(await proofOfInnovation.totalInnovations()).to.equal(0);
    });
  });

  describe("Registration", function () {
    const testHash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("test innovation"));
    const testName = "Test Innovation";

    it("Should register innovation successfully", async function () {
      await expect(proofOfInnovation.connect(addr1).registerInnovation(testHash, testName))
        .to.emit(proofOfInnovation, "InnovationRegistered")
        .withArgs(addr1.address, testHash, anyValue, testName);

      const timestamp = await proofOfInnovation.innovationTimestamps(testHash);
      expect(timestamp).to.be.gt(0);

      const creator = await proofOfInnovation.innovationCreators(testHash);
      expect(creator).to.equal(addr1.address);

      expect(await proofOfInnovation.totalInnovations()).to.equal(1);
    });

    it("Should reject duplicate hash", async function () {
      await proofOfInnovation.connect(addr1).registerInnovation(testHash, testName);
      
      await expect(
        proofOfInnovation.connect(addr2).registerInnovation(testHash, "Different Name")
      ).to.be.revertedWith("ProofOfInnovation: innovation already registered");
    });

    it("Should reject zero hash", async function () {
      await expect(
        proofOfInnovation.connect(addr1).registerInnovation(ethers.constants.HashZero, testName)
      ).to.be.revertedWith("ProofOfInnovation: hash cannot be zero");
    });

    it("Should reject empty name", async function () {
      await expect(
        proofOfInnovation.connect(addr1).registerInnovation(testHash, "")
      ).to.be.revertedWith("ProofOfInnovation: name cannot be empty");
    });

    it("Should reject name too long", async function () {
      const longName = "a".repeat(201);
      await expect(
        proofOfInnovation.connect(addr1).registerInnovation(testHash, longName)
      ).to.be.revertedWith("ProofOfInnovation: name too long");
    });

    it("Should allow different users to register different hashes", async function () {
      const hash1 = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("innovation 1"));
      const hash2 = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("innovation 2"));

      await proofOfInnovation.connect(addr1).registerInnovation(hash1, "Innovation 1");
      await proofOfInnovation.connect(addr2).registerInnovation(hash2, "Innovation 2");

      expect(await proofOfInnovation.totalInnovations()).to.equal(2);
      expect(await proofOfInnovation.isRegistered(hash1)).to.equal(true);
      expect(await proofOfInnovation.isRegistered(hash2)).to.equal(true);
    });
  });

  describe("Verification", function () {
    const testHash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("test"));
    const testName = "Test";

    it("Should verify registered innovation", async function () {
      await proofOfInnovation.connect(addr1).registerInnovation(testHash, testName);
      
      const [timestamp, creator] = await proofOfInnovation.verifyInnovation(testHash);
      expect(timestamp).to.be.gt(0);
      expect(creator).to.equal(addr1.address);
    });

    it("Should return zero for unregistered hash", async function () {
      const unregisteredHash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("unregistered"));
      const [timestamp, creator] = await proofOfInnovation.verifyInnovation(unregisteredHash);
      expect(timestamp).to.equal(0);
      expect(creator).to.equal(ethers.constants.AddressZero);
    });

    it("Should return false for unregistered hash", async function () {
      const unregisteredHash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("unregistered"));
      expect(await proofOfInnovation.isRegistered(unregisteredHash)).to.equal(false);
    });
  });

  describe("Pause/Unpause", function () {
    const testHash = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("test"));
    const testName = "Test";

    it("Should allow owner to pause", async function () {
      await proofOfInnovation.pause();
      expect(await proofOfInnovation.paused()).to.equal(true);
    });

    it("Should prevent registration when paused", async function () {
      await proofOfInnovation.pause();
      await expect(
        proofOfInnovation.connect(addr1).registerInnovation(testHash, testName)
      ).to.be.revertedWith("ProofOfInnovation: contract is paused");
    });

    it("Should allow owner to unpause", async function () {
      await proofOfInnovation.pause();
      await proofOfInnovation.unpause();
      expect(await proofOfInnovation.paused()).to.equal(false);
    });

    it("Should allow registration after unpause", async function () {
      await proofOfInnovation.pause();
      await proofOfInnovation.unpause();
      await expect(
        proofOfInnovation.connect(addr1).registerInnovation(testHash, testName)
      ).to.emit(proofOfInnovation, "InnovationRegistered");
    });

    it("Should reject pause from non-owner", async function () {
      await expect(
        proofOfInnovation.connect(addr1).pause()
      ).to.be.revertedWith("ProofOfInnovation: caller is not owner");
    });
  });

  describe("Ownership", function () {
    it("Should allow owner to transfer ownership", async function () {
      await proofOfInnovation.transferOwnership(addr1.address);
      expect(await proofOfInnovation.owner()).to.equal(addr1.address);
    });

    it("Should reject transfer to zero address", async function () {
      await expect(
        proofOfInnovation.transferOwnership(ethers.constants.AddressZero)
      ).to.be.revertedWith("ProofOfInnovation: new owner cannot be zero address");
    });

    it("Should reject transfer to same owner", async function () {
      await expect(
        proofOfInnovation.transferOwnership(owner.address)
      ).to.be.revertedWith("ProofOfInnovation: new owner must be different");
    });

    it("Should reject transfer from non-owner", async function () {
      await expect(
        proofOfInnovation.connect(addr1).transferOwnership(addr2.address)
      ).to.be.revertedWith("ProofOfInnovation: caller is not owner");
    });
  });

  describe("View Functions", function () {
    it("Should return total innovations count", async function () {
      const hash1 = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("1"));
      const hash2 = ethers.utils.keccak256(ethers.utils.toUtf8Bytes("2"));

      await proofOfInnovation.connect(addr1).registerInnovation(hash1, "One");
      expect(await proofOfInnovation.getTotalInnovations()).to.equal(1);

      await proofOfInnovation.connect(addr2).registerInnovation(hash2, "Two");
      expect(await proofOfInnovation.getTotalInnovations()).to.equal(2);
    });
  });
});

// Helper para anyValue en eventos
function anyValue() {
  return true;
}

