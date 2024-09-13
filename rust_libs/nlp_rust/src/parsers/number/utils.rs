use fancy_regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub struct DataHolder {
    pub type_: &'static str,
    pub data: DataT,
}


#[derive(Debug, PartialEq, Clone)]
pub enum DataT {
    Float(f64),
    Int(i64),
    Text(String),
}

pub type OperationT = dyn Fn(f64, f64) -> f64;

pub fn mul(x: f64, y: f64) -> f64 {
    x * y
}

pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

pub fn to_base(mut x: u32, radix: u32) -> u32 {
    let mut result = vec![];
    loop {
        let m = x % radix;
        x = x / radix;
        // will panic if you use a bad radix (< 2 or > 36).
        result
            .push(
                std::char::from_digit(m, radix)
                .unwrap());
        if x == 0 {
            break;
        }
    }
    result
        .into_iter()
        .rev()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse::<u32>()
        .unwrap()
}

pub fn parse_number(string: &str) -> Result<(i64, &'static str), &'static str> {
    let string = string.trim();
    let string_lower = string.to_lowercase();

    // Match based on prefixes for different bases
    if string_lower.starts_with("0b") {
        // Binary
        match i64::from_str_radix(&string[2..], 2) {
            Ok(num) => Ok((num, "binary")),
            Err(_) => Err("Invalid binary number"),
        }
    } else if string_lower.starts_with("0o") {
        // Octal
        match i64::from_str_radix(&string[2..], 8) {
            Ok(num) => Ok((num, "octal")),
            Err(_) => Err("Invalid octal number"),
        }
    } else if string_lower.starts_with("0x") {
        // Hexadecimal
        match i64::from_str_radix(&string[2..], 16) {
            Ok(num) => Ok((num, "hexadecimal")),
            Err(_) => Err("Invalid hexadecimal number"),
        }
    } else {
        // Decimal or invalid number
        match string.parse::<i64>() {
            Ok(num) => Ok((num, "integer")),
            Err(_) => Err("Invalid decimal number"),
        }
    }
}

pub fn string_to_num(string: String) -> DataHolder {
    let mut type_: &'static str = "text";
    let mut int: i64 = 0; // Default initialization
    let mut float: f64 = 0.0; // Default initialization

    // Try to parse as integer, binary, octal, or hexadecimal
    if let Ok((parsed_int, parsed_type)) = parse_number(&string) {
        int = parsed_int;
        type_ = parsed_type;
    } else if let Ok(parsed_float) = string.parse::<f64>() {
        float = parsed_float;
        type_ = "float";
    }

    match type_ {
        "float" => DataHolder {
            type_: type_,
            data: DataT::Float(float),
        },
        "integer" | "octal" | "binary" | "hexadecimal" => DataHolder {
            type_: type_,
            data: DataT::Int(int),
        },
        _ => DataHolder {
            type_: type_,
            data: DataT::Text(string),
        },
    }
}

pub fn pair(tokens: Vec<DataHolder>, holder: Option<f64>) -> Vec<(DataHolder, DataHolder)> {
    let mut tokens = tokens;
    let mut new_tokens = Vec::new();

    if tokens.len() == 1 {
        return vec![
            (
                tokens[0].clone(),
                DataHolder {
                    type_: "float",
                    data: DataT::Float(holder.unwrap_or(0.0)),
                }
            )
        ];
    }

    if tokens.len() % 2 != 0 {
        tokens.push(
            DataHolder {
                type_: "float",
                data: DataT::Float(holder.unwrap_or(0.0)),
            }
        )
    }

    for chunk in tokens.chunks(2) {
        new_tokens.push((chunk[0].clone(), chunk[1].clone()));
    }

    new_tokens
}

pub fn _clean(data: DataHolder, lower: Option<bool>) -> DataHolder {
    match data.data {
        DataT::Text(text) => {
            let mut cleaned_text = text.clone();
            cleaned_text = Regex::new(r#"[\,'_]"#).unwrap()
                .replace_all(&cleaned_text, "")
                .to_string();
            if lower.unwrap_or(false) {
                cleaned_text = cleaned_text.to_lowercase();
            }
            DataHolder {
                type_: "text",
                data: DataT::Text(cleaned_text),
            }
        },
        _ => data,
    }
}

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
        if let DataT::Int(value) = result.data {
            assert_eq!(value, 123);
            assert_eq!(result.type_, "integer");
        } else {
            panic!("Expected integer, got something else");
        }
    }

    #[test]
    fn test_string_to_num_float() {
        let result = string_to_num("123.45".to_string());
        if let DataT::Float(value) = result.data {
            assert_eq!(value, 123.45);
            assert_eq!(result.type_, "float");
        } else {
            panic!("Expected float, got something else");
        }
    }

    #[test]
    fn test_string_to_num_binary() {
        let result = string_to_num("0b1010".to_string());
        if let DataT::Int(value) = result.data {
            assert_eq!(value, 10);
            assert_eq!(result.type_, "binary");
        } else {
            panic!("Expected binary, got something else");
        }
    }

    #[test]
    fn test_string_to_num_octal() {
        let result = string_to_num("0o755".to_string());
        if let DataT::Int(value) = result.data {
            assert_eq!(value, 493);
            assert_eq!(result.type_, "octal");
        } else {
            panic!("Expected octal, got something else");
        }
    }

    #[test]
    fn test_string_to_num_hexadecimal() {
        let result = string_to_num("0x1A3F".to_string());
        if let DataT::Int(value) = result.data {
            assert_eq!(value, 6719);
            assert_eq!(result.type_, "hexadecimal");
        } else {
            panic!("Expected hexadecimal, got something else");
        }
    }

    #[test]
    fn test_string_to_num_text() {
        let result = string_to_num("Hello, World!".to_string());
        if let DataT::Text(value) = result.data {
            assert_eq!(value, "Hello, World!");
            assert_eq!(result.type_, "text");
        } else {
            panic!("Expected text, got something else");
        }
    }

    #[test]
    fn test_pair_single_token() {
        let tokens = vec![
            DataHolder {
                type_: "float",
                data: DataT::Float(2.0),
            }
        ];
        let result = pair(tokens, Some(3.0));
        assert_eq!(result.len(), 1);
        if let (DataHolder { data: DataT::Float(a), .. }, DataHolder { data: DataT::Float(b), .. }) = &result[0] {
            assert_eq!(*a, 2.0);
            assert_eq!(*b, 3.0);
        } else {
            panic!("Expected pair of floats, got something else");
        }
    }

    #[test]
    fn test_pair_even_tokens() {
        let tokens = vec![
            DataHolder {
                type_: "float",
                data: DataT::Float(2.0),
            },
            DataHolder {
                type_: "float",
                data: DataT::Float(3.0),
            },
        ];
        let result = pair(tokens, Some(0.0));
        assert_eq!(result.len(), 1);
        if let (DataHolder { data: DataT::Float(a), .. }, DataHolder { data: DataT::Float(b), .. }) = &result[0] {
            assert_eq!(*a, 2.0);
            assert_eq!(*b, 3.0);
        } else {
            panic!("Expected pair of floats, got something else");
        }
    }

    #[test]
    fn test_pair_odd_tokens() {
        let tokens = vec![
            DataHolder {
                type_: "float",
                data: DataT::Float(2.0),
            },
            DataHolder {
                type_: "float",
                data: DataT::Float(3.0),
            },
            DataHolder {
                type_: "float",
                data: DataT::Float(4.0),
            },
        ];
        let result = pair(tokens, Some(0.0));
        assert_eq!(result.len(), 2);
        if let (DataHolder { data: DataT::Float(a), .. }, DataHolder { data: DataT::Float(b), .. }) = &result[0] {
            assert_eq!(*a, 2.0);
            assert_eq!(*b, 3.0);
        } else {
            panic!("Expected pair of floats, got something else");
        }
        if let (DataHolder { data: DataT::Float(a), .. }, DataHolder { data: DataT::Float(b), .. }) = &result[1] {
            assert_eq!(*a, 4.0);
            assert_eq!(*b, 0.0);
        } else {
            panic!("Expected pair of floats, got something else");
        }
    }

    #[test]
    fn test_clean_text() {
        let data = DataHolder {
            type_: "text",
            data: DataT::Text("Hello, World!".to_string()),
        };
        let result = _clean(data, Some(true));
        if let DataT::Text(value) = result.data {
            assert_eq!(value, "hello world!");
        } else {
            panic!("Expected cleaned text, got something else");
        }
    }

    #[test]
    fn test_clean_non_text() {
        let data = DataHolder {
            type_: "float",
            data: DataT::Float(3.14),
        };
        let result = _clean(data.clone(), None);
        assert_eq!(result, data); // Should return the same data as it was not text
    }
}
