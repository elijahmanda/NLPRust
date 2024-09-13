#[cfg(test)]
mod tests {
    use nlp_rust::entity::{EntityParser, RegexEntityParser};

    #[test]
    fn test_regex_entity_parser_simple() {
        let patterns = vec![
            ("greeting", r"hello|hi"),
            ("entity", r"\bworld\b")
        ];
        let mut parser = RegexEntityParser::new(patterns, None);

        let tokens = parser.parse("Hello World!");
        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].text, "Hello".to_string());
        assert_eq!(tokens[0].entity.clone(), Some("greeting".to_string()));
        assert_eq!(tokens[0].span.unwrap(), (0, 5));

        assert_eq!(tokens[1].text, "World".to_string());
        assert_eq!(tokens[1].entity.clone(), Some("entity".to_string()));
        assert_eq!(tokens[1].span.unwrap(), (6, 11));
    }

    #[test]
    fn test_regex_entity_parser_multiple_patterns() {
        let patterns = vec![
            ("greeting", r"hello|hi"),
            ("farewell", r"goodbye|bye"),
        ];
        let mut parser = RegexEntityParser::new(patterns, None);

        let tokens = parser.parse("Hello and Goodbye!");
        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].text, "Hello".to_string());
        assert_eq!(tokens[0].entity.clone(), Some("greeting".to_string()));

        assert_eq!(tokens[1].text, "Goodbye".to_string());
        assert_eq!(tokens[1].entity.clone(), Some("farewell".to_string()));
    }

    #[test]
    fn test_regex_entity_parser_no_match() {
        let patterns = vec![
            ("greeting", r"hello|hi")
        ];
        let mut parser = RegexEntityParser::new(patterns, None);

        let tokens = parser.parse("Good evening.");
        assert_eq!(tokens.len(), 0);
    }

    #[test]
    fn test_complex_patterns_overlapping() {
        let patterns = vec![
            ("date", r"\b\d{1,2}/\d{1,2}/\d{4}\b"),
            ("time", r"\b\d{1,2}:\d{2}(?:AM|PM)?\b"),
        ];

        let mut parser = RegexEntityParser::new(patterns, None);

        let text = "The event is on 12/31/2024 10:00AM.";
        let tokens = parser.parse(text);

        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].text, "12/31/2024".to_string());
        assert_eq!(tokens[0].entity.clone(), Some("date".to_string()));

        assert_eq!(tokens[1].text, "10:00AM".to_string());
        assert_eq!(tokens[1].entity.clone(), Some("time".to_string()));
    }
    
    #[test]
    fn test_complex_patterns_combined() {
        let patterns = vec![
            ("email", r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,7}\b"),
            // ("phone", r"\b\d{3}[-.]\d{3}[-.]\d{4}\b"),
            ("mention", r"@\w+"),
            // ("hashtag", r"#\w+"),
            // ("url", r"\bhttps?://[^\s/\$\.\?#].[^\s]*\b"),
        ];

        let mut parser = RegexEntityParser::new(patterns, None);

        let text = "Email me at john.doe@example.com or call 123-456-7890. Follow @username or visit https://example.com #amazing.";
        let tokens = parser.parse(text);
        
        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].text, "john.doe@example.com".to_string());
        assert_eq!(tokens[0].entity.clone(), Some("email".to_string()));

        // assert_eq!(tokens[1].text, "123-456-7890".to_string());
        // assert_eq!(tokens[1].entity.clone(), Some("phone".to_string()));

        assert_eq!(tokens[1].text, "@username".to_string());
        assert_eq!(tokens[1].entity.clone(), Some("mention".to_string()));

        // assert_eq!(tokens[8].text, "https://example.com".to_string());
        // assert_eq!(tokens[8].entity.clone(), Some("url".to_string()));

        // assert_eq!(tokens[3].text, "#amazing".to_string());
        // assert_eq!(tokens[3].entity.clone(), Some("hashtag".to_string()));
    }
}
