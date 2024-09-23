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
  let user: anchor.web3.Keypair;
  let escrowAccount: anchor.web3.PublicKey;
  let userAccount: anchor.web3.PublicKey;
  let usdcMint: anchor.web3.PublicKey;
  let userTokenAccount: PublicKey;
  let escrowTokenAccount: PublicKey;

  const totalPotForWinners = new anchor.BN(10000000);
  const betAmount = new anchor.BN(1000000);

  beforeEach(async () => {

    const connection = new Connection("http://127.0.0.1:8899", "confirmed");

    owner = await getKeypairFromFile(
      "/home/ritikbhatt020/multi-token-escrow/keys/admin-CAT5qnvWfU9LQyprcLrXDMMifR6tL95nCrsNk8Mx12C7.json"
    );
    user = await getKeypairFromFile("./tests/userKeypair.json");
    console.log("user:", user.publicKey)
    console.log("hi");
    usdcMint = new PublicKey("DAG4KRYrzFuuV3TBqZnvPfW4ZJirMoBa7MjBfKmMc4kN");

    [escrowAccount] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("escrow")],
      program.programId
    );
    console.log("Escrow Account PDA:", escrowAccount);

    [userAccount] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("user"), user.publicKey.toBuffer()],
      program.programId
    );
    console.log("User Account PDA:", userAccount);

    userTokenAccount = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        user,
        usdcMint,
        user.publicKey,
        true
      )
    ).address;
    console.log("userTokenAccount:", userTokenAccount);

    escrowTokenAccount = (
      await getOrCreateAssociatedTokenAccount(
        connection,
        owner,
        usdcMint,
        escrowAccount,
        true
      )
    ).address;
    console.log("escrowTokenAccount:", escrowTokenAccount);
  });

  it("Initializes the escrow", async () => {
    // [escrowAccount] = await anchor.web3.PublicKey.findProgramAddressSync(
    //   [Buffer.from("escrow")],
    //   program.programId
    // );
    console.log("Escrow public key:", escrowAccount);
    console.log("Owner public key:", owner.publicKey);

    const tx = await program.methods
      .initializeEscrow(usdcMint, totalPotForWinners, betAmount)
      .accounts({
        owner: owner.publicKey,
      })
      .signers([owner])
      .rpc();

    console.log("Transaction:", tx);

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

  it("Places a bet", async () => {
    console.log("user:", user.publicKey);
    console.log("userTokenAccount:", userTokenAccount);
    console.log("escrowAccountAccount:", escrowTokenAccount)
    const tx = await program.methods
      .bet()
      .accounts({
        user: user.publicKey,
        userTokenAccount: userTokenAccount,
        escrowTokenAccount: escrowTokenAccount,
      })
      .signers([user])
      .rpc();

    console.log("tx:", tx);

    const userAccountState = await program.account.userAccount.fetch(
      userAccount
    );
    console.log("userAccountPda:", userAccountState)
    console.log("userAccountPda:", userAccountState.isEligible)
    console.log("userAccountPda:", userAccountState.payoutAmount)

    assert.equal(
      userAccountState.owner.toBase58(),
      user.publicKey.toBase58(),
      "User account owner should match"
    );
    assert.isFalse(
      userAccountState.isEligible,
      "User should not be eligible after placing the bet"
    );

    const escrowAccountState = await program.account.escrowAccount.fetch(
      escrowAccount
    );

    assert.equal(
      escrowAccountState.usdcBalance.toString(),
      betAmount.toString(),
      "Escrow USDC balance should match bet amount"
    );
  });
});
