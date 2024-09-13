use regex::Regex;
use std::collections::{HashMap, HashSet};

pub struct RegexProcessor;

impl RegexProcessor {

    pub fn new() -> Self {
        let processor = Self {  };
        processor
    }

    pub fn preprocess(&mut self, sent: &str) -> String {
        let replacements = vec![
            (r"\*", ""),
            (r"\s*\(.+\)\s*", ""),
            (r"(.+)([\[].+[\]])", r"$1|$2"),
            (r"(.+)[;](.+)", r"$1|$2"),
            (r"(.+)[/](.+)", r"$1|$2"),
            (r"[\[\]]", ""),
        ];

        let mut result = sent.to_string();
        for (regex, replacement) in replacements {
            let re = Regex::new(regex).unwrap();
            result = re.replace_all(&result, replacement).to_string();
        }
        result.to_lowercase()
    }

    pub fn substitute(&mut self, name: &str) -> String {
        let substitutions = vec![
            (r"[']s\b", r"[']?s?"),
            (r"['](?=[^\]])", r"[']?"),
            (",", ",?"),
            (r"([\-\.])", r"(?:\\$1|\\s+)"),
            (r"([_]+)", r"[\\$1]*"),
        ];

        let mut result = name.to_string();
        for (regex, substitution) in substitutions {
            let re = Regex::new(regex).unwrap();
            result = re.replace_all(&result, substitution).to_string();
        }
        result
    }

    pub fn preprocess_names_to_patterns(&mut self, names: Vec<String>) -> Vec<String> {
        let mut seen = HashSet::new();
        for mut name in names {
            name = self.preprocess(&name);
            name = name.trim().to_string();
            name = name.split_whitespace().collect::<Vec<&str>>().join(r"\s+");
            name = self.substitute(&name);
            seen.insert(name);
        }

        let mut seen_vec: Vec<String> = seen.into_iter().collect();
        seen_vec.sort_by(|a, b| b.len().cmp(&a.len()));
        seen_vec
    }

    pub fn process_string_for_pattern(&mut self, string: String) -> String {
        self.preprocess_names_to_patterns(vec![string])[0].clone()
    }

    pub fn bound(&mut self, pattern: String, sides: Option<(&str, &str)>) -> String {
        if pattern.is_empty() {
            return pattern.to_string();
        }

        let (left, right) = if let Some(sides) = sides {
            let left: &str = sides.0;
            let right: &str = sides.1;
            (left, right)
        } else {
            (r"(?<![a-zA-Z\d'])", r"(?![a-zA-Z\d])")
        };

        format!("{}(?:{}){}", left, pattern, right)
    }

    pub fn no_digits_bound(&mut self, pattern: String) -> String {
        self.bound(pattern, Some((
            r"(?<![\d])",
            r"(?![\d])",
        )))
    }

    pub fn all_cases(&mut self, string: String) -> String {
        string.chars().map(|ch| {
            if ch.is_alphabetic() {
                format!("({}|{})", ch.to_ascii_lowercase(), ch.to_ascii_uppercase())
            } else {
                ch.to_string()
            }
        }).collect::<Vec<String>>().concat()
    }

    pub fn join(&mut self, patterns: Vec<String>, sep: Option<&str>) -> String {
        let mut pats = patterns
            .iter().
            filter(|p| !p.trim().is_empty())
            .collect::<Vec<_>>();
        if pats.is_empty() {
            return "".to_string();
        }
        pats.sort_by(|a, b| b.len().cmp(&a.len()));
        let sep = sep.unwrap_or("|");
        format!(
            "(?:{})",
            pats
                .iter()
                .map(|&s| s.clone())
                .collect::<Vec<String>>()
                .join(sep)
        )
    }

    pub fn retrie(&mut self, patterns: Vec<String>) -> String {
        let mut trie = Whitelist::new(patterns);
        trie.pattern()
    }

    pub fn group_strings(
        &mut self,
        strings: Vec<String>,
        reverse: Option<bool>,
        escape: Option<bool>,
        bounds: Option<(&str, &str)>,
    ) -> String {
        let escape = escape.unwrap_or(false);
        let reverse = reverse.unwrap_or(true);

        let strings = if escape {
            strings.into_iter().map(|s| regex::escape(&s)).collect::<Vec<_>>()
        } else {
            strings
        };

        let mut strings_by_length: HashMap<usize, Vec<String>> = HashMap::new();

        for string in strings {
            let length = string.trim_start_matches("\\").len();
            strings_by_length.entry(length).or_insert_with(Vec::new).push(string);
        }

        let mut grouped_regex = Vec::new();

        if let Some(single_char_list) = strings_by_length.remove(&1) {
            let single_char_regex = format!("[{}]", single_char_list.concat());
            grouped_regex.push(single_char_regex);
        }

        for (_, mut string_list) in strings_by_length {
            if string_list.len() == 1 {
                grouped_regex.push(string_list.pop().unwrap());
            } else {
                let regex_str = format!("({})", string_list.join("|"));
                grouped_regex.push(regex_str);
            }
        }

        if reverse {
            grouped_regex.sort_by(|a, b| {
                let key_a = a.trim_start_matches("(").trim_end_matches(")");
                let key_b = b.trim_start_matches("(").trim_end_matches(")");
                key_b.len().cmp(&key_a.len())
            });
        }

        let grouped_regex = grouped_regex.join("|");

        let (left, right) = bounds.unwrap_or((r"(?<![a-zA-Z'])", r"(?![a-zA-Z])"));
        format!("{}(?:{}){}", left, grouped_regex, right)
    }
}

pub struct Whitelist {
    patterns: Vec<String>,
}

impl Whitelist {

    pub fn new(patterns: Vec<String>) -> Self {
        Whitelist { patterns }
    }

    pub fn pattern(&mut self) -> String {
        self.patterns.join("|")
    }
}


