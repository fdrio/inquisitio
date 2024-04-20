use anyhow;
use regex;
use std::fs::File;
use std::io::Cursor;
use std::io::{self, BufRead};

pub struct Tokenizer {}

impl Tokenizer {
    pub fn tokenize<R: BufRead>(reader: R) -> anyhow::Result<Vec<String>> {
        let mut collector = Vec::<String>::new();
        let rgx = regex::Regex::new(r"[^a-zA-Z]+")?;
        for line in reader.lines() {
            let line = line?;
            for token in rgx.split(line.as_str()) {
                if !token.is_empty() {
                    collector.push(token.to_string());
                }
            }
        }
        Ok(collector)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_tokenizer() -> anyhow::Result<()> {
        let input = "Test; testing!\nThis is another sentence; with tokens.";
        let cursor = Cursor::new(input);
        let tokens = Tokenizer::tokenize(cursor)?;
        let expected_tokens = vec![
            "Test", "testing", "This", "is", "another", "sentence", "with", "tokens",
        ];
        assert_eq!(tokens, expected_tokens);
        Ok(())
    }
}
