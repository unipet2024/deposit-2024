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

describe("deposit-2024", () => {
  let provider = anchor.AnchorProvider.env();

  anchor.setProvider(provider);

  const program = anchor.workspace.Deposit2024 as Program<Deposit2024>;

  it("Is initialized!", async () => {
    // Generate keypair for the new account

    // let user1 = new anchor.web3.Keypair();
    // console.log("User1: ", user1.publicKey.toString());

    // let user2 = new anchor.web3.Keypair();
    // console.log("User2: ", user2.publicKey.toString());

    // let user3 = new anchor.web3.Keypair();
    // console.log("User3: ", user3.publicKey.toString());

    // let owner = provider.wallet as Wallet;

    // await SystemProgram.transfer({
    //   fromPubkey: owner.publicKey,
    //   toPubkey: user1.publicKey,
    //   lamports: LAMPORTS_PER_SOL,
    // });

    // await program.provider.connection.requestAirdrop(
    //   user1.publicKey,
    //   2 * LAMPORTS_PER_SOL
    // );

    // let user1_balance = await program.provider.connection.getBalance(
    //   user1.publicKey
    // );
    // console.log("User 1 balance: ", user1_balance);

    // await SystemProgram.transfer({
    //   fromPubkey: owner.publicKey,
    //   toPubkey: user2.publicKey,
    //   lamports: LAMPORTS_PER_SOL,
    // });

    // await program.provider.connection.requestAirdrop(
    //   user2.publicKey,
    //   2 * LAMPORTS_PER_SOL
    // );

    // let usdtMint = await createMint(
    //   program.provider.connection,
    //   owner.payer,
    //   owner.publicKey,
    //   null,
    //   0
    // );
    // console.log("USDT MINT: ", usdtMint.toString());

    // let adminUsdtAta = await createAssociatedTokenAccount(
    //   program.provider.connection,
    //   owner.payer,
    //   usdtMint,
    //   owner.publicKey
    // );
    // console.log("USDT ADMIN ATA:", adminUsdtAta.toString());

    // let user3UsdtAta = await createAssociatedTokenAccount(
    //   program.provider.connection,
    //   owner.payer,
    //   usdtMint,
    //   user3.publicKey
    // );
    // console.log("USDT USER 3 ATA:", user3UsdtAta.toString());

    // let user1UsdtAta = await createAssociatedTokenAccount(
    //   program.provider.connection,
    //   owner.payer,
    //   usdtMint,
    //   user1.publicKey
    // );
    // console.log("USDT USER 1 ATA:", user1UsdtAta.toString());

    // let user2UsdtAta = await createAssociatedTokenAccount(
    //   program.provider.connection,
    //   owner.payer,
    //   usdtMint,
    //   user2.publicKey
    // );
    // console.log("USDT USER 2 ATA:", user2UsdtAta.toString());

    let [deposit_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("DEPOSIT_ACCOUNT")],
      program.programId
    );
    console.log("DEPOSIT PDA ACCOUNT: ", deposit_pda.toString());

    // let depositUsdtAta = await getOrCreateAssociatedTokenAccount(
    //   program.provider.connection,
    //   owner.payer,
    //   usdtMint,
    //   deposit_pda,
    //   true
    // );
    // console.log("USDT DEPOSIT ATA:", depositUsdtAta.toString());

    let [admin_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("ADMIN_ROLE")],
      program.programId
    );
    console.log("ADMIN PDA ACCOUNT: ", admin_pda.toString());

    let [operator_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("OPERATOR_ROLE")],
      program.programId
    );
    console.log("OPERATOR PDA ACCOUNT: ", operator_pda.toString());
    console.log("Start init ---------");
    // try {
    //   await program.methods
    //     .init()
    //     .accounts({
    //       deposit: deposit_pda,
    //       adminAccount: admin_pda,
    //       operatorAccount: operator_pda,
    //     })
    //     .rpc();
    // } catch (error) {
    //   console.log(error)
    // }

    console.log("End init ---------");

    let deposit_pda_info = await program.account.deposit.fetch(deposit_pda);
    console.log(deposit_pda_info);
    assert.equal(deposit_pda_info.admin.toString(), admin_pda.toString());
    assert.equal(deposit_pda_info.operator.toString(), operator_pda.toString());

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

    let [usdc_package_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("PACKAGE"), usdc.toBuffer()],
      program.programId
    );

    console.log("USDC package pda: ", usdc_package_pda.toString());

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

    let usdc_package_info = await program.account.package.fetch(
      usdc_package_pda
    );
    console.log(usdc_package_info);

    /*
    let admin_pda_inro = await program.account.authorityRole.fetch(admin_pda);
    console.log(admin_pda_inro);
    // assert.equal(
    //   admin_pda_inro.authority.toString(),
    //   owner.publicKey.toString()
    // );
    // assert.equal(admin_pda_inro.role, "Admin");

    let operator_pda_inro = await program.account.authorityRole.fetch(
      operator_pda
    );

    console.log(operator_pda_inro);
    // assert.equal(
    //   operator_pda_inro.authority.toString(),
    //   owner.publicKey.toString()
    // );

    let [usdt_package_pda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("PACKAGE"), usdtMint.toBuffer()],
      program.programId
    );

    console.log("USDC Package: ", usdt_package_pda.toString());

    let usdt_packages = [
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
    await program.methods
      .setPackages(usdtMint, usdt_packages, true)
      .accounts({
        deposit: deposit_pda,
        operatorAccount: operator_pda,
        packageAccount: usdt_package_pda,
      })
      // .signers()
      .rpc();

    let usdt_package_pda_info = await program.account.package.fetch(
      usdt_package_pda
    );

    console.log("usdt_package_pda_info: ", usdt_package_pda_info.packages);
    // return;

    console.log("--------------------------------------");
    console.log("User 1 deposi 100 USDT");

    let [user1_deposit] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("USER_ACCOUNT"),
        // deposit_pda.toBuffer(),
        usdtMint.toBuffer(),
        user1.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("USER 1 DEPOSIT PDA ACCOUNT: ", user1_deposit.toString());

    let [user2_usdt_deposit] = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("USER_ACCOUNT"),
        deposit_pda.toBuffer(),
        usdtMint.toBuffer(),
        user2.publicKey.toBuffer(),
      ],
      program.programId
    );
    console.log("USER 2 DEPOSIT PDA ACCOUNT: ", user2_usdt_deposit.toString());

    console.log("Mint 1000 USDT to user 1");
    await mintTo(
      program.provider.connection,
      owner.payer,
      usdtMint,
      user1UsdtAta,
      owner.payer,
      1000 * 10 ** 6
    );

    console.log("Mint 1000 USDT to user 2");
    await mintTo(
      program.provider.connection,
      owner.payer,
      usdtMint,
      user2UsdtAta,
      owner.payer,
      1000 * 10 ** 6
    );

    console.log("User 1 deposit package id 1002");
    try {
      await program.methods
        .deposit(new anchor.BN(1002))
        .accounts({
          deposit: deposit_pda,
          user: user1.publicKey,
          tokenMint: usdtMint,
          userAta: user1UsdtAta,
          depositAta: depositUsdtAta.address,
          userDeposit: user1_deposit,
          packageAccount: usdt_package_pda,
        })
        .signers([user1])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    console.log("User 1 deposit package id 1002 completed");

    let user1DepositInfo = await program.account.user.fetch(user1_deposit);
    // console.log(user1DepositInfo);

    assert.equal(
      user1DepositInfo.amount.toNumber(),
      4.99 * 10 ** 6,
      "User 1 deposit invalid"
    );

    let user1UsdtBalance =
      await program.provider.connection.getTokenAccountBalance(user1UsdtAta);

    let depositUsdtBalance =
      await program.provider.connection.getTokenAccountBalance(
        depositUsdtAta.address
      );

    assert.equal(
      depositUsdtBalance.value.amount,
      "4990000",
      "deposit balance invalid"
    );
    try {
      await program.methods
        .deposit(new anchor.BN(1002))
        .accounts({
          deposit: deposit_pda,
          user: user1.publicKey,
          tokenMint: usdtMint,
          userAta: user1UsdtAta,
          depositAta: depositUsdtAta.address,
          userDeposit: user1_deposit,
          packageAccount: usdt_package_pda,
        })
        .signers([user1])
        .rpc();
    } catch (error) {
      console.log(error);
    }

    user1DepositInfo = await program.account.user.fetch(user1_deposit);
    console.log(user1DepositInfo);

    // assert.equal(
    //   user1DepositInfo.amount.toNumber(),
    //   300,
    //   "User 1 deposit invalid"
    // );

    // user1UsdtBalance = await program.provider.connection.getTokenAccountBalance(
    //   user1UsdtAta
    // );

    // assert.equal(
    //   user1UsdtBalance.value.amount,
    //   "700",
    //   "User 1 balance invalid"
    // );

    // depositUsdtBalance =
    //   await program.provider.connection.getTokenAccountBalance(
    //     depositUsdtAta.address
    //   );

    // assert.equal(
    //   depositUsdtBalance.value.amount,
    //   "300",
    //   "deposit balance invalid"
    // );

    console.log("User 2 deposit 100");
    await program.methods
      .deposit(new anchor.BN(2003))
      .accounts({
        deposit: deposit_pda,
        user: user2.publicKey,
        tokenMint: usdtMint,
        userAta: user2UsdtAta,
        depositAta: depositUsdtAta.address,
        userDeposit: user2_usdt_deposit,
      })
      .signers([user2])
      .rpc();

    let user2DepositInfo = await program.account.user.fetch(user2_usdt_deposit);
    console.log(user1DepositInfo);

    // assert.equal(
    //   user2DepositInfo.amount.toNumber(),
    //   100,
    //   "User 2 deposit invalid"
    // );

    // let user2UsdtBalance =
    //   await program.provider.connection.getTokenAccountBalance(user2UsdtAta);

    // assert.equal(
    //   user2UsdtBalance.value.amount,
    //   "900",
    //   "User 2 balance invalid"
    // );

    // depositUsdtBalance =
    //   await program.provider.connection.getTokenAccountBalance(
    //     depositUsdtAta.address
    //   );

    // assert.equal(
    //   depositUsdtBalance.value.amount,
    //   "400",
    //   "deposit balance invalid"
    // );

    // await program.methods
    //   .withdraw(new anchor.BN(100))
    //   .accounts({
    //     deposit: deposit_pda,
    //     admin: owner.publicKey,
    //     tokenMint: usdtMint,
    //     depositAta: depositUsdtAta.address,
    //     adminAta: adminUsdtAta,
    //     adminAccount: admin_pda,
    //   })
    //   .rpc();

    // depositUsdtBalance =
    //   await program.provider.connection.getTokenAccountBalance(
    //     depositUsdtAta.address
    //   );

    // assert.equal(
    //   depositUsdtBalance.value.amount,
    //   "300",
    //   "deposit balance invalid"
    // );

    // let adminUsdtBalance =
    //   await program.provider.connection.getTokenAccountBalance(adminUsdtAta);

    // assert.equal(adminUsdtBalance.value.amount, "100", "admin balance invalid");

    // await program.methods
    //   .withdraw(new anchor.BN(150))
    //   .accounts({
    //     deposit: deposit_pda,
    //     admin: owner.publicKey,
    //     tokenMint: usdtMint,
    //     depositAta: depositUsdtAta.address,
    //     adminAta: adminUsdtAta,
    //     adminAccount: admin_pda,
    //   })
    //   .rpc();

    // depositUsdtBalance =
    //   await program.provider.connection.getTokenAccountBalance(
    //     depositUsdtAta.address
    //   );

    // assert.equal(
    //   depositUsdtBalance.value.amount,
    //   "150",
    //   "deposit balance invalid"
    // );

    // adminUsdtBalance = await program.provider.connection.getTokenAccountBalance(
    //   adminUsdtAta
    // );

    // assert.equal(adminUsdtBalance.value.amount, "250", "admin balance invalid");

    // await program.methods
    //   .withdraw(new anchor.BN(150))
    //   .accounts({
    //     deposit: deposit_pda,
    //     admin: user3.publicKey,
    //     tokenMint: usdtMint,
    //     depositAta: depositUsdtAta.address,
    //     adminAta: user3UsdtAta,
    //     adminAccount: admin_pda,
    //   })
    //   .signers([user3])
    //   .rpc();

    // let fakeMint = await createMint(
    //   program.provider.connection,
    //   owner.payer,
    //   owner.publicKey,
    //   null,
    //   0
    // );
    // console.log("FAKE MINT: ", fakeMint.toString());

    // console.log("User 2 deposit 100 fake token");

    // let user2FakeAta = await createAssociatedTokenAccount(
    //   program.provider.connection,
    //   owner.payer,
    //   fakeMint,
    //   user2.publicKey
    // );
    // console.log("FAKE USER 2 ATA:", user2FakeAta.toString());

    // let adminFakeAta = await createAssociatedTokenAccount(
    //   program.provider.connection,
    //   owner.payer,
    //   fakeMint,
    //   owner.publicKey
    // );
    // console.log("FAKE Admin  ATA:", adminFakeAta.toString());

    // let depositFakeAta = await getOrCreateAssociatedTokenAccount(
    //   program.provider.connection,
    //   owner.payer,
    //   fakeMint,
    //   deposit_pda,
    //   true
    // );
    // console.log("FAKE DEPOSIT ATA:", depositFakeAta.address.toString());

    // let [user2_fake_deposit] = anchor.web3.PublicKey.findProgramAddressSync(
    //   [
    //     Buffer.from("USER_ACCOUNT"),
    //     deposit_pda.toBuffer(),
    //     fakeMint.toBuffer(),
    //     user2.publicKey.toBuffer(),
    //   ],
    //   program.programId
    // );
    // console.log("USER 2 DEPOSIT PDA ACCOUNT: ", user2_fake_deposit.toString());

    // await program.methods
    //   .deposit(new anchor.BN(100))
    //   .accounts({
    //     deposit: deposit_pda,
    //     user: user2.publicKey,
    //     tokenMint: fakeMint,
    //     userAta: user2FakeAta,
    //     depositAta: depositFakeAta.address,
    //     userDeposit: user2_fake_deposit,
    //   })
    //   .signers([user2])
    //   .rpc();

    // console.log("Set operator");
    // await program.methods.
    //   .setOperator(operator.publicKey)
    //   .accounts({
    //     deposit: deposit_pda,
    //     adminAccount: admin_pda,
    //     operatorAccount: operator_pda,
    //   })
    //   .rpc();

    // let operator_pda_info = await program.account.authorityRole.fetch(
    //   operator_pda
    // );
    // console.log(operator_pda_info);
    // assert.equal(
    //   operator_pda_info.authority.toString(),
    //   operator.publicKey.toString()
    // );

    // await program.methods
    //   .setTokens({ token: [usdtMint, fakeMint] })
    //   .accounts({
    //     deposit: deposit_pda,
    //     operatorAccount: operator_pda,
    //     operator: operator.publicKey,
    //   })
    //   .signers([operator])
    //   .rpc();

    // deposit_pda_info = await program.account.deposit.fetch(deposit_pda);
    // console.log(deposit_pda_info);

    // console.log("Mint 1000 Fake to user 2");
    // await mintTo(
    //   program.provider.connection,
    //   owner.payer,
    //   fakeMint,
    //   user2FakeAta,
    //   owner.payer,
    //   1000
    // );

    // await program.methods
    //   .deposit(new anchor.BN(100))
    //   .accounts({
    //     deposit: deposit_pda,
    //     user: user2.publicKey,
    //     tokenMint: fakeMint,
    //     userAta: user2FakeAta,
    //     depositAta: depositFakeAta.address,
    //     userDeposit: user2_fake_deposit,
    //   })
    //   .signers([user2])
    //   .rpc();

    // console.log("Admin close deposit");
    // await program.methods
    //   .setStatus(false)
    //   .accounts({
    //     deposit: deposit_pda,
    //     adminAccount: admin_pda,
    //   })
    //   .rpc();

    // deposit_pda_info = await program.account.deposit.fetch(deposit_pda);
    // console.log(deposit_pda_info);

    // await program.methods
    //   .deposit(new anchor.BN(100))
    //   .accounts({
    //     deposit: deposit_pda,
    //     user: user2.publicKey,
    //     tokenMint: fakeMint,
    //     userAta: user2FakeAta,
    //     depositAta: depositFakeAta.address,
    //     userDeposit: user2_fake_deposit,
    //   })
    //   .signers([user2])
    //   .rpc();

    // await program.methods
    //   .withdraw(new anchor.BN(100))
    //   .accounts({
    //     deposit: deposit_pda,
    //     admin: owner.publicKey,
    //     tokenMint: fakeMint,
    //     depositAta: depositFakeAta.address,
    //     adminAta: adminFakeAta,
    //     adminAccount: admin_pda,
    //   })
    //   .rpc();

    */
  });
});
