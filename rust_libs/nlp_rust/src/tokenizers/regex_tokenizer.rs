use rayon::prelude::*;
use fancy_regex::{Regex};
use std::collections::HashSet;

pub const MULTILINE: &str = &"(?m)";
pub const IGNORECASE: &str = &"(?i)";
pub const EXTENDED: &str = &"(?x)";
pub const DEFAULT_RE_FLAGS: &str = &"(?m)(?i)(?x)";

pub struct RegexTokenizer {
    patterns: Vec<(String, String)>,
    compiled_patterns: Vec<(Regex, String)>,
    compiled: bool,
}

impl RegexTokenizer {
    pub fn new(patterns: Option<Vec<(&str, &str)>>) -> Self {
        let mut tokenizer = Self {
            patterns: patterns.unwrap_or_else(Vec::new).par_iter().map(|t| (t.0.to_string(), t.1.to_string())).collect(),
            compiled_patterns: Vec::new(),
            compiled: false,
        };
        if !tokenizer.patterns.is_empty() {
            tokenizer.compile(None, false);
        }
        tokenizer
    }

    pub fn patterns(&self) -> &Vec<(String, String)> {
        &self.patterns
    }

    pub fn compile(&mut self, flags: Option<&str>, sort: bool) {
        self._compile(flags, sort);
    }

    fn _compile(&mut self, flags: Option<&str>, sort: bool) {
        let flags = flags.unwrap_or(DEFAULT_RE_FLAGS);
        if sort {
            self.patterns.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
        }

        self.compiled_patterns = self.patterns.par_iter()
            .map(|(entity, pattern)| {
                let regex = Regex::new(&format!("{}{}", flags, pattern)).unwrap();
                (regex, entity.clone())
            })
            .collect();

        if !self.compiled_patterns.is_empty() {
            self.compiled = true;
        }
    }

    fn _merge_non_entity_tokens(&self, text: &str, tokens: &mut Vec<(String, Option<String>, (usize, usize))>) {
        self._sort(tokens);
        let indexes: Vec<(usize, usize)> = tokens.iter().map(|n| n.2).collect();
        let missing = missing_indexes(&indexes, text.len());
        tokens.extend(missing.into_iter().map(|(start, end)| (
            text[start..end].to_string(),
            None,
            (start, end),
        )));
    }

    fn _sort(&self, tokens: &mut Vec<(String, Option<String>, (usize, usize))>) {
        tokens.sort_by(|a, b| a.2.cmp(&b.2));
    }

    pub fn tokenize(&mut self, text: &str, merge: bool) -> Vec<(String, Option<String>, (usize, usize))> {
        if !self.compiled {
            self.compile(None, false);
        }

        let mut tokens: Vec<(String, Option<String>, (usize, usize))> = Vec::new();
        let mut temp_text = text.to_string();

        for (compiled_pattern, entity) in &self.compiled_patterns {
            let mut replacements = Vec::new();

            for cap in compiled_pattern.captures_iter(&temp_text) {
                if let Some(mat) = cap.expect("No group").get(0) {
                    let start = mat.start();
                    let end = mat.end();
                    tokens.push((mat.as_str().to_string(), Some(entity.clone()), (start, end)));
                    replacements.push((start, end));
                }
            }

            for (start, end) in replacements {
                temp_text.replace_range(start..end, &" ".repeat(end - start));
            }
        }

        if merge {
            self._merge_non_entity_tokens(text, &mut tokens);
        }

        self._sort(&mut tokens);

        tokens
    }

    pub fn add_pattern(&mut self, entity: &str, pattern: &str) {
        assert!(!self.compiled);
        self.patterns.push((entity.to_string(), pattern.to_string()));
    }

    pub fn clear_patterns(&mut self) {
        self.patterns.clear();
        self.compiled_patterns.clear();
        self.compiled = false;
    }

    pub fn set_patterns(&mut self, patterns: Vec<(&str, &str)>, compile: bool) {
        assert!(!self.compiled);
        self.patterns = patterns.par_iter().map(|t| (t.0.to_string(), t.1.to_string())).collect();
        if compile {
            self.compile(None, false);
        }
    }

    pub fn get_entities(&self) -> Vec<String> {
        let mut entities: Vec<String> = self.patterns.iter()
            .map(|(entity, _)| entity.clone())
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        entities.sort();
        entities
    }

    pub fn get_pattern_count(&self) -> usize {
        self.patterns.len()
    }

    pub fn get_entity_count(&self) -> usize {
        self.get_entities().len()
    }
}

fn missing_indexes(indexes: &[(usize, usize)], total: usize) -> Vec<(usize, usize)> {
    let mut missing = Vec::new();

    if indexes.is_empty() {
        missing.push((0, total));
    } else {
        if indexes[0].0 > 0 {
            missing.push((0, indexes[0].0));
        }

        for i in 0..indexes.len() - 1 {
            let first_end = indexes[i].1;
            let next_start = indexes[i + 1].0;
            if next_start > first_end {
                missing.push((first_end, next_start));
            }
        }

        if indexes.last().unwrap().1 < total {
            missing.push((indexes.last().unwrap().1, total));
        }
    }

    missing
}
