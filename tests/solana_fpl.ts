import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert } from "chai";
import { SolanaFpl } from "../target/types/solana_fpl";
import {
  createMint,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import BN from "bn.js";
import { getKeypairFromFile } from "@solana-developers/helpers";
import { PublicKey, Keypair, Connection } from "@solana/web3.js";

describe("solana_fpl", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SolanaFpl as Program<SolanaFpl>;

  let owner: anchor.web3.Keypair;
  let escrowAccount: anchor.web3.PublicKey;
  let usdcMint: anchor.web3.PublicKey;

  const totalPotForWinners = new anchor.BN(10000000);
  const betAmount = new anchor.BN(1000000); 

  beforeEach(async () => {
      owner = await getKeypairFromFile("/home/ritikbhatt020/multi-token-escrow/keys/admin-CAT5qnvWfU9LQyprcLrXDMMifR6tL95nCrsNk8Mx12C7.json");
      console.log("hi")
      usdcMint = await createMint(
        provider.connection,
        owner,
        owner.publicKey,
        null,
        6 
      );
      console.log("USDC Mint:", usdcMint);
    });

  it("Initializes the escrow", async () => {
    [escrowAccount] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow")],
      program.programId
    );
    console.log("Escrow public key:", escrowAccount)
    console.log("Owner public key:", owner.publicKey)
   
    const tx = await program.methods
      .initializeEscrow(usdcMint, totalPotForWinners, betAmount)
      .accounts({
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();

    console.log("Transaction:", tx)

    const escrowAccountState = await program.account.escrowAccount.fetch(
      escrowAccount
    );

    assert.equal(
      escrowAccountState.authority.toBase58(),
      owner.publicKey.toBase58(),
      "Escrow authority should be set to the owner"
    );
    assert.equal(
      escrowAccountState.usdcMint.toBase58(),
      usdcMint.toBase58(),
      "USDC Mint should match"
    );
    assert.equal(
      escrowAccountState.totalPotForWinners.toString(),
      totalPotForWinners.toString(),
      "Total pot for winners should be set correctly"
    );
    assert.equal(
      escrowAccountState.betAmount.toString(),
      betAmount.toString(),
      "Bet amount should be set correctly"
    );
    assert.equal(
      escrowAccountState.usdcBalance.toString(),
      "0",
      "Initial USDC balance should be 0"
    );
  });
});
