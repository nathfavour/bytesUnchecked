import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnTypeConfusion } from "../target/types/vuln_type_confusion";
import { expect } from "chai";

describe("vuln_type_confusion", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnTypeConfusion as Program<VulnTypeConfusion>;
  const provider = anchor.getProvider();

  it("Exploits type confusion (User as Admin)", async () => {
    const user = anchor.web3.Keypair.generate();

    // Initialize as a regular User
    await program.methods
      .initializeUser()
      .accounts({
        user: user.publicKey,
        authority: provider.publicKey,
      })
      .signers([user])
      .rpc();

    // Perform admin action using the User account
    await program.methods
      .adminActionInsecure()
      .accounts({
        admin: user.publicKey,
        authority: provider.publicKey,
      })
      .rpc();
    
    // It succeeds because there's no discriminator check in the insecure method!
  });

  it("Fails when type is checked (Secure)", async () => {
    const user = anchor.web3.Keypair.generate();

    await program.methods
      .initializeUser()
      .accounts({
        user: user.publicKey,
        authority: provider.publicKey,
      })
      .signers([user])
      .rpc();

    try {
      await program.methods
        .adminActionSecure()
        .accounts({
          admin: user.publicKey,
          authority: provider.publicKey,
        })
        .rpc();
      expect.fail("Should have failed due to discriminator mismatch");
    } catch (e: any) {
      // Anchor will throw a discriminator mismatch error (0xbc4 or 3004)
      expect(e.code).to.equal(3004); 
    }
  });
});
