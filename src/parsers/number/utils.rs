use fancy_regex::Regex;

#[derive(Debug, PartialEq, Clone)]
pub struct DataHolder {
    pub type_: &'static str,
    pub text: Option<String>,
    pub int: Option<i64>,
    pub float: Option<f64>,
}

pub fn add(x: f64, y: f64) -> f64 {
    x + y
}

pub fn mul(x: f64, y: f64) -> f64 {
    x * y
}
pub fn to_base(mut x: u32, radix: u32) -> u32 {
    let mut result = vec![];
    loop {
        let m = x % radix;
        x /= radix;
        result.push(std::char::from_digit(m, radix).unwrap());
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

    if string_lower.starts_with("0b") {
        match i64::from_str_radix(&string[2..], 2) {
            Ok(num) => Ok((num, "binary")),
            Err(_) => Err("Invalid binary number"),
        }
    } else if string_lower.starts_with("0o") {
        match i64::from_str_radix(&string[2..], 8) {
            Ok(num) => Ok((num, "octal")),
            Err(_) => Err("Invalid octal number"),
        }
    } else if string_lower.starts_with("0x") {
        match i64::from_str_radix(&string[2..], 16) {
            Ok(num) => Ok((num, "hexadecimal")),
            Err(_) => Err("Invalid hexadecimal number"),
        }
    } else {
        match string.parse::<i64>() {
            Ok(num) => Ok((num, "integer")),
            Err(_) => Err("Invalid decimal number"),
        }
    }
}

pub fn string_to_num(string: String) -> DataHolder {
    let mut type_: &'static str = "text";
    let mut int: Option<i64> = None;
    let mut float: Option<f64> = None;
    let text: Option<String> = Some(string.clone());

    if let Ok((parsed_int, parsed_type)) = parse_number(&string) {
        int = Some(parsed_int);
        type_ = parsed_type;
    } else if let Ok(parsed_float) = string.parse::<f64>() {
        float = Some(parsed_float);
        type_ = "float";
    }

    match type_ {
        "float" => DataHolder { type_, text: None, int: None, float },
        "integer" | "octal" | "binary" | "hexadecimal" => DataHolder { type_, text: None, int, float: None },
        _ => DataHolder { type_: "text", text, int: None, float: None },
    }
}


pub fn pair(tokens: &mut Vec<f64>, holder: Option<f64>) -> Vec<(f64, f64)> {
    let holder = holder.unwrap_or(0.0);
    let mut tk_len = tokens.len();

    // If there's only one token, return a tuple with the holder value
    if tk_len == 1 {
        return vec![(tokens[0], holder)];
    }

    // If there are no tokens, return an empty vector
    if tk_len == 0 {
        return Vec::new();
    }

    // If the length is odd, append the holder to make it even
    if tk_len % 2 != 0 {
        tokens.push(holder);
        tk_len += 1;
    }

    let mut new_tokens = Vec::new();
    let mut n = 0;

    // Pair the tokens, ensuring that n + 1 is within bounds
    while n <= tk_len - 2 {
        new_tokens.push((tokens[n], tokens[n + 1]));
        n += 2;
    }

    new_tokens
}


pub fn _clean(data: DataHolder, lower: Option<bool>) -> DataHolder {
    match data.text {
        Some(text) => {
            let mut cleaned_text = text.clone();
            cleaned_text = Regex::new(r#"[\,'_]"#)
                .unwrap()
                .replace_all(&cleaned_text, "")
                .to_string();
            if lower.unwrap_or(false) {
                cleaned_text = cleaned_text.to_lowercase();
            }
            DataHolder {
                type_: "text",
                text: Some(cleaned_text),
                int: None,
                float: None,
            }
        }
        None => data,
    }
}
