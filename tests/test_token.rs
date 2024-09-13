#[cfg(test)]
mod tests {
    use nlp_rust::tokens::Token;

    #[test]
    fn test_token_creation() {
        let token = Token::new("Hello World!".to_string(), None, Some((0, 11)));
        assert_eq!(token.text, "Hello World!".to_string());
        assert_eq!(token.span, Some((0, 11)));
        assert!(token.entity.is_none());
    }

    #[test]
    fn test_token_with_entity() {
        let token = Token::new("Hello".to_string(), Some("greeting".to_string()), Some((0, 5)));
        assert_eq!(token.text, "Hello".to_string());
        assert_eq!(token.entity.clone(), Some("greeting".to_string()));
        assert_eq!(token.span.unwrap(), (0, 5));
    }

    #[test]
    fn test_token_edge_cases() {
        // Empty text token
        let token = Token::new("".to_string(), None, Some((0, 0)));
        assert_eq!(token.text, "".to_string());
        assert_eq!(token.span.unwrap(), (0, 0));
        assert!(token.entity.is_none());
    }

    #[test]
    fn test_token_with_special_characters() {
        let token = Token::new("Hello, World! @#%$^&*()".to_string(), None, Some((0, 24)));
        assert_eq!(token.text, "Hello, World! @#%$^&*()".to_string());
        assert_eq!(token.span, Some((0, 24)));
    }
}

