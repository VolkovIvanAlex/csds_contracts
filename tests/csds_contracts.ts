import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Csds } from "../target/types/csds";
import { PublicKey, Keypair } from "@solana/web3.js";
import { assert } from "chai";

describe("csds", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  console.log(provider.connection);
  const program = anchor.workspace.Csds as Program<Csds>;
  const creator = provider.wallet.publicKey;
  const orgB = Keypair.generate();
  const reportId = 1;
  const shareIndex = 1;

  const [reportCollectionPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("report_collection"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [reportPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("report_data"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [ownerNftPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("owner_nft"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [shareNftPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("share_nft"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
      new anchor.BN(shareIndex).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [collectionPda] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("collection"),
      creator.toBuffer(),
      new anchor.BN(reportId).toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  let collection = Keypair.generate();

  console.log("reportCollectionPda = ", reportCollectionPda);
  console.log("reportPda = ", reportPda);
  console.log("collectionPda = ", collectionPda);
  console.log("ownerNftPda = ", ownerNftPda);
  console.log("creator = ", creator);
  console.log(
    "anchor.web3.SystemProgram.programId = ",
    anchor.web3.SystemProgram.programId
  );
  console.log("collection = ", collection);

  it("Creates a report with collection and owner NFT", async () => {
    await program.methods
      .createReport(
        new anchor.BN(reportId),
        "https://example.com/report.json",
        "Report Collection",
        "https://example.com/collection.json"
      )
      .accounts({
        reportCollection: reportCollectionPda,
        reportData: reportPda,
        collection: collection.publicKey,
        ownerNft: ownerNftPda,
        creator: creator,
        updateAuthority: creator, // Set creator as update_authority
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: new PublicKey(
          "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        ), // Replace with your core-keypair.json public key
      })
      .signers([collection, provider.wallet.payer])
      .rpc();

    const collectionAccount = await program.account.reportCollection.fetch(
      reportCollectionPda
    );
    assert.equal(collectionAccount.reportId.toNumber(), reportId);
    assert.equal(collectionAccount.creator.toBase58(), creator.toBase58());
    assert.equal(
      collectionAccount.collectionKey.toBase58(),
      collectionPda.toBase58()
    );
    assert.equal(collectionAccount.ownerNft.toBase58(), ownerNftPda.toBase58());

    const reportData = await program.account.reportData.fetch(ownerNftPda);
    assert.equal(reportData.reportId.toNumber(), reportId);
    assert.equal(reportData.contentUri, "https://example.com/report.json");
    assert.equal(reportData.isOwnerNft, true);
    assert.equal(reportData.sharedWith, null);
  });

  it("Shares the report with another organization", async () => {
    await program.methods
      .shareReport(
        new anchor.BN(reportId),
        new anchor.BN(shareIndex),
        "https://example.com/report.json"
      )
      .accounts({
        reportCollection: reportCollectionPda,
        shareData: shareNftPda,
        collection: collectionPda,
        shareNft: shareNftPda,
        creator: creator,
        sharedOrg: orgB.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: new PublicKey(
          "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        ), // Replace with your core-keypair.json public key
      })
      .rpc();

    const shareData = await program.account.reportData.fetch(shareNftPda);
    assert.equal(shareData.reportId.toNumber(), reportId);
    assert.equal(shareData.contentUri, "https://example.com/report.json");
    assert.equal(shareData.isOwnerNft, false);
    assert.equal(shareData.sharedWith.toBase58(), orgB.publicKey.toBase58());
  });

  it("Revokes the share by burning the share NFT", async () => {
    await program.methods
      .revokeShare(new anchor.BN(reportId), new anchor.BN(shareIndex))
      .accounts({
        reportCollection: reportCollectionPda,
        shareData: shareNftPda,
        collection: collectionPda,
        shareNft: shareNftPda,
        creator: creator,
        sharedOrg: orgB.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
        mplCoreProgram: new PublicKey(
          "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        ), // Replace with your core-keypair.json public key
      })
      .rpc();

    // Verify share data account is closed
    try {
      await program.account.reportData.fetch(shareNftPda);
      assert.fail("Share data account should be closed");
    } catch (e) {
      assert.include(e.message, "Account does not exist");
    }
  });
});
