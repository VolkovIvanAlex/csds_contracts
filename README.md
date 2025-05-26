# CSDS: Cybersecurity Data Sharing on Solana

## Overview

CSDS (Cybersecurity Data Sharing) is a Solana-based smart contract program for secure creation, sharing, and revocation of cybersecurity reports using soulbound NFTs (non-transferable tokens). Built with Anchor in Rust, it integrates Metaplex Core for NFT management and Pinata for decentralized IPFS storage. Developed for a hackathon, CSDS enables organizations to share sensitive reports with specific entities while retaining fine-grained access control, ensuring transparency and immutability on Solana’s high-speed blockchain.

---

## Features
- Soulbound NFTs: Reports are represented as non-transferable NFTs, ensuring only authorized parties (owners or shared organizations) can access them.
- Decentralized Storage: Report metadata is stored on IPFS via Pinata, with URIs linked on-chain for tamper-proof retrieval.
- Fine-Grained Access Control: Creators can share reports with specific organizations and revoke access by burning share NFTs.
- Metaplex Core Integration: Manages NFT collections and tokens for reports, using FreezeDelegate to enforce soulbound properties.
- Program-Derived Addresses (PDAs): Ensures unique, program-controlled accounts for reports and shares.
- Secure Authorization: Validates creator authority for all operations, preventing unauthorized actions.

## Workflow
CSDS facilitates a three-step process for managing cybersecurity reports:

### Create a Report:
- A creator initializes a report by defining a report_id, name, organization, and IPFS content URI.
- A Metaplex Core collection is created, along with an owner soulbound NFT locked via FreezeDelegate.
- Metadata is stored in a report_data PDA.

### Share a Report:
- The creator shares the report with an organization by minting a share NFT linked to the report’s collection.
- A share_data PDA records the share, including the organization’s public key and content URI.

### Revoke Access:
- The creator revokes access by burning the share NFT and closing the associated share_data PDA.
- Funds from the closed account are returned to the creator, ensuring efficient resource management.


## Accounts and PDAs

###### ReportCollection:
```
PDA: [b"report_collection", creator.key, report_id]
Stores: report_id, creator, collection_key, owner_nft
Purpose: Tracks report metadata and links to the Metaplex collection.
```

###### ReportData:
```
PDA: [b"report_data", creator.key, report_id] (for owner) or [b"share_nft", creator.key, report_id, share_index] (for shares)
Stores: report_id, content_uri, is_owner_nft, shared_with (organization’s public key for shares)
Purpose: Holds report details and sharing status.
```

###### Metaplex Accounts:
- collection: Metaplex Core collection for the report.
- owner_nft: Soulbound NFT for the creator.
- share_nft: NFT for shared organizations (currently not soulbound).


## Smart Contract Instructions

### 1. Create Report [related instruction](programs/csds_contracts/src/instructions/soulbound/create_report.rs)

###### Actions:
- Validates report_name and organization_name lengths (max 50 chars).
- Initializes a ReportCollection PDA with report metadata.
- Creates a Metaplex Core collection with attributes (report_id, creator, organization_name).
- Mints an owner NFT with a FreezeDelegate plugin (frozen: true) to make it soulbound.
- Initializes a ReportData PDA for the owner NFT, linking to the IPFS content_uri.
###### Security:
- Ensures creator is the payer and update authority.
- Uses PDAs to prevent address collisions.

### 2. Share Report [related instruction](programs/csds_contracts/src/instructions/soulbound/share_report.rs)

###### Actions:
- Verifies the creator’s authority and report_id against the report_collection.
- Mints a share NFT linked to the report’s collection, named “Report X (Shared)”.
- Initializes a share_data PDA with the organization’s public key and content_uri.
###### Security:
- Restricts sharing to the report’s creator.
- Uses share_index for unique share PDAs.

### 3. Revoke Share [related instruction](programs/csds_contracts/src/instructions/soulbound/revoke_share.rs)

###### Actions:
- Verifies the creator, report_id, and that the NFT is a share (not owner).
- Ensures the shared_with organization matches the provided public key.
- Burns the share NFT and closes the share_data PDA, returning lamports to the creator.
###### Security:
- Robust checks prevent burning owner NFTs or unauthorized revocations.
