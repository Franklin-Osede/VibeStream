// SPDX-License-Identifier: MIT
pragma solidity ^0.8.19;

import "@openzeppelin/contracts/token/ERC1155/ERC1155.sol";
import "@openzeppelin/contracts/access/AccessControl.sol";
import "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Pausable.sol";
import "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Burnable.sol";
import "@openzeppelin/contracts/token/ERC1155/extensions/ERC1155Supply.sol";

/**
 * @title LoyaltyWristband
 * @dev ERC1155 Token for Fan Loyalty Wristbands (NFTs).
 * Features:
 * - ERC1155: Supports multiple tiers (VIP, Backstage, General) in one contract.
 * - Soulbound: Can mark specific token IDs as non-transferable to prevent scalping.
 * - AccessControl: Only MINTER_ROLE (Backend) can mint.
 */
contract LoyaltyWristband is ERC1155, AccessControl, ERC1155Pausable, ERC1155Burnable, ERC1155Supply {
    bytes32 public constant URI_SETTER_ROLE = keccak256("URI_SETTER_ROLE");
    bytes32 public constant PAUSER_ROLE = keccak256("PAUSER_ROLE");
    bytes32 public constant MINTER_ROLE = keccak256("MINTER_ROLE");

    // Mapping to store if a token ID is soulbound (non-transferable)
    mapping(uint256 => bool) public isSoulbound;

    constructor() ERC1155("https://api.vibestream.com/metadata/wristband/{id}.json") {
        _grantRole(DEFAULT_ADMIN_ROLE, msg.sender);
        _grantRole(URI_SETTER_ROLE, msg.sender);
        _grantRole(PAUSER_ROLE, msg.sender);
        _grantRole(MINTER_ROLE, msg.sender);
    }

    function setURI(string memory newuri) public onlyRole(URI_SETTER_ROLE) {
        _setURI(newuri);
    }

    function pause() public onlyRole(PAUSER_ROLE) {
        _pause();
    }

    function unpause() public onlyRole(PAUSER_ROLE) {
        _unpause();
    }

    function mint(address account, uint256 id, uint256 amount, bytes memory data)
        public
        onlyRole(MINTER_ROLE)
    {
        _mint(account, id, amount, data);
    }

    function mintBatch(address to, uint256[] memory ids, uint256[] memory amounts, bytes memory data)
        public
        onlyRole(MINTER_ROLE)
    {
        _mintBatch(to, ids, amounts, data);
    }

    /**
     * @dev Sets whether a token ID is soulbound (non-transferable).
     * Only admin can set this.
     */
    function setSoulbound(uint256 id, bool _soulbound) public onlyRole(DEFAULT_ADMIN_ROLE) {
        isSoulbound[id] = _soulbound;
    }

    /**
     * @dev Hook that is called before any token transfer. This includes minting and burning, as well as batched variants.
     * We override this to enforce Soulbound logic.
     */
    function _update(address from, address to, uint256[] memory ids, uint256[] memory values)
        internal
        override(ERC1155, ERC1155Pausable, ERC1155Supply)
    {
        // Allow minting (from == 0) and burning (to == 0)
        // But restrict transfers between users (from != 0 && to != 0) if soulbound
        if (from != address(0) && to != address(0)) {
            for (uint256 i = 0; i < ids.length; i++) {
                require(!isSoulbound[ids[i]], "LoyaltyWristband: Token is soulbound and cannot be transferred");
            }
        }

        super._update(from, to, ids, values);
    }

    // The following functions are overrides required by Solidity.

    function supportsInterface(bytes4 interfaceId)
        public
        view
        override(ERC1155, AccessControl)
        returns (bool)
    {
        return super.supportsInterface(interfaceId);
    }
}
