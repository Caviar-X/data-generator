use anyhow::*;
use colorful::*;
use rand::prelude::*;
use std::fs::*;
use std::iter::repeat_with;
#[derive(Clone, Debug)]
pub struct Parser {
    pub file: Option<String>,
    pub position: usize,
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
        if self.position == (self.context.len() - 1) {
            return None;
        }
        let substr: &str = self
            .context
            .get(self.position..)
            .expect("Failed to get token");
        let (l, r) = (
            substr.find('{').expect("Failed to get token") + 1,
            substr.find('}').expect("Failed to get token"),
        );
        self.position += (r - l) + 2; //plus {}
        Some(
            substr
                .get((l)..(r))
                .expect("Failed to get token")
                .split_whitespace()
                .collect::<Vec<&str>>(),
        )
    }
    fn is_vailid_token(&self, token: &str) -> bool {
        self.parse(vec![token]).is_ok() || token.parse::<f64>().is_ok()
    }
    pub fn parse(&self, tokens: Vec<&str>) -> Result<String> {
        if tokens.len() != 1 {
            return self.parse_statements(tokens);
        }
        match tokens[0] {
            "short" | "i16" => Ok(random::<i16>().to_string()),
            "int" | "i32" => Ok(random::<i32>().to_string()),
            "i64" => Ok(random::<i64>().to_string()),
            "int128_t" | "i128" => Ok(random::<i128>().to_string()),
            "u16" => Ok(random::<u16>().to_string()),
            "u32" => Ok(random::<u32>().to_string()),
            "u64" => Ok(random::<u64>().to_string()),
            "uint128_t" | "u128" => Ok(random::<u128>().to_string()),
            "bigint" => {
                let mut iter = repeat_with(|| thread_rng().gen_range(48_u8..57_u8) as char)
                    .take(random::<u16>() as usize);
                let num = iter.position(|x| x != '0');
                return Ok(if num == None {
                    "0".into()
                } else {
                    iter.clone().collect::<Vec<char>>()[num.unwrap() as usize..]
                        .iter()
                        .collect::<String>()
                });
            }
            "string" => Ok(
                repeat_with(|| thread_rng().gen_range(10_u8..126_u8) as char)
                    .take(random::<u16>() as usize)
                    .collect::<String>(),
            ),
            "char" => Ok(random::<char>().to_string()),
            _ => bail!(
                "{}",
                format!(
                    "failed to parse at position {} : read undeclared tokens",
                    self.position + 1
                )
                .color(Color::Red)
                .bold()
            ),
        }
    }
    pub fn parse_statements(&self, tokens: Vec<&str>) -> Result<String> {
        let mut ret: Result<String> = Ok(String::new());
        for (c, i) in tokens.clone().into_iter().enumerate() {
            if ret.is_err() {
                break;
            }
            match i {
                "to" => {
                    ret = Ok(thread_rng()
                        .gen_range(
                            tokens[c - 1].parse::<u128>().expect("Failed to parse")
                                ..tokens[c + 1].parse::<u128>().expect("Failed to parse"),
                        )
                        .to_string());
                }
                _ => {
                    if !self.is_vailid_token(i) {
                        ret = Err(anyhow!(
                            "{}",
                            format!(
                                "failed to parse at position {} : read undeclared tokens",
                                self.position + 1
                            )
                            .color(Color::Red)
                            .bold()
                        ));
                    }
                }
            }
        }
        ret
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_parse() {
        todo!();
    }
}
