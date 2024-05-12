import { PublicKey } from "@solana/web3.js";
import * as anchor from "@coral-xyz/anchor";


const DEPOSIT_ACCOUNT = "DEPOSIT_ACCOUNT";
const ADMIN_ROLE = "ADMIN_ROLE";
const OPERATOR_ROLE ="OPERATOR_ROLE";
const USER_ACCOUNT = "USER_ACCOUNT";
const PACKAGE = "PACKAGE";

export const getDepositPda =  ( programId: PublicKey): PublicKey => {
    const [mint, _] =   anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(DEPOSIT_ACCOUNT)],
        programId
      );
    return mint;
}

export const getAdminRolePda =  ( programId: PublicKey, auth: PublicKey) : PublicKey=> {
    const [mint, _] =   anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(ADMIN_ROLE), auth.toBuffer()],
        programId
      );
    return mint;
}

export const getOperatorRolePda =  ( programId: PublicKey,  auth: PublicKey): PublicKey => {
    const [mint, _] =   anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(OPERATOR_ROLE), auth.toBuffer()],
        programId
      );
    return mint;
}


export const getPackagePda =  ( programId: PublicKey, id: number): PublicKey => {
    const [mint, _] =   anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(PACKAGE), new anchor.BN(id).toBuffer('le', 2)],
        programId
      );
    return mint;
}

export const getUserPda =  ( programId: PublicKey): PublicKey => {
    const [mint, _] =   anchor.web3.PublicKey.findProgramAddressSync(
        [Buffer.from(USER_ACCOUNT)],
        programId
      );
    return mint; 
}