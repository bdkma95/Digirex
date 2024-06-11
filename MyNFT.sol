// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import "@openzeppelin/contracts/access/Ownable.sol";

// MyNFT is an ERC-721 compliant contract
contract MyNFT is ERC721, Ownable {
    uint256 public nextTokenId;
    
    constructor() ERC721("MyNFT", "MNFT") {}

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
