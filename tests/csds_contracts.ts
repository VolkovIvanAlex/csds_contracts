import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Csds } from "../target/types/csds";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert } from "chai";
import defaultSoulboundMetadataJson from "../metadata/report.metadata.json";
import { PinataSDK } from "pinata-web3";
import * as dotenv from "dotenv";
dotenv.config();

const pinata = new PinataSDK({
  pinataJwt: process.env.PINATA_JWT || "",
  pinataGateway: process.env.PINATA_GATEWAY || "",
});

describe("csds", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.Csds as Program<Csds>;
  const creator = provider.wallet.publicKey;

  const orgB = Keypair.generate();
  const orgX = Keypair.generate();

  const shareIndexB = 1;
  const shareIndexX = 2;

  const reportId = 1;
  const reportName = "Ransomware Report";

  const organizationName = "Csds";
  const collectionName = "Report Collection";
  const collectionUri = "https://example.com/collection.json";
  const mplTokenMetadataProgramId = new PublicKey(
    "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
  );
  const mplCoreProgramId = new PublicKey(
    "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
  );

  const [reportCollectionPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("report_collection"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [reportDataPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("report_data"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [shareNftPdaB] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("share_nft"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
      new anchor.BN(shareIndexB).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [shareNftPdaX] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("share_nft"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
      new anchor.BN(shareIndexX).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [metadataAccount] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      mplTokenMetadataProgramId.toBytes(),
      creator.toBytes(),
    ],
    mplTokenMetadataProgramId
  );

  const collection = Keypair.generate();
  const ownerNft = Keypair.generate();
  const shareNftB = Keypair.generate();
  const shareNftX = Keypair.generate();

  console.log("📌 Creator:", creator.toBase58());
  console.log("📌 OrgB PublicKey:", orgB.publicKey.toBase58());
  console.log("📌 OrgX PublicKey:", orgX.publicKey.toBase58());
  console.log("📌 Report Collection PDA:", reportCollectionPda.toBase58());
  console.log("📌 Report Data PDA:", reportDataPda.toBase58());
  console.log("📌 Share NFT PDA (OrgB):", shareNftPdaB.toBase58());
  console.log("📌 Share NFT PDA (OrgX):", shareNftPdaX.toBase58());
  console.log("📌 Collection Key:", collection.publicKey.toBase58());
  console.log("📌 Owner NFT Key:", ownerNft.publicKey.toBase58());
  console.log("📌 Share NFT Key (OrgB):", shareNftB.publicKey.toBase58());
  console.log("📌 Share NFT Key (OrgX):", shareNftX.publicKey.toBase58());

  it("✅ Creates a report with collection and owner NFT", async () => {
    const metadataUpload = await pinata.upload.json(
      defaultSoulboundMetadataJson
    );
    const uri = `https://${metadataUpload.IpfsHash}.ipfs.dweb.link/`;

    console.log("⏳ Creating report...");
    await program.methods
      .createReport(
        new anchor.BN(reportId),
        reportName,
        uri,
        collectionName,
        collectionUri,
        organizationName
      )
      .accounts({
        reportCollection: reportCollectionPda,
        reportData: reportDataPda,
        metadataAccount: metadataAccount,
        collection: collection.publicKey,
        ownerNft: ownerNft.publicKey,
        updateAuthority: creator,
        creator: creator,
        mplTokenMetadataProgram: mplTokenMetadataProgramId,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: mplCoreProgramId,
      })
      .signers([provider.wallet.payer, collection, ownerNft])
      .rpc();

    console.log("✅ Report created. Verifying on-chain data...");

    const collectionAccount = await program.account.reportCollection.fetch(
      reportCollectionPda
    );
    console.log("🧾 Collection account:", collectionAccount);

    assert.equal(collectionAccount.reportId.toNumber(), reportId);
    assert.equal(collectionAccount.creator.toBase58(), creator.toBase58());
    assert.equal(
      collectionAccount.collectionKey.toBase58(),
      collection.publicKey.toBase58()
    );
    assert.equal(
      collectionAccount.ownerNft.toBase58(),
      ownerNft.publicKey.toBase58()
    );

    const reportData = await program.account.reportData.fetch(reportDataPda);
    console.log("🧾 Report data (owner):", reportData);

    assert.equal(reportData.reportId.toNumber(), reportId);
    assert.equal(reportData.contentUri, uri);
    assert.equal(reportData.isOwnerNft, true);
    assert.equal(reportData.sharedWith, null);
  });

  it("✅ Shares the report with organization B and X", async () => {
    const metadataUpload = await pinata.upload.json(
      defaultSoulboundMetadataJson
    );
    const uri = `https://${metadataUpload.IpfsHash}.ipfs.dweb.link/`;

    console.log("⏳ Sharing report with OrgB...");
    await program.methods
      .shareReport(
        new anchor.BN(reportId),
        reportName,
        new anchor.BN(shareIndexB),
        uri
      )
      .accounts({
        reportCollection: reportCollectionPda,
        shareData: shareNftPdaB,
        collection: collection.publicKey,
        shareNft: shareNftB.publicKey,
        creator: creator,
        sharedOrg: orgB.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: mplCoreProgramId,
      })
      .signers([provider.wallet.payer, collection, shareNftB])
      .rpc();

    console.log("⏳ Sharing report with OrgX...");
    await program.methods
      .shareReport(
        new anchor.BN(reportId),
        reportName,
        new anchor.BN(shareIndexX),
        uri
      )
      .accounts({
        reportCollection: reportCollectionPda,
        shareData: shareNftPdaX,
        collection: collection.publicKey,
        shareNft: shareNftX.publicKey,
        creator: creator,
        sharedOrg: orgX.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: mplCoreProgramId,
      })
      .signers([provider.wallet.payer, collection, shareNftX])
      .rpc();

    console.log("✅ Report shared. Verifying share NFT...");

    const shareDataB = await program.account.reportData.fetch(shareNftPdaB);
    console.log("🧾 Report data (OrgB):", shareDataB);
    assert.equal(shareDataB.reportId.toNumber(), reportId);
    assert.equal(shareDataB.contentUri, uri);
    assert.equal(shareDataB.isOwnerNft, false);
    assert.equal(shareDataB.sharedWith.toBase58(), orgB.publicKey.toBase58());

    const shareDataX = await program.account.reportData.fetch(shareNftPdaX);
    console.log("🧾 Report data (OrgX):", shareDataX);
    assert.equal(shareDataX.reportId.toNumber(), reportId);
    assert.equal(shareDataX.contentUri, uri);
    assert.equal(shareDataX.isOwnerNft, false);
    assert.equal(shareDataX.sharedWith.toBase58(), orgX.publicKey.toBase58());
  });

  it("✅ Revokes the share from OrgB, ensuring only OrgX has access", async () => {
    console.log("⏳ Revoking share with OrgB...");

    await program.methods
      .revokeShare(new anchor.BN(reportId), new anchor.BN(shareIndexB))
      .accounts({
        reportCollection: reportCollectionPda,
        shareData: shareNftPdaB,
        collection: collection.publicKey,
        shareNft: shareNftB.publicKey,
        creator: creator,
        sharedOrg: orgB.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: mplCoreProgramId,
      })
      .signers([provider.wallet.payer, collection, shareNftB])
      .rpc();

    console.log("✅ Share revoked. Verifying state...");

    // Verify OrgB's share is revoked
    try {
      await program.account.reportData.fetch(shareNftPdaB);
      assert.fail("❌ OrgB share data account should be closed");
    } catch (e) {
      assert.include(e.message, "Account does not exist");
      console.log("✅ OrgB share NFT PDA successfully closed.");
    }

    // Verify OrgX's share remains
    const shareDataX = await program.account.reportData.fetch(shareNftPdaX);
    console.log("🧾 Report data (OrgX):", shareDataX);
    assert.equal(shareDataX.reportId.toNumber(), reportId);
    assert.equal(shareDataX.isOwnerNft, false);
    assert.equal(shareDataX.sharedWith.toBase58(), orgX.publicKey.toBase58());
  });
});
