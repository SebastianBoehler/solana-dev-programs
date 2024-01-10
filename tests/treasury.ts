import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { Treasury } from "../target/types/treasury";
import { Keypair, PublicKey } from "@solana/web3.js";
import { getAssociatedTokenAddress } from "@solana/spl-token";
import { expect } from "chai";

anchor.setProvider(anchor.AnchorProvider.env());

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

describe("treasury", async () => {
  const treasury = anchor.workspace.Treasury as Program<Treasury>;

  const userSending = anchor.Wallet.local().payer;
  const userReceiving = Keypair.generate();
  const identifier = new BN(32213123);

  const [escrowPda, pdaBump] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("state"),
      userSending.publicKey.toBuffer(),
      userReceiving.publicKey.toBuffer(),
      //identifier.toBuffer(),
    ],
    treasury.programId
  );

  const [escrowWallet] = anchor.web3.PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("wallet"),
      userSending.publicKey.toBuffer(),
      userReceiving.publicKey.toBuffer(),
      //identifier.toBuffer(),
    ],
    treasury.programId
  );

  const mint = new PublicKey("4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU");
  const bump = 4;

  const userATA = await getAssociatedTokenAddress(mint, userSending.publicKey);

  it("Initialize escrow", async () => {
    const tx = await treasury.methods
      .initialize(identifier, new BN(500000), pdaBump) //0.5 USDC
      .accounts({
        userSending: userSending.publicKey,
        userReceiving: userReceiving.publicKey,
        mintOfTokenBeingSent: mint,
        escrowPda,
        escrowWallet,
        usersSendingTokenAccount: userATA,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    await sleep(1000);

    const escrowAccount = await treasury.account.state.fetch(escrowPda);

    expect(escrowAccount.amount.toNumber()).to.be.equal(500000);
    expect(escrowAccount.userSending.toBase58()).to.be.equal(
      userSending.publicKey.toBase58()
    );
  });

  it("PullBack funds", async () => {
    const tx = await treasury.methods
      .pullBack(identifier, pdaBump)
      .accounts({
        escrowPda,
        escrowWallet,
        userSending: userSending.publicKey,
        userReceiving: userReceiving.publicKey,
        mintOfTokenBeingSent: mint,
        usersSendingTokenAccount: userATA,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    await sleep(1000);

    const escrowAccount = await treasury.account.state.fetch(escrowPda);
    expect(escrowAccount.amount.toNumber()).to.be.equal(0);
  });
});
