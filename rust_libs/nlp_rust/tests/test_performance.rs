#[cfg(test)]
mod performance_tests {
    use super::*;
    use nlp_rust::entity::{EntityParser, RegexEntityParser, ExtractionPipeline};
    use std::time::Instant;

    #[test]
    fn test_regex_entity_parser_performance() {
        let patterns = vec![
            ("email", r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,7}\b"),
            ("url", r"\bhttps?://[^\s/$.?#].[^\s]*\b"),
            ("phone", r"\b\d{3}[-.]\d{3}[-.]\d{4}\b"),
        ];
        let mut parser = RegexEntityParser::new(patterns, None);

        // Generate a large text input by repeating a pattern multiple times
        let mut large_text = String::new();
        for _ in 0..10000 {
            large_text.push_str("Contact john.doe@example.com or visit https://example.com. Call 123-456-7890.\n");
        }

        let start_time = Instant::now();
        let mut tokens = parser.parse(&large_text);
        let duration = start_time.elapsed();

        println!("Parsing large text took: {:?}", duration);
        assert_eq!(tokens.iter().filter(|t| t.entity.is_some()).collect::<Vec<_>>().len(), 10_000 * 3);
    }
    
    #[test]
    fn test_extraction_pipeline_performance() {
        let patterns1 = vec![
            ("email", r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,7}\b")
        ];
        let patterns2 = vec![
            ("url", r"\bhttps?://[^\s/$.?#].[^\s]*\b")
        ];
        let patterns3 = vec![
            ("phone", r"\b\d{3}[-.]\d{3}[-.]\d{4}\b")
        ];

        let mut parser1 = RegexEntityParser::new(patterns1, None);
        let mut parser2 = RegexEntityParser::new(patterns2, None);
        let mut parser3 = RegexEntityParser::new(patterns3, None);

        let mut pipeline = ExtractionPipeline {
            parsers: vec![Box::new(parser1), Box::new(parser2), Box::new(parser3)]
        };
        
        // Generate a large text input by repeating a pattern multiple times
        let mut large_text = String::new();
        for _ in 0..10000 {
            large_text.push_str("Contact john.doe@example.com or visit https://example.com. Call 123-456-7890.\n");
        }

        let start_time = Instant::now();
        let mut tokens = pipeline.extract(&large_text);
        let duration = start_time.elapsed();

        println!("Pipeline extraction on large text took: {:?}", duration);
        assert_eq!(tokens.iter().filter(|t| t.entity.is_some()).collect::<Vec<_>>().len(), 10_000 * 3);
    }
}