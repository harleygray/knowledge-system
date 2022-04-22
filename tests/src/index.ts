import { Orchestrator } from "@holochain/tryorama";

import profiles_entry_def_0 from './knowledge_system/profiles/entry_def_0';
import notes_note from './knowledge_system/notes/note';

let orchestrator: Orchestrator<any>;

orchestrator = new Orchestrator();
profiles_entry_def_0(orchestrator);
orchestrator.run();

orchestrator = new Orchestrator();
notes_note(orchestrator);
orchestrator.run();



