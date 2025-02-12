import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../wba-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const tokenDecimals = 9; // Your token decimals
const mintAmount = 1_000_000_000n * (10n ** BigInt(tokenDecimals)); // 1 billion tokens

// Mint address
const mint = new PublicKey("8iMpYJ5LhT3DPnGHU3pvpPkyY5gN2mWmVrF67KesjFUY");

(async () => {
    try {
        // Create an ATA
        const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
        console.log(`Your ata is: ${ata.address.toBase58()}`);

        // Mint to ATA
        const mintTx = await mintTo(connection, keypair, mint, ata.address, keypair.publicKey,mintAmount)
        console.log(`Your mint txid: ${mintTx}`);
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
































//to burn the tokens

// import { Keypair, PublicKey, Connection, Commitment } from "@solana/web3.js";
// import { getOrCreateAssociatedTokenAccount, burn } from '@solana/spl-token';
// import wallet from "../wba-wallet.json";

// // Import keypair
// const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// // Create a Solana devnet connection
// const commitment: Commitment = "confirmed";
// const connection = new Connection("https://api.devnet.solana.com", commitment);

// // Token Mint Address (Your existing token with 6 decimals)
// const mint = new PublicKey("ATJmZnjE2DRfccEHjyfjdRQGsjpVB4roqbxKdYVc6Z2a");

// // Amount to burn (Make sure it matches your token decimals)
// const tokenDecimals = 6;
// const burnAmount = BigInt(999_999_821_783_259);

// (async () => {
//     try {
//         // Get or create an associated token account (ATA)
//         const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);
//         console.log(`Your ATA is: ${ata.address.toBase58()}`);

//         // Burn tokens
//         const burnTx = await burn(
//             connection,
//             keypair,
//             ata.address,  // Token account holding the tokens
//             mint,         // Token mint address
//             keypair.publicKey, // Owner of the account
//             burnAmount
//         );
//         console.log(`🔥 Tokens burned successfully! TXID: ${burnTx}`);
//     } catch (error) {
//         console.log(`Oops, something went wrong: ${error}`);
//     }
// })();
