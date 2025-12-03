import * as anchor from "@coral-xyz/anchor";
import { createMint } from "@solana/spl-token";

async function main() {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  if (!provider.wallet.payer) {
    throw new Error("Provider wallet does not have a payer!");
  }

  const mint = await createMint(
    provider.connection,
    provider.wallet.payer,      // payer (explicitly exists)
    provider.wallet.publicKey,  // mint authority
    provider.wallet.publicKey,  // freeze authority
    6                           // decimals
  );

  console.log("Mint created:", mint.toBase58());
}

main().catch(console.error);
