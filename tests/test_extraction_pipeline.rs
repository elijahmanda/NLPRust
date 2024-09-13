#[cfg(test)]
mod tests {
    use nlp_rust::entity::{RegexEntityParser, ExtractionPipeline};

    #[test]
    fn test_extraction_pipeline_single_parser() {
        let patterns = vec![
            ("greeting", r"hello|hi")
        ];
        let parser = RegexEntityParser::new(patterns, None);
        let mut pipeline = ExtractionPipeline { parsers: vec![Box::new(parser)] };

        let tokens = pipeline.extract("Hello there!");
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0].text, "Hello".to_string());
        assert_eq!(tokens[0].entity.clone(), Some("greeting".to_string()));
    }

    #[test]
    fn test_extraction_pipeline_multiple_parsers() {
        let patterns1= vec![
            ("greeting", r"hello|hi")
        ];
        let patterns2= vec![
            ("entity", r"world")
        ];

        let parser1 = RegexEntityParser::new(patterns1, None);
        let parser2 = RegexEntityParser::new(patterns2, None);
        let mut pipeline = ExtractionPipeline { parsers: vec![Box::new(parser1), Box::new(parser2)] };

        let tokens = pipeline.extract("Hello world!");
        assert_eq!(tokens.len(), 4);

        assert_eq!(tokens[0].text, "Hello".to_string());
        assert_eq!(tokens[0].entity.clone(), Some("greeting".to_string()));

        assert_eq!(tokens[2].text, "world".to_string());
        assert_eq!(tokens[2].entity.clone(), Some("entity".to_string()));
    }

    #[test]
    fn test_extraction_pipeline_with_gaps() {
        let patterns= vec![
            ("greeting", r"hello")
        ];

        let parser = RegexEntityParser::new(patterns, None);
        let mut pipeline = ExtractionPipeline { parsers: vec![Box::new(parser)] };

        let tokens = pipeline.extract("Hello amazing world!");
        assert_eq!(tokens.len(), 2);

        assert_eq!(tokens[0].text, "Hello".to_string());
        assert_eq!(tokens[0].entity.clone(), Some("greeting".to_string()));

        assert_eq!(tokens[1].text, " amazing world!".to_string());
        assert!(tokens[1].entity.is_none());
    }

    #[test]
    fn test_extraction_pipeline_edge_cases() {
        let patterns = vec![
            ("greeting", r"hello")
        ];

        let parser = RegexEntityParser::new(patterns, None);
        let mut pipeline = ExtractionPipeline { parsers: vec![Box::new(parser)] };

        // Test with empty string
        let tokens = pipeline.extract("");
        assert_eq!(tokens.len(), 1);

        // Test with only whitespace
        let tokens = pipeline.extract("   ");
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0].text, "   ");
        assert!(tokens[0].entity.is_none());
    }
    
    #[test]
    fn test_extraction_pipeline_nested_token_processing() {
        let mut pipeline = ExtractionPipeline {
            parsers: vec![
                Box::new(RegexEntityParser::new(
                    vec![
                        ("word_3", "[A-Za-z]{3}")
                    ].to_vec(),
                    None
                )),
                Box::new(RegexEntityParser::new(
                    vec![
                        ("word", "[A-Za-z]+")
                    ].to_vec(),
                    None
                )),
            ],
        };
        let tokens = pipeline.extract("abcdefg");
        assert_eq!(tokens.len(), 3); // First parser splits into "abc", "def", "g", second parser processes further.
        assert_eq!(tokens[0].text, "abc".to_string());
        assert_eq!(tokens[1].text, "def".to_string());
        assert_eq!(tokens[2].text, "g".to_string());
    }
}
