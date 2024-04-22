use super::tokenizer::Tokenizer;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::time::Instant;
use std::cmp::Ordering;
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
    pub id: Uuid,
    pub name: String,
    pub created_on: Instant,
    pub last_updated_on: Instant,
    pub state: DocumentState,
    pub tf: TF,
    pub tf_total: usize,
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

    pub fn new(path_buf: &PathBuf) -> anyhow::Result<Self> {
        let doc_id = Uuid::new_v4();
        let (doc_tf, tf_total) = Document::new_tf(&path_buf)?;
        if let Some(file_name) = path_buf.file_name().map(|name| name.to_string_lossy().to_string()){
            let doc = Document {
                id: doc_id,
                name: file_name, 
                created_on: Instant::now(),
                last_updated_on: Instant::now(),
                state: DocumentState::Indexed,
                tf: doc_tf,
                tf_total: tf_total,
            };
            return Ok(doc);
        }
        
        Err(anyhow::format_err!("Error: Could not create document: {doc_id}"))
    }

    pub fn compute_tf(&self, token: &String) -> f64 {
        let res = self.tf.get(token);
        if let Some(tf_count) = res {
            let tf_count = *tf_count as f64;
            return tf_count / self.tf_total as f64;
        }
        0.0
    }

}


#[derive(Debug)]
pub struct ScoreDoc<T> {
    priority: f64,
    pub value: T,
}

impl<T> ScoreDoc<T>{
    pub fn new(priority:f64 ,value: T)->ScoreDoc<T>{
        ScoreDoc{
            priority:priority,
            value:value,
        }
    }
}

impl<T> PartialEq for ScoreDoc<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<T> Eq for ScoreDoc<T> {}

impl<T> PartialOrd for ScoreDoc<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for ScoreDoc<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering based on priority
        other.priority.partial_cmp(&self.priority).unwrap()
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
