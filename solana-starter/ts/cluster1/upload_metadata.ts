import wallet from "../wba-wallet.json"; 
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";

// Create a connection to Solana Devnet
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader()); // Use Arweave Bundlr (Irys)
umi.use(signerIdentity(signer));

(async () => {
    try {
        // 1️⃣ Load metadata JSON file
        const metadata = await readFile("metadata.json");

        // 2️⃣ Convert JSON to a generic file
        const genericFile = createGenericFile(metadata, "metadata.json", {
            contentType: "application/json",
        });

        // 3️⃣ Upload metadata JSON to Arweave
        const [jsonUri] = await umi.uploader.upload([genericFile]);

        console.log("✅ Your metadata JSON URI: ", jsonUri);
    }
    catch (error) {
        console.log("❌ Oops.. Something went wrong", error);
    }
})();
