import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { expect } from "chai";

describe("voting-app", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  const program = anchor.workspace.VotingApp as Program<any>;

  let proposalPda: anchor.web3.PublicKey;
  const creator = provider.wallet.publicKey;

  it("Crée une proposition", async () => {
    const title = "Mon sondage";
    const description = "Choisissez une option";
    const choices = ["A", "B", "C"];
    const [pda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("proposal"), creator.toBuffer(), Buffer.from(title)],
      program.programId
    );
    proposalPda = pda;

    const now = Math.floor(Date.now() / 1000);
    const start = now;
    const end = now + 60 * 60; // +1h

    await program.rpc.createProposal(
      description,
      title,
      choices,
      new anchor.BN(start),
      new anchor.BN(end),
      {
        accounts: {
          proposal: proposalPda,
          creator,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [],
        instructions: [],
      }
    );

    const proposal = (await program.account.proposal.fetch(proposalPda)) as any;
    expect(proposal.title).to.equal(title);
    expect(proposal.description).to.equal(description);
    expect((proposal.choices as any[]).length).to.equal(3);
  });

  it("Permet de voter puis compte correctement", async () => {
    const voter = anchor.web3.Keypair.generate();
    await provider.connection.requestAirdrop(voter.publicKey, 1e9);

    const [votePda] = await anchor.web3.PublicKey.findProgramAddress(
      [Buffer.from("vote"), proposalPda.toBuffer(), voter.publicKey.toBuffer()],
      program.programId
    );

    await program.rpc.castVote(
      0,
      {
        accounts: {
          proposal: proposalPda,
          vote: votePda,
          voter: voter.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        },
        signers: [voter],
      }
    );

    const proposal = (await program.account.proposal.fetch(proposalPda)) as any;
    expect((proposal.choices as any[])[0].count.toNumber()).to.equal(1);
  });

  it("Empêche le vote en dehors de la période", async () => {
    // Avance l'horloge de 2 heures (cast en any)
    await (provider.connection as any).setBlockTime(
      Math.floor(Date.now() / 1000) + 2 * 60 * 60
    );

    let caught = false;
    try {
      await program.rpc.castVote(
        1,
        {
          accounts: {
            proposal: proposalPda,
            vote: anchor.web3.Keypair.generate().publicKey,
            voter: provider.wallet.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
          },
        }
      );
    } catch (err: any) {
      caught = true;
      expect(err.toString()).to.match(/Voting period has ended/);
    }
    expect(caught).to.be.true;
  });

  it("Supprime la proposition après 1 mois", async () => {
    const oneMonth = 30 * 24 * 60 * 60;
    await (provider.connection as any).setBlockTime(
      Math.floor(Date.now() / 1000) + oneMonth + 10
    );

    await program.rpc.deleteProposal({
      accounts: {
        proposal: proposalPda,
        creator,
      },
    });

    let fetchError = false;
    try {
      await program.account.proposal.fetch(proposalPda);
    } catch {
      fetchError = true;
    }
    expect(fetchError).to.be.true;
  });
});