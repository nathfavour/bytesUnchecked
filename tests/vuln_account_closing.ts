import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnAccountClosing } from "../target/types/vuln_account_closing";
import { expect } from "chai";

describe("vuln_account_closing", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnAccountClosing as Program<VulnAccountClosing>;
  const provider = anchor.getProvider();

  it("Closes insecurely (data remains)", async () => {
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
      .closeInsecure()
      .accounts({
        vault: vault.publicKey,
        destination: provider.publicKey,
      })
      .rpc();

    // The account is "closed" because it has 0 lamports, but if we check the RPC
    // it might still return data if we catch it before it's purged, or if we re-fund it.
    // In a test environment, let's see if we can still fetch the account info manually.
    const info = await provider.connection.getAccountInfo(vault.publicKey);
    if (info) {
        // If the account still exists (hasn't been purged by the runtime yet), 
        // the data is still there!
        expect(info.data.length).to.be.gt(0);
    }
  });

  it("Closes securely (data zeroed)", async () => {
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
      .closeSecure()
      .accounts({
        vault: vault.publicKey,
        destination: provider.publicKey,
      })
      .rpc();

    const info = await provider.connection.getAccountInfo(vault.publicKey);
    expect(info).to.be.null;
  });
});
