#[cfg(test)]
mod tests {
    use super::*;
    use nlp_rust::parsers::number::{Config, Data, DataHolder};

    #[test]
    fn test_single_digit_conversion() {
        let config = Config::default();
        let mut data = Data::new(config);
        let dataholder = DataHolder {
            type_: "text",
            text: Some("five".to_string()),
            int: None,
            float: None,
        };
        
        let result = words2num(dataholder, &mut data);
        assert_eq!(result, Some(5.0));
    }

    #[test]
    fn test_teens_conversion() {
        let config = Config::default();
        let mut data = Data::new(config);
        let dataholder = DataHolder {
            type_: "text",
            text: Some("seventeen".to_string()),
            int: None,
            float: None,
        };
        
        let result = words2num(dataholder, &mut data);
        assert_eq!(result, Some(17.0));
    }

    #[test]
    fn test_tens_conversion() {
        let config = Config::default();
        let mut data = Data::new(config);
        let dataholder = DataHolder {
            type_: "text",
            text: Some("forty-two".to_string()),
            int: None,
            float: None,
        };
        
        let result = words2num(dataholder, &mut data);
        assert_eq!(result, Some(42.0));
    }

    #[test]
    fn test_large_number_conversion() {
        let config = Config::default();
        let mut data = Data::new(config);
        let dataholder = DataHolder {
            type_: "text",
            text: Some("one thousand two hundred thirty-four".to_string()),
            int: None,
            float: None,
        };
        
        let result = words2num(dataholder, &mut data);
        assert_eq!(result, Some(1234.0));
    }

    #[test]
    fn test_invalid_text() {
        let config = Config::default();
        let mut data = Data::new(config);
        let dataholder = DataHolder {
            type_: "text",
            text: Some("invalid text".to_string()),
            int: None,
            float: None,
        };
        
        let result = words2num(dataholder, &mut data);
        assert_eq!(result, None);
    }

    #[test]
    fn test_mixed_case_text() {
        let config = Config::default();
        let mut data = Data::new(config);
        let dataholder = DataHolder {
            type_: "text",
            text: Some("Three Hundred and Fifty-Six".to_string()),
            int: None,
            float: None,
        };
        
        let result = words2num(dataholder, &mut data);
        assert_eq!(result, Some(356.0));
    }

    #[test]
    fn test_float_conversion() {
        let config = Config::default();
        let mut data = Data::new(config);
        let dataholder = DataHolder {
            type_: "text",
            text: Some("one point five".to_string()),
            int: None,
            float: None,
        };
        
        let result = words2num(dataholder, &mut data);
        assert_eq!(result, Some(1.5));
    }
}
