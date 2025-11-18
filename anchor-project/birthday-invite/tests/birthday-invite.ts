import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BirthdayInvite } from "../target/types/birthday_invite";
import { PublicKey } from '@solana/web3.js';
import { assert } from "chai";

const EVENT_SEED = "EVENT_SEED";

describe("birthday-invite", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.BirthdayInvite as Program<BirthdayInvite>;

  const creator = anchor.web3.Keypair.generate();
  const guest1 = anchor.web3.Keypair.generate();
  const guest2 = anchor.web3.Keypair.generate();

  const eventName = "Alice's 30th Birthday!";
  const eventDate_future = Math.floor(Date.now() / 1000) + 7 * 24 * 60 * 60; // 7 days
  const commentText = "Can't wait for the party!";

  // ========== TEST 1: Create Birthday Event ==========
  it("Test 1: Should successfully create a birthday event", async () => {
    await airdrop(provider.connection, creator.publicKey, 5000000000);
    const [event_pkey, _] = getEventAddress(eventName, creator.publicKey, program.programId);

    await program.methods.initializeBdayEvent(eventName, new anchor.BN(eventDate_future))
      .accounts({
        creator: creator.publicKey,
        birthdayEvent: event_pkey,
        systemProgram: anchor.web3.SystemProgram.programId
      })
      .signers([creator])
      .rpc({ commitment: "confirmed" });

    let eventData = await program.account.birthdayEvent.fetch(event_pkey);
    assert.strictEqual(eventData.eventName, eventName);
    assert.strictEqual(eventData.creator.toString(), creator.publicKey.toString());
    assert.strictEqual(eventData.comingCount, 0);
    assert.strictEqual(eventData.busyCount, 0);
  });

  // ========== TEST 2: RSVP Coming ==========
  it("Test 2: Should successfully add coming RSVP to event", async () => {
    await airdrop(provider.connection, guest1.publicKey, 5000000000);
    const [event_pkey, _] = getEventAddress(eventName, creator.publicKey, program.programId);

    await program.methods.confirmAttendance(eventName)
      .accounts({
        signer: guest1.publicKey,
        birthdayEvent: event_pkey,
      })
      .signers([guest1])
      .rpc({ commitment: "confirmed" });

    let eventData = await program.account.birthdayEvent.fetch(event_pkey);
    assert.strictEqual(eventData.comingCount, 1);
  });

  // ========== TEST 3: RSVP Toggle Coming → Busy ==========
  it("Test 3: Should toggle RSVP from coming to busy", async () => {
    const [event_pkey, _] = getEventAddress(eventName, creator.publicKey, program.programId);

    await program.methods.declineAttendance(eventName)
      .accounts({
        signer: guest1.publicKey,
        birthdayEvent: event_pkey,
      })
      .signers([guest1])
      .rpc({ commitment: "confirmed" });

    let eventData = await program.account.birthdayEvent.fetch(event_pkey);
    assert.strictEqual(eventData.comingCount, 0);
    assert.strictEqual(eventData.busyCount, 1);
  });

  // ========== TEST 4: Add Comment ==========
  it("Test 4: Should successfully add comment to event", async () => {
    await airdrop(provider.connection, guest2.publicKey, 5000000000);
    const [event_pkey, _] = getEventAddress(eventName, creator.publicKey, program.programId);

    await program.methods.addComment(eventName, commentText)
      .accounts({
        author: guest2.publicKey,
        birthdayEvent: event_pkey,
      })
      .signers([guest2])
      .rpc({ commitment: "confirmed" });

    let eventData = await program.account.birthdayEvent.fetch(event_pkey);
    assert.isAtLeast(eventData.comments.length, 1);
    assert.strictEqual(eventData.comments[0].content, commentText);
    assert.strictEqual(eventData.comments[0].commentAuthor.toString(), guest2.publicKey.toString());
  });

  // ========== TEST 5: Remove Comment ==========
  it("Test 5: Should successfully remove comment from event", async () => {
    const [event_pkey, _] = getEventAddress(eventName, creator.publicKey, program.programId);

    let eventData = await program.account.birthdayEvent.fetch(event_pkey);
    const initialCommentCount = eventData.comments.length;
    const firstCommentId = eventData.comments[0].commentId.toNumber();

    await program.methods.removeComment(eventName, new anchor.BN(firstCommentId))
      .accounts({
        author: guest2.publicKey,
        birthdayEvent: event_pkey,
      })
      .signers([guest2])
      .rpc({ commitment: "confirmed" });

    let updatedEventData = await program.account.birthdayEvent.fetch(event_pkey);
    assert.strictEqual(updatedEventData.comments.length, initialCommentCount - 1);
  });

  // ========== TEST 6: Multiple Guests RSVP ==========
  it("Test 6: Should allow multiple guests to RSVP", async () => {
    await airdrop(provider.connection, guest2.publicKey, 5000000000);
    const [event_pkey, _] = getEventAddress(eventName, creator.publicKey, program.programId);

    await program.methods.confirmAttendance(eventName)
      .accounts({
        signer: guest2.publicKey,
        birthdayEvent: event_pkey,
      })
      .signers([guest2])
      .rpc({ commitment: "confirmed" });

    let eventData = await program.account.birthdayEvent.fetch(event_pkey);
    assert.isAtLeast(eventData.comingCount, 1);
  });
});

// ========== HELPER FUNCTIONS ==========

async function airdrop(connection: any, address: any, amount = 5000000000) {
  await connection.confirmTransaction(
    await connection.requestAirdrop(address, amount), 
    "confirmed"
  );
}

function getEventAddress(eventName: string, creator: PublicKey, programID: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [
      eventName.length > 0 ? anchor.utils.bytes.utf8.encode(eventName) : new Uint8Array(32),
      anchor.utils.bytes.utf8.encode("EVENT_SEED"),
      creator.toBuffer()
    ],
    programID
  );
}