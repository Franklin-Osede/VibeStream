const { expect } = require("chai");
const { ethers } = require("hardhat");

describe("Vibestream Contracts", function () {
  let token, wristband, registry;
  let owner, user1, user2;

  before(async function () {
    [owner, user1, user2] = await ethers.getSigners();
  });

  describe("VibestreamToken", function () {
    it("Should deploy with correct name and symbol", async function () {
      const Token = await ethers.getContractFactory("VibestreamToken");
      token = await Token.deploy();
      await token.waitForDeployment();
      
      expect(await token.name()).to.equal("Vibestream");
      expect(await token.symbol()).to.equal("VIBE");
    });

    it("Should allow admin to mint tokens", async function () {
      await token.mint(user1.address, ethers.parseEther("100"));
      expect(await token.balanceOf(user1.address)).to.equal(ethers.parseEther("100"));
    });
  });

  describe("LoyaltyWristband", function () {
    it("Should deploy and support soulbound check", async function () {
      const Wristband = await ethers.getContractFactory("LoyaltyWristband");
      wristband = await Wristband.deploy();
      await wristband.waitForDeployment();

      // Mint a wristband to user1
      await wristband.mint(user1.address, 1, 1, "0x");
      expect(await wristband.balanceOf(user1.address, 1)).to.equal(1);
    });

    it("Should prevent transfer of soulbound tokens", async function () {
      // Mark ID 1 as soulbound
      await wristband.setSoulbound(1, true);
      
      // Try to transfer from user1 to user2 (Should fail)
      await expect(
        wristband.connect(user1).safeTransferFrom(user1.address, user2.address, 1, 1, "0x")
      ).to.be.revertedWith("LoyaltyWristband: Token is soulbound and cannot be transferred");
    });
  });
});
