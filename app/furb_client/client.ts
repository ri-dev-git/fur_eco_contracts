import * as anchor from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID, createMint } from "@solana/spl-token";
import fs from "fs";

// Load keypair (update path to your wallet file)
const keypair = Keypair.fromSecretKey(
    new Uint8Array(JSON.parse(fs.readFileSync("/path-to-your-wallet.json", "utf-8")))
);

const programId = new PublicKey("6n94E9pEixrcVfap2iF9GE7nuhPQ8Kf9rVhxwjYAksrp");
const network = "http://127.0.0.1:8899"; // Use mainnet if needed
const connection = new Connection(network, "confirmed");

// Initialize provider and wallet
const wallet = new anchor.Wallet(keypair);
const provider = new anchor.AnchorProvider(connection, wallet, {
    preflightCommitment: "confirmed",
});
anchor.setProvider(provider);

async function initializeMint() {
    // Load the program
    const idl = await anchor.Program.fetchIdl(programId, provider);
    const program = new anchor.Program(idl!, provider);

    console.log("Program loaded!");

    // Create a mint account
    const mint = await createMint(
        connection,               // Connection
        keypair,                  // Payer
        keypair.publicKey,        // Mint authority
        keypair.publicKey,        // Freeze authority
        6                         // Decimals
    );

    console.log(`Mint created at: ${mint.toBase58()}`);

    // Call the program's `initialize` instruction
    const tx = await program.methods
        .initialize()
        .accounts({
            signer: keypair.publicKey,
            mint,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
        })
        .rpc();

    console.log(`Transaction Signature: ${tx}`);
}

initializeMint()
    .then(() => console.log("Mint initialized successfully!"))
    .catch((err) => console.error(`Failed to initialize mint: ${err}`));
