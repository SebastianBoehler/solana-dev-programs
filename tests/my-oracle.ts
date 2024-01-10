import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { OracleCpi } from "../target/types/oracle_cpi";
import { MyOracle } from "../target/types/my_oracle";

anchor.setProvider(anchor.AnchorProvider.env());

describe.skip("oracle", () => {
  const oracleM = anchor.workspace.OracleCpi as Program<OracleCpi>;
  const oracle = anchor.workspace.MyOracle as Program<MyOracle>;

  const user = anchor.Wallet.local().payer;
  const [oraclePda, _] = anchor.web3.PublicKey.findProgramAddressSync(
    [anchor.utils.bytes.utf8.encode("oracle"), user.publicKey.toBuffer()],
    oracle.programId
  );

  it.skip("Initialize account", async () => {
    const tx = await oracle.methods
      .initialize("BTC/USD")
      .accounts({
        user: anchor.Wallet.local().payer.publicKey,
        dataStore: oraclePda,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("Write price data", async () => {
    const tx = await oracle.methods
      .update(new BN(Math.floor(Math.random() * 100000)))
      .accounts({
        dataStore: oraclePda,
      })
      .rpc();

    console.log("Your transaction signature", tx);
  });

  it("testing pullData", async () => {
    // Add your test here.
    const tx = await oracleM.methods
      .pullData()
      .accounts({
        oraclePda,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });

  it("testing pushData", async () => {
    // Add your test here.
    const tx = await oracleM.methods
      .pushData(new BN(Math.floor(Math.random() * 100000)))
      .accounts({
        oraclePda,
        oracleProgram: oracle.programId,
        signer: user.publicKey,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
});
