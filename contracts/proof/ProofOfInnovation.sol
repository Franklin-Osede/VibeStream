// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

/**
 * @title ProofOfInnovation
 * @dev Secure contract to timestamp innovation proofs on Polygon
 * @notice This contract provides immutable proof-of-existence for innovations
 * @author VibeStream Team
 */
contract ProofOfInnovation {
    // =============================================================================
    // EVENTS
    // =============================================================================
    
    /// @notice Emitted when a new innovation is registered
    /// @param creator Address that registered the innovation
    /// @param conceptHash Hash of the innovation documentation (indexed for filtering)
    /// @param timestamp Block timestamp when registered
    /// @param name Human-readable name of the innovation
    event InnovationRegistered(
        address indexed creator,
        bytes32 indexed conceptHash,
        uint256 timestamp,
        string name
    );

    // =============================================================================
    // STATE VARIABLES
    // =============================================================================
    
    /// @notice Mapping from concept hash to registration timestamp
    /// @dev Returns 0 if innovation is not registered
    mapping(bytes32 => uint256) public innovationTimestamps;
    
    /// @notice Mapping from concept hash to creator address
    /// @dev Allows querying who registered a specific innovation
    mapping(bytes32 => address) public innovationCreators;
    
    /// @notice Total number of innovations registered
    uint256 public totalInnovations;
    
    /// @notice Owner of the contract (can pause in emergency)
    address public owner;
    
    /// @notice Whether the contract is paused
    bool public paused;

    // =============================================================================
    // MODIFIERS
    // =============================================================================
    
    /// @dev Ensures only owner can call
    modifier onlyOwner() {
        require(msg.sender == owner, "ProofOfInnovation: caller is not owner");
        _;
    }
    
    /// @dev Ensures contract is not paused
    modifier whenNotPaused() {
        require(!paused, "ProofOfInnovation: contract is paused");
        _;
    }
    
    /// @dev Ensures hash is not zero
    modifier validHash(bytes32 _hash) {
        require(_hash != bytes32(0), "ProofOfInnovation: hash cannot be zero");
        _;
    }

    // =============================================================================
    // CONSTRUCTOR
    // =============================================================================
    
    /// @dev Sets the contract deployer as owner
    constructor() {
        owner = msg.sender;
        paused = false;
        totalInnovations = 0;
    }

    // =============================================================================
    // MAIN FUNCTIONS
    // =============================================================================
    
    /**
     * @dev Register a new innovation proof
     * @param _hash Hash of the innovation documentation (must be SHA256 or similar)
     * @param _name Human-readable name of the innovation/project
     * @notice The hash must be unique. Cannot register the same hash twice.
     * @notice Emits InnovationRegistered event
     */
    function registerInnovation(
        bytes32 _hash,
        string memory _name
    ) 
        public 
        whenNotPaused 
        validHash(_hash)
    {
        // Check if innovation already exists
        require(
            innovationTimestamps[_hash] == 0,
            "ProofOfInnovation: innovation already registered"
        );
        
        // Validate name is not empty
        require(
            bytes(_name).length > 0,
            "ProofOfInnovation: name cannot be empty"
        );
        
        // Validate name length (prevent gas griefing)
        require(
            bytes(_name).length <= 200,
            "ProofOfInnovation: name too long (max 200 chars)"
        );
        
        // Store timestamp and creator
        innovationTimestamps[_hash] = block.timestamp;
        innovationCreators[_hash] = msg.sender;
        totalInnovations++;
        
        // Emit event with indexed fields for efficient filtering
        emit InnovationRegistered(
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
     * @return creator address of who registered it (address(0) if not registered)
     */
    function verifyInnovation(bytes32 _hash) 
        public 
        view 
        returns (uint256 timestamp, address creator) 
    {
        timestamp = innovationTimestamps[_hash];
        creator = innovationCreators[_hash];
    }
    
    /**
     * @dev Check if an innovation is registered
     * @param _hash Hash to check
     * @return true if registered, false otherwise
     */
    function isRegistered(bytes32 _hash) public view returns (bool) {
        return innovationTimestamps[_hash] != 0;
    }

    // =============================================================================
    // ADMIN FUNCTIONS
    // =============================================================================
    
    /**
     * @dev Pause the contract (emergency only)
     * @notice Only owner can pause. Prevents new registrations.
     */
    function pause() public onlyOwner {
        paused = true;
    }
    
    /**
     * @dev Unpause the contract
     * @notice Only owner can unpause. Re-enables registrations.
     */
    function unpause() public onlyOwner {
        paused = false;
    }
    
    /**
     * @dev Transfer ownership to a new address
     * @param newOwner Address of the new owner
     * @notice Only current owner can transfer ownership
     */
    function transferOwnership(address newOwner) public onlyOwner {
        require(
            newOwner != address(0),
            "ProofOfInnovation: new owner cannot be zero address"
        );
        require(
            newOwner != owner,
            "ProofOfInnovation: new owner must be different"
        );
        owner = newOwner;
    }

    // =============================================================================
    // VIEW FUNCTIONS
    // =============================================================================
    
    /**
     * @dev Get total number of registered innovations
     * @return count of registered innovations
     */
    function getTotalInnovations() public view returns (uint256) {
        return totalInnovations;
    }
}
