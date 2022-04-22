<script lang="ts">
import { createEventDispatcher, getContext } from 'svelte';
import '@material/mwc-button';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';

import { appWebsocketContext, appInfoContext } from '../../../contexts';
import { Note } from '../../../types/knowledge_system/notes';
import '@type-craft/content/create-content';
import '@type-craft/date-time/create-date-time';

let appInfo = getContext(appInfoContext).getAppInfo();
let appWebsocket = getContext(appWebsocketContext).getAppWebsocket();

const dispatch = createEventDispatcher();

let content: string | undefined;
let author: string | undefined;
let timestamp: number | undefined;

$: content, author, timestamp;

async function createNote() {
  const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'knowledge_system')!;

  const note: Note = {
    content: content!,
        author: author!,    // TODO: set the author
        timestamp: timestamp!,
  };

  
  const { entryHash } = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: cellData.cell_id,
    zome_name: 'notes',
    fn_name: 'create_note',
    payload: note,
    provenance: cellData.cell_id[1]
  });

  dispatch('note-created', { entryHash });
}

</script>
<div style="display: flex; flex-direction: column">
  <span style="font-size: 18px">Create Note</span>

  <create-content
      
      on:change="{e => content = e.target.value}"
      style="margin-top: 16px"
    ></create-content>

  

  <create-date-time
      
      on:change="{e => timestamp = e.target.value}"
      style="margin-top: 16px"
    ></create-date-time>

  <mwc-button 
    label="Create Note"
    disabled={!(content && author && timestamp)}
    on:click="{() => createNote()}"
  ></mwc-button>
</div>
