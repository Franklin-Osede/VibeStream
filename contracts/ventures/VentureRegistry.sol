// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "./CrowdfundVenture.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title VentureRegistry
 * @dev Factory contract to deploy and track Ventures.
 * Ensures that only the platform can create official Ventures.
 */
contract VentureRegistry is Ownable {
    // Array of all deployed venture contracts
    address[] public deployedVentures;

    // Mapping to check if an address is a valid venture deployed by this factory
    mapping(address => bool) public isValidVenture;

    event VentureCreated(
        address indexed ventureAddress,
        address indexed artist,
        uint256 fundingGoal,
        uint256 deadline
    );

    constructor() Ownable(msg.sender) {}

    /**
     * @dev Create a new Venture.
     * @param _token The currency token address (e.g. VibestreamToken)
     * @param _artist The artist receiving the funds
     * @param _fundingGoal The amount to raise
     * @param _durationInDays How long the campaign lasts
     */
    function createVenture(
        address _token,
        address _artist,
        uint256 _fundingGoal,
        uint256 _durationInDays
    ) external onlyOwner returns (address) {
        CrowdfundVenture newVenture = new CrowdfundVenture(
            _token,
            _artist,
            _fundingGoal,
            _durationInDays
        );

        address ventureAddr = address(newVenture);
        deployedVentures.push(ventureAddr);
        isValidVenture[ventureAddr] = true;

        emit VentureCreated(
            ventureAddr,
            _artist,
            _fundingGoal,
            _durationInDays
        );

        return ventureAddr;
    }

    /**
     * @dev Return all deployed ventures.
     */
    function getVentures() external view returns (address[] memory) {
        return deployedVentures;
    }
}
