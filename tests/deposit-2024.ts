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
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";

import { assert, expect } from "chai";

import { Wallet } from "@coral-xyz/anchor";
import {
  SystemProgram,
  LAMPORTS_PER_SOL,
  Keypair,
  PublicKey,
} from "@solana/web3.js";
import { getAdminRolePda, getDepositPda, getOperatorPda, getOperatorRolePda, getPackagePda, getUserPda } from "./utils";
import { it } from "mocha";
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


  // it("setup_operator", async () => {
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

  // it("set_ new admin", async () => {  
  //   const new_admin = new PublicKey("2z6bJQHscXWHNQAB8Q3YA1RiKg2QBn84Uax3FSANtvDU")
  //     // const usdc = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW")
  
  //     const admin_pda = getAdminRolePda(programId, provider.wallet.publicKey);
  //     const tx = await program.methods.setAdminAuthority(new_admin).accounts({
  //       depositAccount: deposit_pda,
  //       newAdminAccount: getAdminRolePda(programId,new_admin),
  //       adminAccount: admin_pda,
  //       authority: provider.wallet.publicKey,
  //       systemProgram: anchor.web3.SystemProgram.programId,
       
  //     }).rpc();
  //     console.log("set admin tx: ", tx);
  //     let deposit_pda_info = await program.account.deposit.fetch(deposit_pda);
  //     console.log(JSON.stringify(deposit_pda_info));
  //   });

  // it("deposit by sol", async () => {
  //   let deposit_pda_info = await program.account.deposit.fetch(deposit_pda);

  //   console.log("deposit ",JSON.stringify(deposit_pda_info));
  
    
  //   const package_id = 1001;
  //   const packagePda = getPackagePda(programId, package_id);

  //   const packageData = await program.account.package.fetch(packagePda);


  //   console.log("wallet: ",  provider.wallet.publicKey.toString());
  //   console.log("Package data: ", JSON.stringify(packageData));
  //   const userPda = getUserPda(programId, provider.wallet.publicKey);
  //   console.log("User PDA: ", userPda.toString());
  //   //get userdata
  //   const userData = await program.account.user.fetch(userPda);
  //   console.log("User data: ", JSON.stringify(userData));
  //   console.log("Package PDA: ", packagePda.toString());


  //   const tx = await program.methods.userBuyPackageBySol(package_id).accounts({
  //     depositAccount: deposit_pda,
  //     chainlinkProgram: new PublicKey(CHAINLINK_PROGRAM_ID),
  //     chainlinkFeed: new PublicKey("99B2bTijsU6f1GCT73HmdR7HCFFjGMBcPZY6jZ96ynrR"), //keypair SOL/usd,
  //     packageAccount: packagePda,
  //     userDeposit: userPda,
  //     user: provider.wallet.publicKey,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   console.log("Deposit by sol tx: ", tx);

   
  // })

  it("update package", async () => {
    const package_id = 1002;
    const packagePda = getPackagePda(programId, package_id);
    const packageData = await program.account.package.fetch(packagePda);
    console.log("Package data: ", JSON.stringify(packageData));
    const tx = await program.methods.updatePackage(package_id, 4.99 * 10**4).accounts({
      packageAccount: packagePda,
      operatorAccount : getAdminRolePda(programId, provider.wallet.publicKey),
      operator: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Update package tx: ", tx);
  })

  // it("deposit by udc", async () => {

  //   let deposit_pda_info = await program.account.deposit.fetch(deposit_pda);

  //   console.log("deposit ",JSON.stringify(deposit_pda_info));
  //   const package_id = 1002;
  //   const packagePda = getPackagePda(programId, package_id);
  //   const packageData = await program.account.package.fetch(packagePda);
  //   console.log("Package data: ", JSON.stringify(packageData));
  //   const userPda = getUserPda(programId, provider.wallet.publicKey);
  //   console.log("User PDA: ", userPda.toString());
  //   const token_mint  = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW");
  //   console.log("Token mint: ", token_mint.toString());
    
  //   // console.log("user_ata:", getAssociatedTokenAddressSync(token_mint, provider.wallet.publicKey, true));
    

   

  //   const tx = await program.methods.userBuyPackageBySpl(package_id).accounts({
  //     depositAccount: deposit_pda,
  //     packageAccount: packagePda,
  //     tokenMint: token_mint,
  //     depositAta: getAssociatedTokenAddressSync(token_mint, deposit_pda, true),
  //     userDeposit: userPda,
  //     userAta: getAssociatedTokenAddressSync(token_mint, provider.wallet.publicKey, true),
  //     user: provider.wallet.publicKey,
  //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  //     tokenProgram: TOKEN_PROGRAM_ID,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   }).rpc();

  //   console.log("Deposit by usdc tx: ", tx);

   
  // })



//   it("withdraw by owner", async () => {
  
//     const tx = await program.methods.adminWithdraw().accounts({
//           depositAccount: deposit_pda,
//           authority: provider.wallet.publicKey,
//         }).rpc();
//         console.log("admin withdraw sol: ", tx);
//         // let deposit_pda_info = await program.account.deposit.fetch(deposit_pda);
//         // console.log(JSON.stringify(deposit_pda_info));
//   }
// )


  // console.log("wallet:", provider.publicKey.toString());
  

});
