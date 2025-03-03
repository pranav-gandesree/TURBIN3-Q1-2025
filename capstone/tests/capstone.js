// import * as anchor from "@coral-xyz/anchor";
// import { Program } from "@coral-xyz/anchor";
// import { Capstone } from "../target/types/capstone.js";
// import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
// import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID } from "@solana/spl-token";
// import { assert, expect } from "chai";
// describe("capstone", () => {
//   const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);
//   const connection = provider.connection
//   const program = anchor.workspace.Capstone as Program<Capstone>;
//   let creator = Keypair.generate();
//   let eventId = new anchor.BN(1);
//   let title = "Test Event";
//   let seed = new anchor.BN(Math.floor(Math.random() * 1000));
//   // let usdcMint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
//   let eventPDA, winPoolATA;
//   let eventBump;
//   let usdcMint;
//   before(async () => {
//     // Fund the creator account
//     const creatorAirdrop = await provider.connection.requestAirdrop(creator.publicKey, anchor.web3.LAMPORTS_PER_SOL * 10);
//     const latestBlockhash = await provider.connection.getLatestBlockhash();
//     await provider.connection.confirmTransaction({
//       signature: creatorAirdrop,
//       blockhash: latestBlockhash.blockhash,
//       lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
//     });
//    usdcMint = await createMint(connection, creator, creator.publicKey, null, 9, Keypair.generate());
//     // Find PDA for the event
//     [eventPDA, eventBump] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("EVENT"),
//         creator.publicKey.toBuffer(),
//         eventId.toArrayLike(Buffer, "le", 8),
//         seed.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//     console.log("Derived eventPDA:", eventPDA.toBase58());
//     // Get the associated token account for the win pool
//     winPoolATA = getAssociatedTokenAddressSync(usdcMint, eventPDA, true);
//   });
//   it("should create an event successfully", async () => {
//     // Call the create_event instruction
//     const tx = await program.methods
//       .createEvent(eventId, title, seed)
//       .accountsPartial({
//         creator: creator.publicKey,
//         event: eventPDA, 
//         systemProgram: anchor.web3.SystemProgram.programId,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
//         rent: anchor.web3.SYSVAR_RENT_PUBKEY,
//         usdcMint: usdcMint,
//         winPool: winPoolATA,
//       })
//       .remainingAccounts([
//         {
//           pubkey: eventPDA,
//           isWritable: true,
//           isSigner: false,
//         },
//       ])
//       .signers([creator])
//       .rpc();
//     console.log("Transaction Signature:", tx);
//     // Fetch the created event account
//     const eventAccount = await program.account.event.fetch(eventPDA);
//     // Assertions
//     expect(eventAccount.eventId.toNumber()).to.equal(eventId.toNumber());
//     expect(eventAccount.title).to.equal(title);
//     expect(eventAccount.creator.toBase58()).to.equal(creator.publicKey.toBase58());
//     expect(eventAccount.resolved).to.be.false;
//     expect(eventAccount.winPool.toBase58()).to.equal(winPoolATA.toBase58());
//   });
//   let outcomeYes, outcomeNo;
//   let outcomeYesSeed = new anchor.BN(Math.floor(Math.random() * 1000));
//   let outcomeNoSeed = new anchor.BN(Math.floor(Math.random() * 1000));
//   let outcomeYesId = new anchor.BN(1);
//   let outcomeNoId = new anchor.BN(2);
//   it("should initialize outcomes successfully", async () => {
//     // Derive PDAs for outcome accounts
//     [outcomeYes] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("OUTCOME"),
//         eventPDA.toBuffer(),
//         outcomeYesId.toArrayLike(Buffer, "le", 8),
//         outcomeYesSeed.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//     [outcomeNo] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("OUTCOME"),
//         eventPDA.toBuffer(),
//         outcomeNoId.toArrayLike(Buffer, "le", 8),
//         outcomeNoSeed.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//     console.log("Derived Outcome Yes PDA:", outcomeYes.toBase58());
//     console.log("Derived Outcome No PDA:", outcomeNo.toBase58());
//     // Call initializeOutcomes
//     const tx = await program.methods
//       .initializeOutcomes(outcomeYesId, outcomeNoId, outcomeYesSeed, outcomeNoSeed)
//       .accountsPartial({
//         creator: creator.publicKey,
//         event: eventPDA,
//         outcomeYes,
//         outcomeNo,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();
//     console.log("Transaction Signature:", tx);
//     // Fetch and verify Outcome accounts
//     const outcomeYesAccount = await program.account.outcome.fetch(outcomeYes);
//     const outcomeNoAccount = await program.account.outcome.fetch(outcomeNo);
//     expect(outcomeYesAccount.outcomeId.toNumber()).to.equal(outcomeYesId.toNumber());
//     expect(outcomeNoAccount.outcomeId.toNumber()).to.equal(outcomeNoId.toNumber());
//     expect(outcomeYesAccount.eventId.toBase58()).to.equal(eventPDA.toBase58());
//     expect(outcomeNoAccount.eventId.toBase58()).to.equal(eventPDA.toBase58());
//     expect(outcomeYesAccount.shares.toNumber()).to.equal(0);
//     expect(outcomeNoAccount.shares.toNumber()).to.equal(0);
//     expect(outcomeYesAccount.resolved).to.be.false;
//     expect(outcomeNoAccount.resolved).to.be.false;
//   });
//   let userPDA;
//   let userSeed = new anchor.BN(Math.floor(Math.random() * 1000));
//   it("should initialize user successfully", async () => {
//     // Derive the User PDA
//     [userPDA] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("user"),
//         creator.publicKey.toBuffer(),
//         userSeed.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//     console.log("Derived User PDA:", userPDA.toBase58());
//     // Call initializeUser
//     const tx = await program.methods
//       .initializeUser(userSeed)
//       .accountsPartial({
//         authority: creator.publicKey,
//         user: userPDA,
//         systemProgram: anchor.web3.SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();
//     console.log("Transaction Signature:", tx);
//     // Fetch and verify the User account
//     const userAccount = await program.account.user.fetch(userPDA);
//     expect(userAccount.user.toBase58()).to.equal(creator.publicKey.toBase58());
//     expect(userAccount.totalBets.toNumber()).to.equal(0);
//     expect(userAccount.seed.toNumber()).to.equal(userSeed.toNumber());
//   });
//   let betPDA, userBetPDA;
//   let betSeed = new anchor.BN(Math.floor(Math.random() * 1000));
//   let outcomeYesPDA, outcomeNoPDA;
//   let betAmount = new anchor.BN(100_000_000); // 100 USDC (assuming 6 decimals)
//   it("should place a bet successfully", async () => {
//     // Derive Outcome PDAs
//     [outcomeYesPDA] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("OUTCOME"),
//         eventPDA.toBuffer(),
//         new anchor.BN(1).toArrayLike(Buffer, "le", 8),
//         seed.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//     [outcomeNoPDA] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("OUTCOME"),
//         eventPDA.toBuffer(),
//         new anchor.BN(0).toArrayLike(Buffer, "le", 8),
//         seed.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//     // Derive Bet PDA
//     [betPDA] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("bet"),
//         creator.publicKey.toBuffer(),
//         eventPDA.toBuffer(),
//         betSeed.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//     // Derive UserBet PDA
//     [userBetPDA] = PublicKey.findProgramAddressSync(
//       [
//         Buffer.from("user_bet"),
//         creator.publicKey.toBuffer(),
//         betPDA.toBuffer(),
//         betSeed.toArrayLike(Buffer, "le", 8),
//       ],
//       program.programId
//     );
//     console.log("Derived Outcome Yes PDA:", outcomeYesPDA.toBase58());
//     console.log("Derived Outcome No PDA:", outcomeNoPDA.toBase58());
//     console.log("Derived Bet PDA:", betPDA.toBase58());
//     console.log("Derived User Bet PDA:", userBetPDA.toBase58());
//     // Call placeBet
//     const tx = await program.methods
//       .placeBet(betAmount, 1, betSeed)
//       .accountsPartial({
//         better: creator.publicKey,
//         event: eventPDA,
//         outcomeNo: outcomeNoPDA,
//         outcomeYes: outcomeYesPDA,
//         bet: betPDA,
//         user: userPDA,
//         userBet: userBetPDA,
//         betterTokenAccount: winPoolATA, // Assuming creator is funding the bet from the win pool
//         winPool: winPoolATA,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();
//     console.log("Transaction Signature:", tx);
//     // Fetch and verify Bet account
//     const betAccount = await program.account.bet.fetch(betPDA);
//     expect(betAccount.better.toBase58()).to.equal(creator.publicKey.toBase58());
//     expect(betAccount.event.toBase58()).to.equal(eventPDA.toBase58());
//     expect(betAccount.betAmount.toNumber()).to.equal(betAmount.toNumber());
//     expect(betAccount.outcomeIndex).to.equal(1);
//     // Fetch and verify UserBet account
//     const userBetAccount = await program.account.userBet.fetch(userBetPDA);
//     expect(userBetAccount.user.toBase58()).to.equal(creator.publicKey.toBase58());
//     expect(userBetAccount.bet.toBase58()).to.equal(betPDA.toBase58());
//     // Fetch and verify updated User account
//     const userAccount = await program.account.user.fetch(userPDA);
//     expect(userAccount.totalBets.toNumber()).to.equal(1);
//     // Fetch and verify updated Outcome account
//     const outcomeYesAccount = await program.account.outcome.fetch(outcomeYesPDA);
//     expect(outcomeYesAccount.shares.toNumber()).to.be.greaterThan(0);
//     expect(outcomeYesAccount.totalLiquidity.toNumber()).to.be.greaterThan(0);
//   });
//   let resolveTx;
//   let eventResolved;
//   it("should resolve an event", async () => {
//     // Ensure the event was created
//     eventResolved = await program.account.event.fetch(eventPDA);
//     expect(eventResolved.resolved).to.be.false;
//     // Call resolveEvent with outcome = 1 (Yes wins)
//     resolveTx = await program.methods
//       .resolveEvent(1)
//       .accountsPartial({
//         authority: creator.publicKey,
//         event: eventPDA,
//         outcomeYes: outcomeYesPDA,
//         outcomeNo: outcomeNoPDA,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();
//     console.log("Resolve Event Transaction:", resolveTx);
//     // Fetch updated event account
//     eventResolved = await program.account.event.fetch(eventPDA);
//     expect(eventResolved.resolved).to.be.true;
//     expect(eventResolved.winningOutcome).to.equal(1);
//     // Fetch updated outcome accounts
//     outcomeYes = await program.account.outcome.fetch(outcomeYesPDA);
//     outcomeNo = await program.account.outcome.fetch(outcomeNoPDA);
//     expect(outcomeYes.resolved).to.be.true;
//     expect(outcomeNo.resolved).to.be.false;
//   });
//   let claimTx;
//   let betPlaced, betterBalanceBefore, betterBalanceAfter;
//   it("should claim reward for a winning bet", async () => {
//     // Fetch event and bet data before claim
//     eventResolved = await program.account.event.fetch(eventPDA);
//     betPlaced = await program.account.bet.fetch(betPDA);
//     expect(eventResolved.resolved).to.be.true;
//     expect(betPlaced.claimed).to.be.false;
//     // Fetch user's token balance before claim
//     betterBalanceBefore = (await program.provider.connection.getTokenAccountBalance(winPoolATA)).value.amount;
//     // Call claimReward
//     claimTx = await program.methods
//       .claimReward()
//       .accountsPartial({
//         event: eventPDA,
//         bet: betPDA,
//         better: creator.publicKey,
//         betterTokenAccount: winPoolATA,
//         winPool: winPoolATA,
//         outcome: outcomeYesPDA,
//         tokenProgram: TOKEN_PROGRAM_ID,
//         systemProgram: SystemProgram.programId,
//       })
//       .signers([creator])
//       .rpc();
//     console.log("Claim Reward Transaction:", claimTx);
//     // Fetch updated bet account
//     betPlaced = await program.account.bet.fetch(betPDA);
//     expect(betPlaced.claimed).to.be.true;
//     // Fetch user's token balance after claim
//     betterBalanceAfter = (await program.provider.connection.getTokenAccountBalance(winPoolATA)).value.amount;
//     // Ensure tokens were received
//     expect(parseInt(betterBalanceAfter)).to.be.greaterThan(parseInt(betterBalanceBefore));
//   });
// });
import * as anchor from "@coral-xyz/anchor";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { ASSOCIATED_TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync, createAssociatedTokenAccountInstruction, createMintToInstruction, TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
describe("capstone", () => {
    const provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const connection = provider.connection;
    const program = anchor.workspace.Capstone;
    let creator = Keypair.generate();
    let eventId = new anchor.BN(1);
    let title = "Test Event";
    let seed = new anchor.BN(Math.floor(Math.random() * 1000));
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
        [eventPDA, eventBump] = PublicKey.findProgramAddressSync([
            Buffer.from("EVENT"),
            creator.publicKey.toBuffer(),
            eventId.toArrayLike(Buffer, "le", 8),
            seed.toArrayLike(Buffer, "le", 8),
        ], program.programId);
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
    let outcomeYes, outcomeNo;
    // Use the same seed for outcomes to match what your program expects
    let outcomeSeed = seed;
    let outcomeYesId = new anchor.BN(1);
    let outcomeNoId = new anchor.BN(0);
    it("should initialize outcomes successfully", async () => {
        // Derive PDAs for outcome accounts using the SAME seed as the event
        [outcomeYes] = PublicKey.findProgramAddressSync([
            Buffer.from("OUTCOME"),
            eventPDA.toBuffer(),
            outcomeYesId.toArrayLike(Buffer, "le", 8),
            seed.toArrayLike(Buffer, "le", 8),
        ], program.programId);
        [outcomeNo] = PublicKey.findProgramAddressSync([
            Buffer.from("OUTCOME"),
            eventPDA.toBuffer(),
            outcomeNoId.toArrayLike(Buffer, "le", 8),
            seed.toArrayLike(Buffer, "le", 8),
        ], program.programId);
        console.log("Derived Outcome Yes PDA:", outcomeYes.toBase58());
        console.log("Derived Outcome No PDA:", outcomeNo.toBase58());
        // Call initializeOutcomes
        const tx = await program.methods
            .initializeOutcomes(outcomeYesId, outcomeNoId, seed, seed)
            .accountsPartial({
            creator: creator.publicKey,
            event: eventPDA,
            outcomeYes,
            outcomeNo,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
            .signers([creator])
            .rpc();
        console.log("Transaction Signature:", tx);
        // Fetch and verify Outcome accounts
        const outcomeYesAccount = await program.account.outcome.fetch(outcomeYes);
        const outcomeNoAccount = await program.account.outcome.fetch(outcomeNo);
        expect(outcomeYesAccount.outcomeId.toNumber()).to.equal(outcomeYesId.toNumber());
        expect(outcomeNoAccount.outcomeId.toNumber()).to.equal(outcomeNoId.toNumber());
        expect(outcomeYesAccount.eventId.toBase58()).to.equal(eventPDA.toBase58());
        expect(outcomeNoAccount.eventId.toBase58()).to.equal(eventPDA.toBase58());
        expect(outcomeYesAccount.shares.toNumber()).to.equal(0);
        expect(outcomeNoAccount.shares.toNumber()).to.equal(0);
        expect(outcomeYesAccount.resolved).to.be.false;
        expect(outcomeNoAccount.resolved).to.be.false;
    });
    let userPDA;
    let userSeed = new anchor.BN(Math.floor(Math.random() * 1000));
    it("should initialize user successfully", async () => {
        // Derive the User PDA
        [userPDA] = PublicKey.findProgramAddressSync([
            Buffer.from("user"),
            creator.publicKey.toBuffer(),
            userSeed.toArrayLike(Buffer, "le", 8),
        ], program.programId);
        console.log("Derived User PDA:", userPDA.toBase58());
        // Call initializeUser
        const tx = await program.methods
            .initializeUser(userSeed)
            .accountsPartial({
            authority: creator.publicKey,
            user: userPDA,
            systemProgram: anchor.web3.SystemProgram.programId,
        })
            .signers([creator])
            .rpc();
        console.log("Transaction Signature:", tx);
        // Fetch and verify the User account
        const userAccount = await program.account.user.fetch(userPDA);
        expect(userAccount.user.toBase58()).to.equal(creator.publicKey.toBase58());
        expect(userAccount.totalBets.toNumber()).to.equal(0);
        expect(userAccount.seed.toNumber()).to.equal(userSeed.toNumber());
    });
    let betPDA, userBetPDA;
    let betSeed = new anchor.BN(Math.floor(Math.random() * 1000));
    let betAmount = new anchor.BN(100000000); // 100 tokens with 9 decimals
    let userTokenAccount;
    it("should place a bet successfully", async () => {
        // Create and fund a token account for the user
        userTokenAccount = getAssociatedTokenAddressSync(usdcMint, creator.publicKey, false);
        // Create token account and mint tokens in a single transaction
        const tx = new anchor.web3.Transaction();
        // Check if the account exists first
        const accountInfo = await connection.getAccountInfo(userTokenAccount);
        if (!accountInfo) {
            // Create ATA if it doesn't exist
            tx.add(createAssociatedTokenAccountInstruction(creator.publicKey, userTokenAccount, creator.publicKey, usdcMint));
        }
        // Mint tokens to the user
        tx.add(createMintToInstruction(usdcMint, userTokenAccount, creator.publicKey, 1000000000 // 1000 tokens with 9 decimals
        ));
        await provider.sendAndConfirm(tx, [creator]);
        console.log("User Token Account:", userTokenAccount.toBase58());
        console.log("User Token Balance:", (await connection.getTokenAccountBalance(userTokenAccount)).value.amount);
        // Derive Bet PDA
        [betPDA] = PublicKey.findProgramAddressSync([
            Buffer.from("bet"),
            creator.publicKey.toBuffer(),
            eventPDA.toBuffer(),
            betSeed.toArrayLike(Buffer, "le", 8),
        ], program.programId);
        // Derive UserBet PDA
        [userBetPDA] = PublicKey.findProgramAddressSync([
            Buffer.from("user_bet"),
            creator.publicKey.toBuffer(),
            betPDA.toBuffer(),
            betSeed.toArrayLike(Buffer, "le", 8),
        ], program.programId);
        console.log("Derived Bet PDA:", betPDA.toBase58());
        console.log("Derived User Bet PDA:", userBetPDA.toBase58());
        // Call placeBet
        const betTx = await program.methods
            .placeBet(betAmount, 1, betSeed) // Betting on outcome 1 (Yes)
            .accountsPartial({
            better: creator.publicKey,
            event: eventPDA,
            outcomeYes: outcomeYes,
            outcomeNo: outcomeNo,
            bet: betPDA,
            user: userPDA,
            userBet: userBetPDA,
            betterTokenAccount: userTokenAccount,
            winPool: winPoolATA,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
            .signers([creator])
            .rpc();
        console.log("Place Bet Transaction:", betTx);
        // Fetch and verify Bet account
        const betAccount = await program.account.bet.fetch(betPDA);
        expect(betAccount.better.toBase58()).to.equal(creator.publicKey.toBase58());
        expect(betAccount.event.toBase58()).to.equal(eventPDA.toBase58());
        expect(betAccount.betAmount.toNumber()).to.equal(betAmount.toNumber());
        expect(betAccount.outcomeIndex).to.equal(1);
        // Fetch and verify UserBet account
        const userBetAccount = await program.account.userBet.fetch(userBetPDA);
        expect(userBetAccount.user.toBase58()).to.equal(creator.publicKey.toBase58());
        expect(userBetAccount.bet.toBase58()).to.equal(betPDA.toBase58());
        // Fetch and verify updated User account
        const userAccount = await program.account.user.fetch(userPDA);
        expect(userAccount.totalBets.toNumber()).to.equal(1);
        // Fetch and verify updated Outcome account
        const outcomeYesAccount = await program.account.outcome.fetch(outcomeYes);
        expect(outcomeYesAccount.shares.toNumber()).to.be.greaterThan(0);
    });
    let resolveTx;
    let eventResolved;
    it("should resolve an event", async () => {
        // Ensure the event was created
        eventResolved = await program.account.event.fetch(eventPDA);
        expect(eventResolved.resolved).to.be.false;
        // Call resolveEvent with outcome = 1 (Yes wins)
        resolveTx = await program.methods
            .resolveEvent(1)
            .accountsPartial({
            authority: creator.publicKey,
            event: eventPDA,
            outcomeYes: outcomeYes,
            outcomeNo: outcomeNo,
            systemProgram: SystemProgram.programId,
        })
            .signers([creator])
            .rpc();
        console.log("Resolve Event Transaction:", resolveTx);
        // Fetch updated event account
        eventResolved = await program.account.event.fetch(eventPDA);
        expect(eventResolved.resolved).to.be.true;
        expect(eventResolved.winningOutcome).to.equal(1);
        // Fetch updated outcome accounts
        const outcomeYesAccount = await program.account.outcome.fetch(outcomeYes);
        const outcomeNoAccount = await program.account.outcome.fetch(outcomeNo);
        expect(outcomeYesAccount.resolved).to.be.true;
        expect(outcomeNoAccount.resolved).to.be.false;
    });
    let claimTx;
    let betPlaced, betterBalanceBefore, betterBalanceAfter;
    it("should claim reward for a winning bet", async () => {
        // Fetch event and bet data before claim
        eventResolved = await program.account.event.fetch(eventPDA);
        betPlaced = await program.account.bet.fetch(betPDA);
        expect(eventResolved.resolved).to.be.true;
        expect(betPlaced.claimed).to.be.false;
        // Fetch user's token balance before claim
        betterBalanceBefore = (await connection.getTokenAccountBalance(userTokenAccount)).value.amount;
        console.log("Balance before claim:", betterBalanceBefore);
        // Call claimReward
        claimTx = await program.methods
            .claimReward()
            .accountsPartial({
            event: eventPDA,
            bet: betPDA,
            better: creator.publicKey,
            betterTokenAccount: userTokenAccount,
            winPool: winPoolATA,
            outcome: outcomeYes,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
            .signers([creator])
            .rpc();
        console.log("Claim Reward Transaction:", claimTx);
        // Fetch updated bet account
        betPlaced = await program.account.bet.fetch(betPDA);
        expect(betPlaced.claimed).to.be.true;
        // Fetch user's token balance after claim
        betterBalanceAfter = (await connection.getTokenAccountBalance(userTokenAccount)).value.amount;
        console.log("Balance after claim:", betterBalanceAfter);
        // Ensure tokens were received (balance should increase)
        expect(parseInt(betterBalanceAfter)).to.be.greaterThan(parseInt(betterBalanceBefore));
    });
});
