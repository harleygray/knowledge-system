



export interface NewNoteInput {
  content: string;
  tags: Array<string>;
}

export interface Note {
  content: string;
  author: string;
  timestamp: number;
}