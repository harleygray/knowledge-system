use hdk::prelude::*;

mod note;

use note::Note;

entry_defs![
  Note::entry_def(),
  PathEntry::entry_def()
];