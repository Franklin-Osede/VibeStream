// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/utils/ReentrancyGuard.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

/**
 * @title CrowdfundVenture
 * @dev Separate Escrow contract for a single Venture.
 * Funds are held here until the goal is met or deadline passes.
 * Security: ReentrancyGuard, PullPayment pattern for refunds.
 */
contract CrowdfundVenture is ReentrancyGuard, Ownable {
    using SafeERC20 for IERC20;

    IERC20 public immutable token; // The currency accepted (VibestreamToken or USDT/USDC)
    address public immutable artist;
    uint256 public immutable fundingGoal;
    uint256 public immutable deadline;
    
    mapping(address => uint256) public contributions;
    uint256 public totalRaised;
    
    bool public finalized;
    bool public goalReached;

    event Contributed(address indexed backer, uint256 amount);
    event Withdrawn(address indexed artist, uint256 amount);
    event Refunded(address indexed backer, uint256 amount);
    event VentureFinalized(bool success);

    constructor(
        address _token,
        address _artist,
        uint256 _fundingGoal,
        uint256 _durationInDays
    ) Ownable(msg.sender) {
        require(_token != address(0), "Invalid token");
        require(_artist != address(0), "Invalid artist");
        require(_fundingGoal > 0, "Invalid goal");
        
        token = IERC20(_token);
        artist = _artist;
        fundingGoal = _fundingGoal;
        deadline = block.timestamp + (_durationInDays * 1 days);
    }

    /**
     * @dev Contribute tokens to the venture.
     * Tokens are transferred from user to this contract.
     */
    function contribute(uint256 amount) external nonReentrant {
        require(block.timestamp < deadline, "Venture has ended");
        require(!finalized, "Venture already finalized");
        require(amount > 0, "Amount must be > 0");

        token.safeTransferFrom(msg.sender, address(this), amount);
        
        contributions[msg.sender] += amount;
        totalRaised += amount;
        
        emit Contributed(msg.sender, amount);
    }

    /**
     * @dev Check goal and finalize the state.
     * Can be called by anyone after deadline, or by owner if goal met early.
     */
    function finalize() external nonReentrant {
        require(!finalized, "Already finalized");
        require(block.timestamp >= deadline || totalRaised >= fundingGoal, "Cannot finalize yet");

        finalized = true;
        
        if (totalRaised >= fundingGoal) {
            goalReached = true;
            emit VentureFinalized(true);
        } else {
            goalReached = false;
            emit VentureFinalized(false);
        }
    }

    /**
     * @dev If successful, artist withdraws funds.
     */
    function withdrawFunds() external nonReentrant {
        require(finalized, "Not finalized");
        require(goalReached, "Goal not reached");
        require(msg.sender == artist, "Only artist can withdraw");
        
        uint256 balance = token.balanceOf(address(this));
        require(balance > 0, "No funds to withdraw");
        
        token.safeTransfer(artist, balance);
        emit Withdrawn(artist, balance);
    }

    /**
     * @dev If failed, backers withdraw their refund.
     */
    function claimRefund() external nonReentrant {
        require(finalized, "Not finalized");
        require(!goalReached, "Goal was reached, cannot refund");
        
        uint256 contributedAmount = contributions[msg.sender];
        require(contributedAmount > 0, "Nothing to refund");
        
        contributions[msg.sender] = 0;
        token.safeTransfer(msg.sender, contributedAmount);
        
        emit Refunded(msg.sender, contributedAmount);
    }
}
