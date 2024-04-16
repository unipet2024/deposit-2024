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
const programId = "3dHnbHNAVfx1u27VSoCszk3xbkrfVLXnP3BCxrp3AZju";
// Generate the program client from IDL.
const program = new anchor.Program(idl, programId, provider);

async function set_package() {
  let owner = provider.wallet as Wallet;
  // const payer = owner.payer;
  // Configure the client to use the local cluster.
  anchor.setProvider(provider);

  // const holder = new PublicKey("ESAaePH3mJjw9zZxnLGfnR1jVdnA7ieq2YaYeu8NcKum");

  const deposit_pda = getDepositAccount();
  // const admin_pda = getAdminAccount();
  const operator_pda = getOperatorAccount();
  const usdc = new PublicKey("BUJST4dk6fnM5G3FnhTVc3pjxRJE7w2C5YL9XgLbdsXW");

  let usdc_packages = [
    {
      id: new anchor.BN(1001),
      price: new anchor.BN(99 * 10 ** 4),
      reward: new anchor.BN(60),
    },
    {
      id: new anchor.BN(1002),
      price: new anchor.BN(499 * 10 ** 4),
      reward: new anchor.BN(300),
    },
    {
      id: new anchor.BN(1003),
      price: new anchor.BN(999 * 10 ** 4),
      reward: new anchor.BN(600),
    },
    {
      id: new anchor.BN(1004),
      price: new anchor.BN(1499 * 10 ** 4),
      reward: new anchor.BN(980),
    },
    {
      id: new anchor.BN(1005),
      price: new anchor.BN(2999 * 10 ** 4),
      reward: new anchor.BN(1980),
    },
    {
      id: new anchor.BN(1006),
      price: new anchor.BN(4999 * 10 ** 4),
      reward: new anchor.BN(3280),
    },
    {
      id: new anchor.BN(1007),
      price: new anchor.BN(9999 * 10 ** 4),
      reward: new anchor.BN(6480),
    },
    {
      id: new anchor.BN(1008),
      price: new anchor.BN(99 * 10 ** 4),
      reward: new anchor.BN(0),
    },
    {
      id: new anchor.BN(1009),
      price: new anchor.BN(999 * 10 ** 4),
      reward: new anchor.BN(3000),
    },
    {
      id: new anchor.BN(1010),
      price: new anchor.BN(2999 * 10 ** 4),
      reward: new anchor.BN(3000),
    },

    {
      id: new anchor.BN(2001),
      price: new anchor.BN(99 * 10 ** 4),
      reward: new anchor.BN(60),
    },
    {
      id: new anchor.BN(2002),
      price: new anchor.BN(499 * 10 ** 4),
      reward: new anchor.BN(300),
    },
    {
      id: new anchor.BN(2003),
      price: new anchor.BN(999 * 10 ** 4),
      reward: new anchor.BN(600),
    },
    {
      id: new anchor.BN(2004),
      price: new anchor.BN(1499 * 10 ** 4),
      reward: new anchor.BN(980),
    },
    {
      id: new anchor.BN(2005),
      price: new anchor.BN(2999 * 10 ** 4),
      reward: new anchor.BN(1980),
    },
    {
      id: new anchor.BN(2006),
      price: new anchor.BN(4999 * 10 ** 4),
      reward: new anchor.BN(3280),
    },
    {
      id: new anchor.BN(2007),
      price: new anchor.BN(9999 * 10 ** 4),
      reward: new anchor.BN(6480),
    },
    {
      id: new anchor.BN(2008),
      price: new anchor.BN(99 * 10 ** 4),
      reward: new anchor.BN(0),
    },
    {
      id: new anchor.BN(2009),
      price: new anchor.BN(999 * 10 ** 4),
      reward: new anchor.BN(3000),
    },
    {
      id: new anchor.BN(2010),
      price: new anchor.BN(2999 * 10 ** 4),
      reward: new anchor.BN(3000),
    },

    {
      id: new anchor.BN(3001),
      price: new anchor.BN(99 * 10 ** 4),
      reward: new anchor.BN(60),
    },
    {
      id: new anchor.BN(3002),
      price: new anchor.BN(499 * 10 ** 4),
      reward: new anchor.BN(300),
    },
    {
      id: new anchor.BN(3003),
      price: new anchor.BN(999 * 10 ** 4),
      reward: new anchor.BN(600),
    },
    {
      id: new anchor.BN(3004),
      price: new anchor.BN(1499 * 10 ** 4),
      reward: new anchor.BN(980),
    },
    {
      id: new anchor.BN(3005),
      price: new anchor.BN(2999 * 10 ** 4),
      reward: new anchor.BN(1980),
    },
    {
      id: new anchor.BN(3006),
      price: new anchor.BN(4999 * 10 ** 4),
      reward: new anchor.BN(3280),
    },
    {
      id: new anchor.BN(3007),
      price: new anchor.BN(9999 * 10 ** 4),
      reward: new anchor.BN(6480),
    },
    {
      id: new anchor.BN(3008),
      price: new anchor.BN(99 * 10 ** 4),
      reward: new anchor.BN(0),
    },
    {
      id: new anchor.BN(3009),
      price: new anchor.BN(999 * 10 ** 4),
      reward: new anchor.BN(3000),
    },
    {
      id: new anchor.BN(3010),
      price: new anchor.BN(2999 * 10 ** 4),
      reward: new anchor.BN(3000),
    },
  ];

  let usdc_package_pda = getPackageAccount(usdc);

  try {
    await program.methods
      .setPackages(usdc, usdc_packages, true)
      .accounts({
        deposit: deposit_pda,
        operatorAccount: operator_pda,
        packageAccount: usdc_package_pda,
      })
      .rpc();
  } catch (error) {
    console.log(error);
  }

  let usdc_package_info = await program.account.package.fetch(usdc_package_pda);
  console.log(usdc_package_info);
}

const getDepositAccount = () => {
  let [deposit_pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("DEPOSIT_ACCOUNT")],
    program.programId
  );
  console.log("Deposit account: : ", deposit_pda.toString());
  return deposit_pda;
};

const getPackageAccount = (token) => {
  let [package_pda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("PACKAGE"), token.toBuffer()],
    program.programId
  );

  console.log("package account: ", package_pda.toString());
  return package_pda;
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

set_package();
