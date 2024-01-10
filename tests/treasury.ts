import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Treasury } from "../target/types/treasury";
import { Keypair, PublicKey } from "@solana/web3.js";

anchor.setProvider(anchor.AnchorProvider.env());

describe("treasury", () => {
  const treasury = anchor.workspace.MyOracle as Program<Treasury>;

  const userSending = anchor.Wallet.local().payer;
  const userReceiving = Keypair.generate();
  const identifier = new BN(32213123);

  const [escrowPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("state"),
      userSending.publicKey.toBuffer(),
      userReceiving.publicKey.toBuffer(),
      identifier.toBuffer(),
    ],
    treasury.programId
  );

  const [escrowWallet] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("wallet"),
      userSending.publicKey.toBuffer(),
      userReceiving.publicKey.toBuffer(),
      identifier.toBuffer(),
    ],
    treasury.programId
  );

  it("Initialize escrow", async () => {
    const tx = treasury.methods
      .initialize(identifier, new BN(500000), 4) //0.5 USDC
      .accounts({
        userSending: userSending.publicKey,
        userReceiving: userReceiving.publicKey,
        mintOfTokenBeingSent: new PublicKey(
          "4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU"
        ),
        escrowPda,
        escrowWallet,
      });

    console.log("Your transaction signature", tx);
  });
});
