const hre = require("hardhat");
const fs = require('fs');
const crypto = require('crypto');

async function main() {
  // Generate hash of the project documentation
  const projectDocs = fs.readFileSync('docs/PROJECT_STRUCTURE.md', 'utf8');
  const conceptHash = '0x' + crypto
    .createHash('sha256')
    .update(projectDocs)
    .digest('hex');

  console.log("Documentation hash:", conceptHash);

  // Deploy the contract
  const ProofOfInnovation = await hre.ethers.getContractFactory("ProofOfInnovation");
  const proof = await ProofOfInnovation.deploy();
  await proof.deployed();

  console.log("ProofOfInnovation deployed to:", proof.address);

  // Register the innovation
  const tx = await proof.registerInnovation(conceptHash, "VibeStream");
  await tx.wait();

  console.log("Innovation registered! Transaction:", tx.hash);
  
  // Verify the registration
  const timestamp = await proof.verifyInnovation(conceptHash);
  console.log("Verified timestamp:", new Date(timestamp * 1000).toISOString());
}

main()
  .then(() => process.exit(0))
  .catch((error) => {
    console.error(error);
    process.exit(1);
  }); 