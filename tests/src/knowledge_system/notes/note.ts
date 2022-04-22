
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  
  orchestrator.registerScenario("note CRUD tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([config, config]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find(cell => cell.cellRole.includes('/knowledge_system.dna')) as Cell;
    const bob = bob_happ.cells.find(cell => cell.cellRole.includes('/knowledge_system.dna')) as Cell;

    const entryContents = {
  "content": "I, uh, don't think I'm, y'know, so different than your average, y'know, average. My dad once told me, laugh and the world laughs with you, Cry, and I'll give you something to cry about you little bastard! I, uh, don't think I'm, y'know, so different than your average, y'know, average.",
  "author": "uhCAkstOzoZxfJWBCPCsekEMfkjFjC9G0afJYurpL6zSwhUu16nnJ",
  "timestamp": 1650611914346
};

    // Alice creates a note
    const create_output = await alice.call(
        "notes",
        "create_note",
        entryContents
    );
    t.ok(create_output.headerHash);
    t.ok(create_output.entryHash);

    await sleep(200);
    
    // Bob gets the created note
    const entry = await bob.call("notes", "get_note", create_output.entryHash);
    t.deepEqual(entry, entryContents);
    
    
    // Alice updates the note
    const update_output = await alice.call(
      "notes",
      "update_note",
      {
        originalHeaderHash: create_output.headerHash,
        updatedNote: {
          "content": "It's mysterious what attracts you to a person. It must mean my character is interesting in some way. It's nice to play a character that has a soulful, dependent, close relationship.",
  "author": "uhCAkbH3qC2cwqHehOO2JsBDYwQcHrISn_95rv40fxJYtnuiJz8w5",
  "timestamp": 1650611914346
}
      }
    );
    t.ok(update_output.headerHash);
    t.ok(update_output.entryHash);
    await sleep(200);

      
    
    // Alice delete the note
    await alice.call(
      "notes",
      "delete_note",
      create_output.headerHash
    );
    await sleep(200);

    
    // Bob tries to get the deleted note, but he doesn't get it because it has been deleted
    const deletedEntry = await bob.call("notes", "get_note", create_output.entryHash);
    t.notOk(deletedEntry);
      
  });

}
