import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Capstone } from "../target/types/capstone";
import { Keypair, PublicKey } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";

describe("capstone", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection
  const program = anchor.workspace.Capstone as Program<Capstone>;

  let creator = Keypair.generate();
  let eventId = new anchor.BN(1);
  let title = "Test Event";
  let seed = new anchor.BN(Math.floor(Math.random() * 1000));
  // let usdcMint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
  
  let eventPDA, winPoolATA;
  let eventBump;
  let usdcMint;
  
  before(async () => {
    // Fund the creator account
    const creatorAirdrop = await provider.connection.requestAirdrop(creator.publicKey, anchor.web3.LAMPORTS_PER_SOL * 10);
    
    const latestBlockhash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
      signature: creatorAirdrop,
      blockhash: latestBlockhash.blockhash,
      lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
    });

   usdcMint = await createMint(connection, creator, creator.publicKey, null, 9, Keypair.generate());

    // Find PDA for the event
    [eventPDA, eventBump] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("EVENT"),
        creator.publicKey.toBuffer(),
        eventId.toArrayLike(Buffer, "le", 8),
        seed.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );

    console.log("Derived eventPDA:", eventPDA.toBase58());

    // Get the associated token account for the win pool
    winPoolATA = getAssociatedTokenAddressSync(usdcMint, eventPDA, true);
  });

  it("should create an event successfully", async () => {
    // Call the create_event instruction
    const tx = await program.methods
      .createEvent(eventId, title, seed)
      .accountsPartial({
        creator: creator.publicKey,
        event: eventPDA, 
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        usdcMint: usdcMint,
        winPool: winPoolATA,
      })
      .remainingAccounts([
        {
          pubkey: eventPDA,
          isWritable: true,
          isSigner: false,
        },
      ])
      .signers([creator])
      .rpc();
    
    console.log("Transaction Signature:", tx);

    // Fetch the created event account
    const eventAccount = await program.account.event.fetch(eventPDA);

    // Assertions
    expect(eventAccount.eventId.toNumber()).to.equal(eventId.toNumber());
    expect(eventAccount.title).to.equal(title);
    expect(eventAccount.creator.toBase58()).to.equal(creator.publicKey.toBase58());
    expect(eventAccount.resolved).to.be.false;
    expect(eventAccount.winPool.toBase58()).to.equal(winPoolATA.toBase58());
  });
});