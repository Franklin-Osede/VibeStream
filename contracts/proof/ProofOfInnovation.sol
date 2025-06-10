// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title ProofOfInnovation
 * @dev Simple contract to timestamp innovation proofs on Polygon
 */
contract ProofOfInnovation {
    // Event emitted when a new innovation is registered
    event Innovation(
        address indexed creator,
        bytes32 conceptHash,
        uint256 timestamp,
        string name
    );

    // Mapping to store innovation timestamps
    mapping(bytes32 => uint256) public innovationTimestamps;
    
    /**
     * @dev Register a new innovation proof
     * @param _hash Hash of the innovation documentation
     * @param _name Name of the innovation/project
     */
    function registerInnovation(bytes32 _hash, string memory _name) public {
        require(innovationTimestamps[_hash] == 0, "Innovation already registered");
        
        innovationTimestamps[_hash] = block.timestamp;
        
        emit Innovation(
            msg.sender,
            _hash,
            block.timestamp,
            _name
        );
    }

    /**
     * @dev Verify if an innovation was registered and when
     * @param _hash Hash to verify
     * @return timestamp when the innovation was registered (0 if not registered)
     */
    function verifyInnovation(bytes32 _hash) public view returns (uint256) {
        return innovationTimestamps[_hash];
    }
} 