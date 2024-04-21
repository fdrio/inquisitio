use super::document::Document;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::DirEntry;
use std::path::PathBuf;
use std::time::Instant;

pub enum IndexState {
    Initializing,
    Updating,
    Ready,
    Error,
}

type IDF = HashMap<String, usize>;
type Documents = HashMap<PathBuf, Document>;

#[derive(Debug)]
pub struct Index {
    index_id: uuid::Uuid,
    index_name: String,
    idf: IDF,
    documents: Documents,
}

impl Index {
    fn index(file_path: &PathBuf) -> anyhow::Result<Document> {
        Document::new(&file_path)
    }

    pub fn new(dir_str: &str, index_name: &str) -> anyhow::Result<Self> {
        let path = std::fs::read_dir(dir_str)?;
        let mut documents = HashMap::new();
        for file in path {
            let file_path = file?.path();
            let doc = Self::index(&file_path)?;
            documents.insert(file_path, doc);
        }
        let idf = Self::new_idf(&documents);
        let index = Self {
            index_id: uuid::Uuid::new_v4(),
            index_name: index_name.to_string(),
            documents: documents,
            idf: idf,
        };
        Ok(index)
    }

    fn new_idf(documents: &Documents) -> IDF {
        let mut idf = IDF::new();
        for (_, doc) in documents {
            for token in doc.tf.keys() {
                match idf.get_mut(token) {
                    Some(count) => {
                        *count += 1;
                    },
                    None => {
                        idf.insert(token.clone(), 1);
                    }
                }
            }
        }
        idf
    }
    pub fn compute_idf(&self, token: String) -> f64 {
        let total = self.documents.len() as f64;
        let doc_count = self.idf.get(&token);
        if let Some(doc_count) = doc_count {
            let doc_count = *doc_count as f64;
            let probability = (1.0 + doc_count) / (1.0 + total);
            let entropy = (1.0 / probability).log2();
            return entropy;
        }
        0.0
    }


    pub fn compute_tf_idf(&self, token: String)->f64{
        todo!("Need to finish this")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_create_index() {
        let index = Index::new("./docs", "test-index");
        assert!(
            index.is_ok(),
            "Index::new returned an error: {:?}",
            index.err()
        );
    }
}
