// scripts/init.ts
import * as anchor from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";

const MINT_ADDRESS = "5xa7QFVJsw29gVnCYR3885kZk8F5LCWnwGXmysyKX8pK"; // replace with the result from create_mint.ts

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Eog as anchor.Program;

  // PDAs
  const [state] = PublicKey.findProgramAddressSync([Buffer.from("state")], program.programId);
  const [houseVault] = PublicKey.findProgramAddressSync([Buffer.from("house")], program.programId);
  const [kitchenVault] = PublicKey.findProgramAddressSync([Buffer.from("kitchen")], program.programId);
  const [devVault] = PublicKey.findProgramAddressSync([Buffer.from("dev")], program.programId);

  console.log("State   :", state.toBase58());
  console.log("House   :", houseVault.toBase58());
  console.log("Dev     :", devVault.toBase58());
  console.log("Kitchen :", kitchenVault.toBase58());

  await program.methods
    .initialize()
    .accounts({
      state,
      houseVault,
      devVault,
      kitchenVault,
      mint: new PublicKey(MINT_ADDRESS),
      authority: provider.wallet.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      rent: anchor.web3.SYSVAR_RENT_PUBKEY,
    })
    .rpc();

  console.log("EOG vaults initialized with mint!");
}

main().catch(console.error);
