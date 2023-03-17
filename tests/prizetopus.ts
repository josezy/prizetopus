import * as anchor from "@project-serum/anchor";
import { Program, web3 } from "@project-serum/anchor";
import { Prizetopus } from "../target/types/prizetopus";

export const PRIZETOPUS_PROG_ID = new web3.PublicKey(
  'Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS'
);

describe("prizetopus", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Prizetopus as Program<Prizetopus>;

  const tournamentAuthority = anchor.web3.Keypair.generate();

  const payer = anchor.web3.Keypair.generate();
  const tournament = anchor.web3.Keypair.generate();

  const [prizePool, prizePoolBump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("virtue_poker"), tournament.publicKey.toBuffer(), Buffer.from("prize_pool")],
    PRIZETOPUS_PROG_ID
  )
  const [prizeDistribution, prizeDistributionBump] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("virtue_poker"), tournament.publicKey.toBuffer(), Buffer.from("prize_distribution")],
    PRIZETOPUS_PROG_ID
  )

  it("Test init tournament", async () => {
    const tx = await program.methods.initTournament(
      prizePoolBump,
      prizeDistributionBump,
      new anchor.BN(1000),
      new anchor.BN(9),
      new anchor.BN(60 * 10)
    ).accounts({
      tournament: tournament.publicKey,
      tournamentAuthority: tournamentAuthority.publicKey,
      payer: payer.publicKey,
      prizePool,
      prizeDistribution,
      systemProgram: web3.SystemProgram.programId,
      rent: web3.SYSVAR_RENT_PUBKEY,
    }).signers([
      payer,
      tournament,
    ]).rpc();

    console.log("Your transaction signature", tx);
  })
});
