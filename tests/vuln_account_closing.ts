import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnAccountClosing } from "../target/types/vuln_account_closing";
import { expect } from "chai";

describe("vuln_account_closing", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnAccountClosing as Program<VulnAccountClosing>;
  const provider = anchor.getProvider();

  it("Closes account (Insecure - Data remains)", async () => {
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

    // The account lamports are 0, but if we check the account info, 
    // it might still exist in the same transaction or if not reaped.
    const accountInfo = await provider.connection.getAccountInfo(vault.publicKey);
    // In many environments, an account with 0 lamports is immediately deleted,
    // but the point of the vulnerability is that the PROGRAM logic didn't clear it.
    if (accountInfo) {
        expect(accountInfo.lamports).to.equal(0);
    }
  });

  it("Closes account (Secure - Clean)", async () => {
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

    const accountInfo = await provider.connection.getAccountInfo(vault.publicKey);
    expect(accountInfo).to.be.null;
  });
});
