use rayon::prelude::*;
use crate::tokens::Token;
use crate::tokenizers::{RegexTokenizer};
use crate::utils::sequences::missing_indexes;

pub trait EntityParser {
    fn parse(&mut self, text: &str) -> Vec<Token> {
        vec![Token::new(text.to_string(), None, Some((0, text.len())))]
    }

    fn parse_tokenize(&mut self, text: &str) -> Vec<Token> {
        if text.trim().is_empty() {
            return vec![Token::new(text.to_string(), None, Some((0, text.len())))];
        }

        let mut tokens = self.parse(text);
        if tokens.is_empty() {
            return vec![Token::new(text.to_string(), None, Some((0, text.len())))];
        }

        tokens.sort_by(|a, b| a.span.unwrap().0.cmp(&b.span.unwrap().0));
        let missing_spans = missing_indexes(tokens.par_iter().map(|t| t.span.unwrap()).collect(), text.len());
        for (start, end) in missing_spans {
            tokens.push(Token::new(text[start..end].to_string(), None, Some((start, end))));
        }
        tokens.sort_by(|a, b| a.span.unwrap().0.cmp(&b.span.unwrap().0));
        tokens
    }
}

pub struct RegexEntityParser {
    tokenizer: RegexTokenizer,
}

impl RegexEntityParser {
    pub fn new(patterns: Vec<(&str, &str)>, flags: Option<&str>) -> Self {
        let mut tokenizer = RegexTokenizer::new(None);
        tokenizer.set_patterns(patterns.clone(), false);
        tokenizer.compile(flags, false);
        RegexEntityParser {
            tokenizer,
        }
    }
}

impl EntityParser for RegexEntityParser {
    fn parse(&mut self, text: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let parsed = self.tokenizer.tokenize(text, false);
        for (text, entity, span) in parsed {
            tokens.push(Token::new(text, entity, Some((span.0, span.1))));
        }
        tokens
    }
}

pub struct ExtractionPipeline {
    pub parsers: Vec<Box<dyn EntityParser>>,
}

impl ExtractionPipeline {

    pub fn extract(&mut self, text: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut i: u32 = 0;
        for parser in self.parsers.iter_mut() {
            let mut new_tokens: Vec<Token> = Vec::new();
            if i == 0 {
                tokens = parser.parse_tokenize(text);
                i += 1;
                continue;
            }

            for token in tokens {
                if token.entity.is_some() || token.text.trim().is_empty() {
                    new_tokens.push(token);
                    continue;
                }
                let entity_tokens = parser.parse_tokenize(&token.text);
                let dstart = token.span.unwrap().0;
                for entity_token in entity_tokens {
                    new_tokens.push(Token::new(
                        entity_token.text,
                        entity_token.entity,
                        Some((entity_token.span.unwrap().0 + dstart, entity_token.span.unwrap().1 + dstart)),
                    ));
                }
            }
            tokens = new_tokens;
        }
        tokens.sort_by(|a, b| a.span.unwrap().0.cmp(&b.span.unwrap().0));
        tokens
    }
}
