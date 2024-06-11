// SPDX-License-Identifier: MIT
pragma solidity ^0.8.24;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol"; //The standard implementation of ERC-721 from OpenZeppelin.
import "@openzeppelin/contracts/access/Ownable.sol"; //A contract module which provides basic access control, where an account (owner) can be granted exclusive access to specific functions.

// MyNFT is an ERC-721 compliant contract
contract MyNFT is ERC721, Ownable {
    uint256 public nextTokenId; //A counter to keep track of the next token ID to be minted.
    
    constructor() ERC721("MyNFT", "MNFT") {} //Initializes the ERC-721 token with a name ("MyNFT") and a symbol ("MNFT").

    // Mint a new NFT
    function mint(address to) external onlyOwner {
        uint256 tokenId = nextTokenId;
        nextTokenId++;
        _mint(to, tokenId);
    }

    // Simplified function to transfer an NFT
    function transferNFT(address from, address to, uint256 tokenId) external {
        // Ensure the caller is the owner of the token or approved
        require(_isApprovedOrOwner(_msgSender(), tokenId), "Not approved or owner");

        // Transfer the token from the current owner to the new owner
        _transfer(from, to, tokenId);
    }
}
