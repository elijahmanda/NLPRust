pub mod config;
pub mod data;
pub mod tokenizer;
pub mod constants;
pub mod normalize;
pub mod utils;
pub mod words2num;
pub mod classes;

pub use config::Config;
pub use data::Data;
pub use tokenizer::tokenize;
pub use normalize::{Pipe, normalize_and, check_valid, recover_real_indices_and_match};
pub use words2num::words2num;
pub use utils::DataHolder;



#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_number_tokenizer() {
        let mut text = "
    1 -89 +799 588
    2.5 .577 1.2e2 23E3 -208.89 +13.7
    -2.7e2 +199.123E90 +5e-12 -.5
    -0.244e+19
    2,000 12,999 123,689 2,078,689
    12,089,688 1,230.0971 799,089.13
    124,799,799,588.8981349901
    2,008e10 -12,899 +77,089 9,000.7
    -134,799 -6,799.999 +13,689E-12
    +799,799e+1 -67,799.968e-123
    +78,000E10 -123,147E2
    2'000 12'999 123'689 2'078'689
    12'089'688 1'230.0971 799'089.13
    124'799'799'588.8981349901
    2'008e10 -12'899 +77'089 9'000.7
    -134'799 -6'799.999 +13'689E-12
    +799'799e+1 -67'799.968e-123
    +144'478E10 -133'899E2
    1k -89M +799B 588m
    2.5c .577G 1.2e2T 23E3Z -208.89y
    ".to_string();
        let expected: usize = text
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .len();
    
        let tokens: Vec<String> = tokenize(&text);
        let parsed_length: usize = tokens.len();
        assert_eq!(expected, parsed_length);
    }
    
    #[test]
    fn test_valid_token() {
        let mut config = Config::default();
        let mut data = Data::new(config);
        let nums = vec![
            "one".to_string(),
            "two".to_string(),
            "three".to_string(),
            "four".to_string(),
            "five".to_string(),
            "six".to_string(),
            "seven".to_string(),
            "eight".to_string(),
            "nine".to_string(),
        ];
        for num in nums {
            assert!(check_valid(num, &mut data));
        }
    }
}