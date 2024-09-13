# NLPRust

NLPRust is a reimplementation of the original NLP library, focusing on performance improvements using Rust. This library provides tools for entity extraction, tokenization, and parsing with flexible options to define custom patterns and parsers. It is optimized for parsing and identifying structured entities such as numbers, IP addresses, emails, and more, all in Rust.

## Features

- **Custom Entity Parsers:**
  - `EntityParser` base class for creating custom parsers.
  - `RegexEntityParser` for parsing entities using regular expressions.
  
- **Tokenization:**
  - Efficient tokenization based on user-defined patterns and regex.
  
- **Entity Extraction Pipeline:**
  - The `ExtractionPipeline` allows you to combine multiple parsers in sequence for extracting various entities from text input.

## Installation

To use NLPRust, add the following to your `Cargo.toml` file:

```toml
[dependencies]
nlp_rust = "0.1.0"
```

## Usage

### 1. **Basic Entity Parsing with Regex**

Create a basic entity parser using the `RegexEntityParser`.

```rust
use nlp_rust::entity::RegexEntityParser;

fn main() {
    let patterns = vec![
        ("greeting", r"hello|hi"),
        ("entity", r"\bworld\b")
    ];
    let mut parser = RegexEntityParser::new(patterns, None);

    let tokens = parser.parse("Hello World!");
    for token in tokens {
        println!("{:?}", token);
    }
}
```

### 2. **Combining Multiple Parsers with Extraction Pipeline**

You can use `ExtractionPipeline` to combine different parsers into a single pipeline, extracting various entities from a single text input.

```rust
use nlp_rust::entity::{RegexEntityParser, ExtractionPipeline};

fn main() {
    let patterns1 = vec![
        ("greeting", r"hello|hi"),
    ];
    let patterns2 = vec![
        ("entity", r"\bworld\b")
    ];

    let parser1 = RegexEntityParser::new(patterns1, None);
    let parser2 = RegexEntityParser::new(patterns2, None);
    
    let mut pipeline = ExtractionPipeline { parsers: vec![Box::new(parser1), Box::new(parser2)] };
    
    let tokens = pipeline.extract("Hello world!");
    for token in tokens {
        println!("{:?}", token);
    }
}
```

### 3. **Advanced Pattern Matching**

The `RegexEntityParser` supports more complex patterns like dates, times, and custom formats.

```rust
use nlp_rust::entity::RegexEntityParser;

fn main() {
    let patterns = vec![
        ("date", r"\b\d{1,2}/\d{1,2}/\d{4}\b"),
        ("time", r"\b\d{1,2}:\d{2}(?:AM|PM)?\b"),
    ];

    let mut parser = RegexEntityParser::new(patterns, None);

    let tokens = parser.parse("The event is on 12/31/2024 at 10:00AM.");
    for token in tokens {
        println!("{:?}", token);
    }
}
```

### 4. **Edge Cases and Empty Inputs**

NLPRust is designed to handle edge cases such as empty strings and text with only whitespace.

```rust
use nlp_rust::entity::RegexEntityParser;
use nlp_rust::entity::ExtractionPipeline;

fn main() {
    let patterns = vec![
        ("greeting", r"hello")
    ];

    let parser = RegexEntityParser::new(patterns, None);
    let mut pipeline = ExtractionPipeline { parsers: vec![Box::new(parser)] };

    // Test with empty string
    let tokens = pipeline.extract("");
    println!("{:?}", tokens);

    // Test with whitespace
    let tokens = pipeline.extract("   ");
    println!("{:?}", tokens);
}
```

## Unit Testing

NLPRust includes comprehensive tests to ensure functionality. Here is a sample test suite for `RegexEntityParser`:

```rust
#[cfg(test)]
mod tests {
    use nlp_rust::entity::{RegexEntityParser, ExtractionPipeline};

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

```

## Conclusion

NLPRust is a high-performance NLP library that provides flexible tools for tokenization, entity extraction, and regex-based parsing. It’s well-suited for handling structured entities in unstructured text, with a design focused on efficiency and scalability.

## License

This project is licensed under the MIT License.
