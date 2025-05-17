# CSDS: Cybersecurity Data Sharing on Solana

## Overview

CSDS (Cybersecurity Data Sharing) is a Solana-based smart contract program designed to securely create, share, and revoke access to cybersecurity reports using soulbound NFTs (non-transferable tokens). Leveraging Solana's high-speed blockchain, Metaplex Core for NFT management, and Pinata for decentralized storage on IPFS, CSDS enables organizations to share sensitive reports with specific entities while maintaining control over access.

This project was developed as part of a hackathon to demonstrate secure data sharing in the cybersecurity domain, ensuring transparency, immutability, and fine-grained access control.

---

## Features

### Create Reports with Soulbound NFTs

Initializes a new cybersecurity report by creating a Metaplex Core collection and an owner NFT. It stores report metadata (report ID, name, organization) on-chain, links to an IPFS URI via Pinata, and locks the NFT as soulbound using a FreezeDelegate plugin. Validates input lengths and sets up PDAs for report_collection and report_data.

[related instruction](programs/csds_contracts/src/instructions/soulbound/create_report.rs)

### Share Reports with Organizations

Shares an existing report with a specific organization by minting a share NFT. Verifies the creator’s authority and report ID, creates the NFT with the report’s content URI, and updates report_data to reflect the shared organization’s public key.

[related instruction](programs/csds_contracts/src/instructions/soulbound/share_report.rs)


### Revoke Access

Revokes access to a shared report by burning the associated share NFT. Validates the creator’s authority, ensures the NFT is a share (not owner), and closes the report_data account, returning funds to the creator.

[related instruction](programs/csds_contracts/src/instructions/soulbound/revoke_share.rs)

### Accounts and PDAs

- `report_collection`: Stores metadata about the report collection (report ID, creator, collection key, owner NFT).
- `report_data`: Stores report details (report ID, content URI, ownership status, shared organization).
- PDAs are derived using seeds like `report_collection`, `report_data`, and `share_nft` to ensure unique, program-controlled addresses.

### Decentralized Storage

- Use Pinata to upload report metadata to IPFS, ensuring decentralized and tamper-proof storage.
- Store the IPFS URI on-chain for retrieval.


### Smart Contract Program

- Written in Rust using Anchor, with instructions for `create_report`, `share_report`, and `revoke_share`.
- Uses PDAs to manage report collections (`report_collection`) and report data (`report_data` for owner and share NFTs).

### Metaplex Core Integration

- Leverages Metaplex Core for NFT creation and management.
- Creates a collection for each report, an owner NFT for the creator, and share NFTs for organizations.
- Applies a FreezeDelegate plugin to make NFTs soulbound (non-transferable).