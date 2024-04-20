use super::tokenizer::Tokenizer;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::time::Instant;
use std::env;
use uuid::Uuid;

#[derive(Debug)]
pub enum DocumentState {
    Pending,
    Indexing,
    Indexed,
    Failed,
}

#[derive(Debug)]
pub struct Document {
    doc_id: Uuid,
    created_on: Instant,
    last_updated_on: Instant,
    state: DocumentState,
    tf: HashMap<String, usize>,
}

impl Document {
    fn compute_tf(path_buf: PathBuf) -> anyhow::Result<HashMap<String, usize>> {
        let mut tf = HashMap::new();
        if path_buf.is_file() {
            let file = File::open(path_buf.as_path())?;
            let reader = io::BufReader::new(file);
            for token in Tokenizer::tokenize(reader)?{
                match tf.get_mut(&token){
                    Some(v)=>{
                        *v +=1;
                    },
                    None=>{
                        tf.insert(token, 1);
                    },
                }
            }
        }

        Ok(tf)

    }

    pub fn new(path_buf: PathBuf) -> anyhow::Result<Document> {
        let doc_id = Uuid::new_v4();        
        let state = DocumentState::Pending;
        let doc_tf = Document::compute_tf(path_buf)?;
        let doc = Document{
            doc_id: doc_id,
            created_on: Instant::now(),
            last_updated_on: Instant::now(),
            state: DocumentState::Indexed,
            tf: doc_tf,
        }; 
        Ok(doc) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compute_tf() -> anyhow::Result<()> {
        let mut path = fs::read_dir("./docs") ;
        let mut path = path?;
        for file in path{
            let file = file?;
            let tf = Document::compute_tf(file.path());
            assert!(tf.is_ok(), "Document::compute_tf returned an error: {:?}", tf.err());
        }
        Ok(())
    }

    #[test]
    fn test_create_doc() -> anyhow::Result<()> {
        let mut path = fs::read_dir("./docs") ;
        let mut path = path?;
        for file in path{
            let file = file?;
            let doc = Document::new(file.path());
            assert!(doc.is_ok(), "Document::compute_tf returned an error: {:?}", doc.err());
        }
        Ok(())
    }
}
