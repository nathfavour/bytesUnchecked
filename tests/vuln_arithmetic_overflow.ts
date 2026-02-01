import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VulnArithmeticOverflow } from "../target/types/vuln_arithmetic_overflow";
import { expect } from "chai";

describe("vuln_arithmetic_overflow", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const program = anchor.workspace.VulnArithmeticOverflow as Program<VulnArithmeticOverflow>;
  const provider = anchor.getProvider();

  it("Exploits arithmetic overflow (Insecure)", async () => {
    const vault = anchor.web3.Keypair.generate();
    // Initial balance near max u64
    const initialBalance = new anchor.BN("ffffffffffffffff", 16);

    await program.methods
      .initialize(initialBalance)
      .accounts({
        vault: vault.publicKey,
        user: provider.publicKey,
      })
      .signers([vault])
      .rpc();

    // Adding 1 to max u64 will wrap to 0 in our insecure implementation
    await program.methods
      .depositInsecure(new anchor.BN(1))
      .accounts({
        vault: vault.publicKey,
      })
      .rpc();

    const account = await program.account.vault.fetch(vault.publicKey);
    expect(account.balance.toNumber()).to.equal(0);
  });

  it("Fails on arithmetic overflow (Secure)", async () => {
    const vault = anchor.web3.Keypair.generate();
    const initialBalance = new anchor.BN("ffffffffffffffff", 16);

    await program.methods
      .initialize(initialBalance)
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
      expect.fail("Should have thrown an overflow error");
    } catch (err) {
      expect(err.toString()).to.include("Overflow");
    }
  });
});
