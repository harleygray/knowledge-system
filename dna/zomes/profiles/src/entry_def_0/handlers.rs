use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::EntryDef0;

#[hdk_extern]
pub fn get_entry_def_0(entry_hash: EntryHashB64) -> ExternResult<Option<EntryDef0>> {
  let maybe_element = get(EntryHash::from(entry_hash), GetOptions::default())?;

  match maybe_element {
    None => Ok(None),
    Some(element) => {
      let entry_def_0: EntryDef0 = element.entry()
        .to_app_option()?
        .ok_or(WasmError::Guest("Could not deserialize element to EntryDef0.".into()))?;
    
      Ok(Some(entry_def_0))
    }
  }
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewEntryDef0Output {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}

#[hdk_extern]
pub fn create_entry_def_0(entry_def_0: EntryDef0) -> ExternResult<NewEntryDef0Output> {
  let header_hash = create_entry(&entry_def_0)?;

  let entry_hash = hash_entry(&entry_def_0)?;

  let output = NewEntryDef0Output {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateEntryDef0Input {
  original_header_hash: HeaderHashB64,
  updated_entry_def_0: EntryDef0
}

#[hdk_extern]
pub fn update_entry_def_0(input: UpdateEntryDef0Input) -> ExternResult<NewEntryDef0Output> {
  let header_hash = update_entry(HeaderHash::from(input.original_header_hash), &input.updated_entry_def_0)?;

  let entry_hash = hash_entry(&input.updated_entry_def_0)?;

  let output = NewEntryDef0Output {
    header_hash: HeaderHashB64::from(header_hash),
    entry_hash: EntryHashB64::from(entry_hash)
  };

  Ok(output)
}


#[hdk_extern]
pub fn delete_entry_def_0(header_hash: HeaderHashB64) -> ExternResult<HeaderHash> {
  delete_entry(HeaderHash::from(header_hash))
}

