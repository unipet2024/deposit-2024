import * as anchor from "@coral-xyz/anchor";
import { Program, AnchorProvider } from "@coral-xyz/anchor";
import { IDL } from "../target/types/deposit_2024";
import { Wallet } from "@coral-xyz/anchor";

// import data from "../keys/dev/holder.json";

// import { setTimeout } from "timers/promises";

import { PublicKey, Keypair, Connection, clusterApiUrl } from "@solana/web3.js";
const connection = new Connection(clusterApiUrl("devnet"), "confirmed");

const wallet = Keypair.fromSecretKey(
  Uint8Array.from([
    60, 31, 216, 134, 68, 78, 5, 54, 175, 135, 221, 227, 168, 70, 131, 114, 133,
    65, 139, 93, 195, 126, 28, 32, 17, 15, 252, 196, 1, 237, 44, 57, 8, 134, 50,
    123, 56, 199, 184, 99, 61, 162, 196, 68, 143, 51, 117, 64, 26, 54, 84, 218,
    154, 157, 209, 231, 34, 3, 251, 190, 216, 153, 90, 113,
  ])
);
console.log("Wallet:", wallet.publicKey.toString());

new Wallet(wallet);

const provider = new AnchorProvider(
  connection,
  new Wallet(wallet),
  anchor.AnchorProvider.defaultOptions()
);
// console.log("Provider: ", provider);

const idl = IDL;
// Address of the deployed program.
const programId = "Huj8fVYRHM1asSm58Zy7YuwiUYoWa9nTzj7arVK4V4Uo";
// Generate the program client from IDL.
const program = new anchor.Program(idl, programId, provider);

async function init() {
  let owner = provider.wallet as Wallet;
  // const payer = owner.payer;
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // const holder = new PublicKey("ESAaePH3mJjw9zZxnLGfnR1jVdnA7ieq2YaYeu8NcKum");

  const deposit_pda = getDepositAccount();
  const admin_pda = getAdminAccount();
  const operator_pda = getOperatorAccount();

  try {
    await program.methods
      .init()
      .accounts({
        deposit: deposit_pda,
        adminAccount: admin_pda,
        operatorAccount: operator_pda,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  let deposit_account_info = await program.account.deposit.fetch(deposit_pda);
  console.log(deposit_account_info);
}

const getDepositAccount = () => {
  let [deposit_pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("DEPOSIT_ACCOUNT")],
    program.programId
  );
  console.log("Deposit account: : ", deposit_pda.toString());
  return deposit_pda;
};

const getOperatorAccount = () => {
  const OPERATOR_ROLE = "OPERATOR_ROLE";
  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(OPERATOR_ROLE)],
    program.programId
  );
  console.log("operator_account: ", mint.toString());
  return mint;
};

const getAdminAccount = () => {
  const ADMIN_ROLE = "ADMIN_ROLE";
  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(ADMIN_ROLE)],
    program.programId
  );
  console.log("admin_account: ", mint.toString());

  return mint;
};

init();
