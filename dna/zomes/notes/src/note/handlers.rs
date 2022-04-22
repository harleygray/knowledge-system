use std::collections::BTreeMap;

use hdk::prelude::*;
use hdk::prelude::holo_hash::*;
use super::Note;
use super::utils::*;

const ALL_TAGS: &str = "ALL_TAGS";

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



#[hdk_extern]
pub fn get_notes_for_author(author: AgentPubKeyB64) -> ExternResult<Vec<Note>> {

  let links = get_links(AgentPubKey::from(author).into(), Some(LinkTag::new(String::from("PUBLISHED"))))?;

  let mut notes = Vec::new();

  for link in links {
    let note: Note = try_get_and_convert(link.target)?;
    notes.push(note);
  }

  Ok(notes)
}


#[hdk_extern]
pub fn get_notes_for_tag(tag: String) -> ExternResult<Vec<Note>> {

  let anchor = anchor(ALL_TAGS.into(), ALL_TAGS.into())?;
  let links = get_links(anchor, Some(LinkTag::new(tag)))?;

  let mut notes = Vec::new();

  for link in links {
    let note: Note = try_get_and_convert(link.target)?;
    notes.push(note);
  }

  Ok(notes)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddTagInput {
  entry_hash: EntryHashB64,
  tag: String,
}

#[hdk_extern]
fn add_tag_to_note(input: AddTagInput) -> ExternResult<()> {

  let anchor = anchor(ALL_TAGS.into(), ALL_TAGS.into())?;
  create_link(anchor, input.entry_hash.into(), LinkTag::new(input.tag))?;

  Ok(())
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ReferenceType {
  Quote,
  Agree,
  Disagree,
  Other,
}



#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LinkNotesInput {
  link_from: EntryHashB64,
  link_to: EntryHashB64,
  reference_type: ReferenceType,
}


#[hdk_extern]
pub fn link_notes(input: LinkNotesInput) -> ExternResult<()> {

  let link_tag = match input.reference_type.clone() {
    ReferenceType::Quote => LinkTag::new(String::from("forward.QUOTE")),
    ReferenceType::Agree => LinkTag::new(String::from("forward.AGREE")),
    ReferenceType::Disagree => LinkTag::new(String::from("forward.DISAGREE")),
    ReferenceType::Other => LinkTag::new(String::from("forward.OTHER")),
  };

  let backlink_tag = match input.reference_type {
    ReferenceType::Quote => LinkTag::new(String::from("backward.QUOTE")),
    ReferenceType::Agree => LinkTag::new(String::from("backward.AGREE")),
    ReferenceType::Disagree => LinkTag::new(String::from("backward.DISAGREE")),
    ReferenceType::Other => LinkTag::new(String::from("backward.OTHER")),
  };

  create_link(input.link_from.clone().into(), input.link_to.clone().into(), link_tag)?;
  create_link(input.link_to.into(), input.link_from.into(), backlink_tag)?;


  Ok(())
}


#[hdk_extern]
pub fn get_forward_links(entry_hash: EntryHashB64) -> ExternResult<BTreeMap<String, Vec<Note>>> {

  let ref_types: Vec<String> = vec!["QUOTE".into(), "AGREE".into(), "DISAGREE".into(), "OTHER".into()];

  let mut all_notes: BTreeMap<String, Vec<Note>> = BTreeMap::new();

  for ref_type in ref_types {
    let mut notes = Vec::new();
    let links = get_links(entry_hash.clone().into(), Some(LinkTag::new(format!("forward.{}", ref_type))))?;
    for link in links {
      let note: Note = try_get_and_convert(link.target)?;
      notes.push(note);
    }
    all_notes.insert(ref_type, notes);
  };

  /*

  {
    "QUOTE": [note1, note2],
    "AGREE": [note3, note 4],
    ...
  }

  */

  Ok(all_notes)
}

#[hdk_extern]
pub fn get_backward_links(entry_hash: EntryHashB64) -> ExternResult<BTreeMap<String, Vec<Note>>> {

  let ref_types: Vec<String> = vec!["QUOTE".into(), "AGREE".into(), "DISAGREE".into(), "OTHER".into()];

  let mut all_notes: BTreeMap<String, Vec<Note>> = BTreeMap::new();

  for ref_type in ref_types {
    let mut notes = Vec::new();
    let links = get_links(entry_hash.clone().into(), Some(LinkTag::new(format!("backward.{}", ref_type))))?;
    for link in links {
      let note: Note = try_get_and_convert(link.target)?;
      notes.push(note);
    }
    all_notes.insert(ref_type, notes);
  };

  /*

  {
    "QUOTE": [note1, note2],
    "AGREE": [note3, note 4],
    ...
  }

  */

  Ok(all_notes)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewNoteOutput {
  header_hash: HeaderHashB64,
  entry_hash: EntryHashB64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewNoteInput {
  content: String,
  tags: Vec<String>,
}

#[hdk_extern]
pub fn create_note(input: NewNoteInput) -> ExternResult<NewNoteOutput> {

  let author = agent_info()?.agent_initial_pubkey;
  let timestamp = sys_time()?;
  let tags_anchor_hash = anchor(ALL_TAGS.into(),ALL_TAGS.into())?;

  let note = Note {
    content: input.content,
    author: author.clone().into(),
    timestamp,
  };

  let header_hash = create_entry(&note)?;

  let entry_hash = hash_entry(&note)?;

  //add tags
  for tag in input.tags {
    create_link(tags_anchor_hash.clone(), entry_hash.clone(), LinkTag::new(tag))?;
  }

  create_link(author.into(), entry_hash.clone(), LinkTag::new(String::from("PUBLISHED")))?;

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

