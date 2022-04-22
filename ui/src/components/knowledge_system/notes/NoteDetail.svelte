<script lang="ts">
import { onMount, getContext } from 'svelte';
import '@material/mwc-circular-progress';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { appInfoContext, appWebsocketContext } from '../../../contexts';
import { Note } from '../../../types/knowledge_system/notes';
import '@type-craft/content/content-detail';
import '@holochain-open-dev/utils/copiable-hash';
import '@type-craft/date-time/date-time-detail';

export let entryHash: string;

let appInfo = getContext(appInfoContext).getAppInfo();
let appWebsocket = getContext(appWebsocketContext).getAppWebsocket();

let note: Note | undefined;

$: note;

onMount(async () => {
  const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === 'knowledge_system')!;

  note = await appWebsocket.callZome({
    cap_secret: null,
    cell_id: cellData.cell_id,
    zome_name: 'notes',
    fn_name: 'get_note',
    payload: entryHash,
    provenance: cellData.cell_id[1]
  });
});
</script>

{#if note}
  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Note</span>

    
    <content-detail
    
      value={note.content}
      style="margin-top: 16px"
    ></content-detail>

    
    <copiable-hash
    
      value={note.author}
      style="margin-top: 16px"
    ></copiable-hash>

    
    <date-time-detail
    
      value={note.timestamp}
      style="margin-top: 16px"
    ></date-time-detail>

  </div>
{:else}
  <div style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
{/if}
