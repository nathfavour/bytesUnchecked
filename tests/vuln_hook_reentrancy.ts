import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnHookReentrancy } from "../target/types/vuln_hook_reentrancy";
import { expect } from "chai";

describe("vuln_hook_reentrancy", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnHookReentrancy as Program<VulnHookReentrancy>;
  const provider = anchor.getProvider();

  it("Performs withdrawal (Insecure)", async () => {
    const vault = anchor.web3.Keypair.generate();

    await program.methods
      .initialize()
      .accounts({
        vault: vault.publicKey,
        user: provider.publicKey,
      })
      .signers([vault])
      .rpc();

    await program.methods
      .withdrawInsecure(new anchor.BN(100))
      .accounts({
        vault: vault.publicKey,
        destination: provider.publicKey,
      })
      .rpc();

    const account = await program.account.vault.fetch(vault.publicKey);
    expect(account.balance.toNumber()).to.equal(900);
  });

  it("Performs withdrawal (Secure)", async () => {
    const vault = anchor.web3.Keypair.generate();

    await program.methods
      .initialize()
      .accounts({
        vault: vault.publicKey,
        user: provider.publicKey,
      })
      .signers([vault])
      .rpc();

    await program.methods
      .withdrawSecure(new anchor.BN(100))
      .accounts({
        vault: vault.publicKey,
        destination: provider.publicKey,
      })
      .rpc();

    const account = await program.account.vault.fetch(vault.publicKey);
    expect(account.balance.toNumber()).to.equal(900);
  });
});
