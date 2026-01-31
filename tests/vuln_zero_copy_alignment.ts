import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnZeroCopyAlignment } from "../target/types/vuln_zero_copy_alignment";
import { expect } from "chai";

describe("vuln_zero_copy_alignment", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnZeroCopyAlignment as Program<VulnZeroCopyAlignment>;
  const provider = anchor.getProvider();

  it("Initializes and updates zero-copy account", async () => {
    const data = anchor.web3.Keypair.generate();

    await program.methods
      .initialize()
      .accounts({
        data: data.publicKey,
        user: provider.publicKey,
      })
      .signers([data])
      .rpc();

    await program.methods
      .updateSecure(new anchor.BN(100))
      .accounts({
        data: data.publicKey,
      })
      .rpc();

    const account = await program.account.bigData.fetch(data.publicKey);
    expect(account.val.toNumber()).to.equal(100);
  });

  it("Attempts insecure update (Manual Cast)", async () => {
    const data = anchor.web3.Keypair.generate();

    await program.methods
      .initialize()
      .accounts({
        data: data.publicKey,
        user: provider.publicKey,
      })
      .signers([data])
      .rpc();

    // This might succeed in the test environment because accounts are usually 
    // 8-byte aligned, but the unsafe cast in the program is the "vulnerability".
    await program.methods
      .updateInsecure(new anchor.BN(200))
      .accounts({
        data: data.publicKey,
      })
      .rpc();
  });
});
