import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnPdaSeedLeak } from "../target/types/vuln_pda_seed_leak";
import { expect } from "chai";
import { createHash } from "crypto";

describe("vuln_pda_seed_leak", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnPdaSeedLeak as Program<VulnPdaSeedLeak>;
  const provider = anchor.getProvider();

  it("Initializes insecure PDA with raw sensitive seeds", async () => {
    const sensitiveId = "user-ssn-123-456";
    
    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), Buffer.from(sensitiveId)],
      program.programId
    );

    await program.methods
      .initializeInsecure(sensitiveId)
      .accounts({
        profile: profilePda,
        user: provider.publicKey,
      })
      .rpc();

    const account = await program.account.userProfile.fetch(profilePda);
    expect(account.sensitiveId).to.equal(sensitiveId);
  });

  it("Initializes secure PDA with hashed sensitive seeds", async () => {
    const sensitiveId = "user-ssn-789-012";
    
    // Hash the seed as the program does
    const hashedSeed = createHash("sha256").update(sensitiveId).digest();

    const [profilePda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("profile"), hashedSeed],
      program.programId
    );

    await program.methods
      .initializeSecure(sensitiveId)
      .accounts({
        profile: profilePda,
        user: provider.publicKey,
      })
      .rpc();

    const account = await program.account.userProfile.fetch(profilePda);
    expect(account.sensitiveId).to.equal(sensitiveId);
  });
});
