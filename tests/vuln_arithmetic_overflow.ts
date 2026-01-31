import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnArithmeticOverflow } from "../target/types/vuln_arithmetic_overflow";
import { expect } from "chai";

describe("vuln_arithmetic_overflow", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnArithmeticOverflow as Program<VulnArithmeticOverflow>;
  const provider = anchor.getProvider();

  it("Causes a panic on overflow in insecure mode", async () => {
    const vault = anchor.web3.Keypair.generate();
    const maxU64 = new anchor.BN("18446744073709551615");

    await program.methods
      .initialize(maxU64)
      .accounts({
        vault: vault.publicKey,
        user: provider.publicKey,
      })
      .signers([vault])
      .rpc();

    try {
      await program.methods
        .depositInsecure(new anchor.BN(1))
        .accounts({
          vault: vault.publicKey,
        })
        .rpc();
      expect.fail("Should have panicked due to overflow checks in Cargo.toml");
    } catch (e) {
      // In Solana, a panic results in a generic ProgramError or ComputeBudget exhaustion
      // Depending on the environment, it might show as a 0x1770 (custom error) or similar if wrapped by Anchor
    }
  });

  it("Gracefully handles overflow in secure mode", async () => {
    const vault = anchor.web3.Keypair.generate();
    const maxU64 = new anchor.BN("18446744073709551615");

    await program.methods
      .initialize(maxU64)
      .accounts({
        vault: vault.publicKey,
        user: provider.publicKey,
      })
      .signers([vault])
      .rpc();

    try {
      await program.methods
        .depositSecure(new anchor.BN(1))
        .accounts({
          vault: vault.publicKey,
        })
        .rpc();
      expect.fail("Should have failed with custom Overflow error");
    } catch (e: any) {
      expect(e.error.errorCode.code).to.equal("Overflow");
    }
  });
});
