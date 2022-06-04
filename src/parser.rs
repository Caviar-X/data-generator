use rand::prelude::*;
use std::fs::*;
#[derive(Clone, Debug)]
pub struct Parser {
    pub file: Option<String>,
    position: usize,
    context: String,
}
impl Parser {
    pub fn new(file: String) -> Parser {
        Parser {
            file: Some(file.clone()),
            position: 0,
            context: read_to_string(file).expect("Failed to read file"),
        }
    }
    pub fn new_with_context(context: String) -> Parser {
        Parser {
            file: None,
            position: 0,
            context,
        }
    }
    pub fn next_token(&mut self) -> Option<Vec<&str>> {
        if self.position == self.context.len() {
            return None;
        }
        let substr: &str = self
            .context
            .get(self.position..self.context.len())
            .expect("Failed to get token");
        let (l, r) = (
            substr.find('{').expect("Failed to get token") + 1,
            substr.find('}').expect("Failed to get token"),
        );
        self.position += (r - l) + 2;
        Some(
            substr
                .get(l..r)
                .expect("Failed to get token")
                .split_whitespace()
                .collect::<Vec<&str>>(),
        )
    }
    fn parse(&self, tokens: Vec<&str>) -> Option<String> {
        if tokens.len() != 1 {
            return self.parse_statements(tokens);
        }
        match tokens[0] {
            "short" | "i16" => random::<i16>().to_string(),
            "int" | "i32" => random::<i32>().to_string(),
            "long long" | "i64" => random::<i64>().to_string(),
            "int128_t" | "i128" => random::<i128>().to_string(),
            "unsigned short" | "u16" => random::<u16>().to_string(),
            "unsigned int" | "u32" => random::<u32>().to_string(),
            "unsigned long long" | "u64" => Some(random::<u64>().to_string()),
            "uint128_t" | "u128" => Some(random::<u128>().to_string()),
            _ => {
                None
            }
        }
    }
    fn parse_statements(&self, tokens: Vec<&str>) -> String {
        todo!()
    }
}
