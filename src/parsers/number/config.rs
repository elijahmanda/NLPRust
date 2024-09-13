

#[derive(Clone, Debug)]
pub struct Config {
    pub language: Option<String>,
    pub signs_allowed: Option<bool>,
    pub parse_complex: Option<bool>,
    pub bounded_numbers: Option<bool>,
    pub mixed_nums: Option<bool>,
    pub merge: Option<bool>,
    pub merge_multiples: Option<bool>,
    pub merge_implied: Option<bool>,
    pub merge_points: Option<bool>,
    pub merge_informals: Option<bool>,
    pub exclude_separators: Option<Vec<String>>,
    pub exclude_suffixes: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            language: Some("en".to_string()),
            signs_allowed: Some(false),
            parse_complex: Some(false),
            bounded_numbers: Some(false),
            mixed_nums: Some(true),
            merge: Some(true),
            merge_multiples: Some(true),
            merge_implied: Some(false),
            merge_points: Some(false),
            merge_informals: Some(true),
            exclude_separators: None,
            exclude_suffixes: vec!["m".to_string(), "y".to_string()].into(),
        }
    }
}
