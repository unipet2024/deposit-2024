import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Deposit2024 } from "../target/types/deposit_2024";

import {
  createMint,
  createAssociatedTokenAccount,
  getAssociatedTokenAddress,
  getOrCreateAssociatedTokenAccount,
  mintTo,
  transfer,
} from "@solana/spl-token";

import { assert, expect } from "chai";

import { Wallet } from "@coral-xyz/anchor";
import {
  SystemProgram,
  LAMPORTS_PER_SOL,
  Keypair,
  PublicKey,
} from "@solana/web3.js";
import { getAdminRolePda, getDepositPda, getOperatorRolePda, getPackagePda, getUserPda } from "./utils";
const CHAINLINK_PROGRAM_ID = "HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny";//devnet and mainnet chainlink program

describe("deposit-2024", () => {
  let provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.Deposit2024 as Program<Deposit2024>;

  const programId = new PublicKey(program.programId);
  const deposit_pda = getDepositPda(programId);
  
  // it("Is initialized!", async () => {
  //   console.log("Program ID: ", programId.toString());
    

  //   const adminPda = getAdminRolePda(programId, provider.wallet.publicKey);
    
  //   console.log("Deposit PDA: ", deposit_pda.toString());
  //   console.log("Admin PDA: ", adminPda.toString());
  //   const operator_wallet = new PublicKey("9kPRkHCcnhgpByJc4fyYuPU6EU68yzC5yKRQrwm2cNYS")
  //   const tx = await program.methods.init(operator_wallet).accounts({
  //     depositAccount: deposit_pda,
  //     adminAccount: adminPda,
  //     authority: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   console.log("Init tx: ", tx);
    
  //   let deposit_pda_info = await program.account.deposit.fetch(deposit_pda);
  //   console.log(JSON.stringify(deposit_pda_info));
    
  // });


  // it("add_operator", async () => {
  //   const operator =  new PublicKey("2z6bJQHscXWHNQAB8Q3YA1RiKg2QBn84Uax3FSANtvDU"); //wallet test api gateway

  //   const operator_pda = getOperatorRolePda(programId, operator);
  //   const admin_pda = getAdminRolePda(programId, provider.wallet.publicKey);

  //   console.log("Operator PDA: ", operator_pda.toString());
  //   const tx = await program.methods.addOperator(operator).accounts({
  //     depositAccount: deposit_pda,
  //     adminAccount: admin_pda,
  //     operatorAccount: operator_pda,
  //   }).rpc();
  //   console.log("Add operator tx: ", tx);

  // });

  

  // it("add_currency", async () => {  
  //   const usdc = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW")

  //   const admin_pda = getAdminRolePda(programId, provider.wallet.publicKey);
  //   const tx = await program.methods.addCurrency(usdc).accounts({
  //     depositAccount: deposit_pda,
  //     adminAccount: admin_pda,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //     admin: provider.wallet.publicKey,
  //   }).rpc();
  //   console.log("Add operator tx: ", tx);
  //   let deposit_pda_info = await program.account.deposit.fetch(deposit_pda);
  //   console.log(JSON.stringify(deposit_pda_info));
  // });

  it("deposit by sol", async () => {
    let deposit_pda_info = await program.account.deposit.fetch(deposit_pda);

  console.log("deposit ", deposit_pda.toString());
  
    
    const package_id = 1007;
    const packagePda = getPackagePda(programId, package_id);

    const packageData = await program.account.package.fetch(packagePda);

    console.log("Package data: ", JSON.stringify(packageData));
    const userPda = getUserPda(programId, provider.wallet.publicKey);
    console.log("User PDA: ", userPda.toString());
    //get userdata
    const userData = await program.account.user.fetch(userPda);
    console.log("User data: ", JSON.stringify(userData));
    console.log("Package PDA: ", packagePda.toString());


    const tx = await program.methods.userBuyPackageBySol(package_id).accounts({
      depositAccount: deposit_pda,
      chainlinkProgram: new PublicKey(CHAINLINK_PROGRAM_ID),
      chainlinkFeed: new PublicKey("99B2bTijsU6f1GCT73HmdR7HCFFjGMBcPZY6jZ96ynrR"), //keypair SOL/usd,
      packageAccount: packagePda,
      userDeposit: userPda,
      user: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    console.log("Deposit by sol tx: ", tx);

   
  })




});
