import * as anchor from "@coral-xyz/anchor";
import { BN, Program } from "@coral-xyz/anchor";
import { OracleMaster } from "../target/types/oracle_master";
import { MyOracle } from "../target/types/my_oracle";

describe("my-oracle", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const oracleM = anchor.workspace.MyOracle as Program<OracleMaster>;
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
});
