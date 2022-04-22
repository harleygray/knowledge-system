use hdk::prelude::holo_hash::AgentPubKeyB64;
use hdk::prelude::*;


#[hdk_entry(id = "note")]
#[serde(rename_all = "camelCase")]
#[derive(Clone)]
pub struct Note {
  pub content: String,
  pub author: AgentPubKeyB64,
  pub timestamp: Timestamp,
}