/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_number_integer() {
        assert_eq!(parse_number("123").unwrap(), (123, "integer"));
    }

    #[test]
    fn test_parse_number_binary() {
        assert_eq!(parse_number("0b1010").unwrap(), (10, "binary"));
        assert!(parse_number("0b102").is_err()); // Invalid binary number
    }

    #[test]
    fn test_parse_number_octal() {
        assert_eq!(parse_number("0o755").unwrap(), (493, "octal"));
        assert!(parse_number("0o78").is_err()); // Invalid octal number
    }

    #[test]
    fn test_parse_number_hexadecimal() {
        assert_eq!(parse_number("0x1A3F").unwrap(), (6719, "hexadecimal"));
        assert!(parse_number("0x1G").is_err()); // Invalid hexadecimal number
    }

    #[test]
    fn test_parse_number_invalid() {
        assert!(parse_number("xyz").is_err());
    }

    #[test]
    fn test_string_to_num_integer() {
        let result = string_to_num("123".to_string());
        assert_eq!(result.int, Some(123));
        assert_eq!(result.type_, "integer");
    }

    #[test]
    fn test_string_to_num_float() {
        let result = string_to_num("123.45".to_string());
        assert_eq!(result.float, Some(123.45));
        assert_eq!(result.type_, "float");
    }

    #[test]
    fn test_string_to_num_binary() {
        let result = string_to_num("0b1010".to_string());
        assert_eq!(result.int, Some(10));
        assert_eq!(result.type_, "binary");
    }

    #[test]
    fn test_string_to_num_octal() {
        let result = string_to_num("0o755".to_string());
        assert_eq!(result.int, Some(493));
        assert_eq!(result.type_, "octal");
    }

    #[test]
    fn test_string_to_num_hexadecimal() {
        let result = string_to_num("0x1A3F".to_string());
        assert_eq!(result.int, Some(6719));
        assert_eq!(result.type_, "hexadecimal");
    }

    #[test]
    fn test_string_to_num_text() {
        let result = string_to_num("Hello, World!".to_string());
        assert_eq!(result.text, Some("Hello, World!".to_string()));
        assert_eq!(result.type_, "text");
    }

    #[test]
    fn test_pair_single_token() {
        let tokens = vec![2.0];
        let result = pair(tokens, Some(3.0));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0.float, Some(2.0));
        assert_eq!(result[0].1.float, Some(3.0));
    }

    #[test]
    fn test_pair_even_tokens() {
        let tokens = vec![2.0, 3.0];
        let result = pair(tokens, Some(0.0));
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].0.float, Some(2.0));
        assert_eq!(result[0].1.float, Some(3.0));
    }

    #[test]
    fn test_pair_odd_tokens() {
        let tokens = vec![
            DataHolder {
                type_: "float",
                text: None,
                int: None,
                float: Some(2.0),
            },
            DataHolder {
                type_: "float",
                text: None,
                int: None,
                float: Some(3.0),
            },
            DataHolder {
                type_: "float",
                text: None,
                int: None,
                float: Some(4.0),
            },
        ];
        let result = pair(tokens, Some(0.0));
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].0.float, Some(2.0));
        assert_eq!(result[0].1.float, Some(3.0));
        assert_eq!(result[1].0.float, Some(4.0));
        assert_eq!(result[1].1.float, Some(0.0));
    }

        #[test]
    fn test_clean_text() {
        let data = DataHolder {
            type_: "text",
            text: Some("Hello, World!".to_string()),
            int: None,
            float: None,
        };
        let result = _clean(data, Some(true));
        assert_eq!(result.text.unwrap(), "hello world");
    }

    #[test]
    fn test_clean_non_text() {
        let data = DataHolder {
            type_: "float",
            text: None,
            int: None,
            float: Some(3.14),
        };
        let result = _clean(data.clone(), None);
        assert_eq!(result, data); // Should return the same data as it was not text
    }
}
*/
