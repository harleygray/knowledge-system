use hdk::prelude::*;





#[hdk_entry(id = "entry_def_0")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct EntryDef0 {
  pub title: String,
  pub content: String,
}