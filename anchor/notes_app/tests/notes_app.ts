import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import { NotesApp } from "../target/types/notes_app";
import { expect } from "chai";

describe("notes_app", () => {
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);
  const program = anchor.workspace.NotesApp as Program<NotesApp>;

  const signer = provider.wallet.publicKey;

  it("Should create a note", async () => {
    const noteId = "this";
    const noteText = "hello world";

    const [notePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("note"), signer.toBuffer(), Buffer.from(noteId)],
      program.programId
    );

    await program.methods
      .createNote(noteId, noteText)
      .accounts({
        notePda: notePda,
        signer: signer,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    const account = await program.account.note.fetch(notePda);
    expect(account.note).to.equal(noteText);
  });

  it("Should update the old note", async () => {
    const noteId = "this";
    const noteText = "this is the new text";

    const [notePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("note"), signer.toBuffer(), Buffer.from(noteId)],
      program.programId
    );

    await program.methods
      .updateNote(noteId, noteText)
      .accounts({ notePda, signer, systemProgram: SystemProgram.programId })
      .rpc();

    const account = await program.account.note.fetch(notePda);
    expect(account.note).to.equal(noteText);
  });

  it("Should fail for long notes", async () => {
    const noteId = "note-2";
    const longNote = "x".repeat(100);

    const [notePda] = PublicKey.findProgramAddressSync(
      [Buffer.from("note"), signer.toBuffer(), Buffer.from(noteId)],
      program.programId
    );

    try {
      await program.methods
        .createNote(noteId, longNote)
        .accounts({
          notePda,
          signer,
          systemProgram: SystemProgram.programId,
        })
        .rpc();

      throw new Error("Expected error but transaction succeeded");
    } catch (err) {
      expect(err.error.errorCode.code).to.equal("NoteTooLong");
    }
  });
});
