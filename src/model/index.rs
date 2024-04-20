use super::document::Document;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Instant;

pub enum IndexState {
    Initializing,
    Updating,
    Ready,
    Error,
}

pub struct Index{
    index_id: String,
    count: usize,
    document_frequency: HashMap<String, usize>,
    documents: HashMap<PathBuf, Document>,
}
