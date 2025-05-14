use anchor_lang::prelude::*;

#[error_code]
pub enum MintSoulboundNftErrorCode {
    #[msg("Failed to create asset during minting soulbound NFT")]
    AssetCreationError,
    #[msg("Failed to update asset metadata during minting soulbound NFT")]
    UpdateAssetMetadataError,
    #[msg("Unknown error has occured during minting soulbound NFT")]
    UnknownError,
    #[msg("Unauthorized")]
    Unauthorized,
}

#[error_code]
pub enum CSDSError {
    #[msg("Unauthorized: Only the creator can perform this action")]
    Unauthorized,
    #[msg("Invalid report ID")]
    InvalidReportId,
    #[msg("NFT is not a share NFT")]
    NotShareNFT,
    #[msg("Share NFT not found for organization")]
    ShareNFTNotFound,
    #[msg("Arithmetic overflow occurred")]
    Overflow,
    #[msg("Organization name exceeds maximum length")]
    OrgNameTooLong,
    #[msg("Report name exceeds maximum length")]
    ReportNameTooLong,
}
