import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Prizetopus } from "../target/types/prizetopus";

describe("prizetopus", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Prizetopus as Program<Prizetopus>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
