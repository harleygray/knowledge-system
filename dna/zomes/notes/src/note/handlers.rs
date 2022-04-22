use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::Note;

#[hdk_extern]
pub fn get_note(entry_hash: EntryHashB64) -> ExternResult<Option<Note>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let note: Note = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to Note.".into()))?;
    
      Ok(Some(note))
    }
  }
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewNoteOutput {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}

#[hdk_extern]
pub fn create_note(note: Note) -> ExternResult<NewNoteOutput> {
  let header_hash = create_entry(&note)?;

  let entry_hash = hash_entry(&note)?;

  let output = NewNoteOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateNoteInput {
  original_header_hash: HeaderHashB64,
  updated_note: Note
}

#[hdk_extern]
pub fn update_note(input: UpdateNoteInput) -> ExternResult<NewNoteOutput> {
  let header_hash = update_entry(HeaderHash::from(input.original_header_hash), &input.updated_note)?;

  let entry_hash = hash_entry(&input.updated_note)?;

  let output = NewNoteOutput {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[hdk_extern]
pub fn delete_note(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
  delete_entry(HeaderHash::from(header_hash))
}

