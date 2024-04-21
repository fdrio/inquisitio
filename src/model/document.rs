use super::tokenizer::Tokenizer;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::Instant;
use uuid::Uuid;

#[derive(Debug)]
pub enum DocumentState {
    Pending,
    Indexing,
    Indexed,
    Failed,
}

type TF = HashMap<String, usize>;

#[derive(Debug)]
pub struct Document {
    doc_id: Uuid,
    created_on: Instant,
    last_updated_on: Instant,
    state: DocumentState,
    pub tf: TF,
    tf_total: usize,
}

impl Document {
    fn new_tf(path_buf: &PathBuf) -> anyhow::Result<(TF, usize)> {
        let mut tf = HashMap::new();
        let mut count = 0;
        if path_buf.is_file() {
            let file = File::open(path_buf.as_path())?;
            let reader = io::BufReader::new(file);
            for token in Tokenizer::tokenize(reader)? {
                match tf.get_mut(&token) {
                    Some(v) => {
                        *v += 1;
                    }
                    None => {
                        tf.insert(token, 1);
                    }
                }
                count += 1;
            }
        }
        Ok((tf, count))
    }

    pub fn compute_tf(&self, token: String) -> f64 {
        let res = self.tf.get(&token);
        if let Some(tf_count) = res {
            let tf_count = *tf_count as f64;
            return tf_count / self.tf_total as f64;
        }
        0.0
    }

    pub fn new(path_buf: &PathBuf) -> anyhow::Result<Self> {
        let doc_id = Uuid::new_v4();
        let (doc_tf, tf_total) = Document::new_tf(&path_buf)?;
        let doc = Document {
            doc_id: doc_id,
            created_on: Instant::now(),
            last_updated_on: Instant::now(),
            state: DocumentState::Indexed,
            tf: doc_tf,
            tf_total: tf_total,
        };
        Ok(doc)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_new_tf() -> anyhow::Result<()> {
        let path = std::fs::read_dir("./docs")?;
        for file in path {
            let file = file?;
            let tf = Document::new_tf(&file.path());
            assert!(
                tf.is_ok(),
                "Document::new_tf returned an error: {:?}",
                tf.err()
            );
        }
        Ok(())
    }

    #[test]
    fn test_create_doc() -> anyhow::Result<()> {
        let path = std::fs::read_dir("./docs")?;
        for file in path {
            let file = file?;
            let doc = Document::new(&file.path());
            assert!(
                doc.is_ok(),
                "Document::new returned an error: {:?}",
                doc.err()
            );
        }
        Ok(())
    }
}
