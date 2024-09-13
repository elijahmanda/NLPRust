use fancy_regex::Regex;


fn starting_quotes() -> Vec<(Regex, &'static str)> {
    vec![
    (Regex::new(r#"^[\"']"#).unwrap(), r"``"),
    (Regex::new(r"(``)").unwrap(), r" $1 "),
    (Regex::new(r#"([ \(\[{<])(\"|'{2})"#).unwrap(), r"$1 `` ")]
}

fn punctuation() -> Vec<(Regex, &'static str)> {
    // punctuation
    vec![
    (Regex::new(r#"([,'])([^\d])"#).unwrap(), r"  $1   $2"),
    (Regex::new(r"([:,])$").unwrap(), r" $1 "),
    ]
}

// Pads parentheses
fn parens_brackets() -> Vec<(Regex, &'static str)> {
    vec![
    (Regex::new(r"[\]\[\(\)\{\}\<\>]").unwrap(), r"   \g<0>   ")
    ]
}

fn double_dashes() -> Vec<(Regex, &'static str)> {
    vec![
    (Regex::new(r"--").unwrap(), r"   --   ")
    ]
}

// ending quotes
fn ending_quotes() ->  Vec<(Regex, &'static str)> {
    vec![
    (Regex::new(r#"([\D])([\"'])(\d)"#).unwrap(), r"  $1   $2   $3  "),
    (Regex::new(r#"([\"'])([\D])"#).unwrap(), r"  $1   $2  "),
    (Regex::new(r#"([\D])([\"'])"#).unwrap(), r"  $1   $2  "),
    ]
}

struct TreebankWordTokenizer;

impl TreebankWordTokenizer {

    pub fn tokenize(sentence: &str) -> Vec<String> {
        let mut text: String = format!(" {sentence} ");
        for (regexp, substitution) in starting_quotes() {
            text = regexp.replace_all(&text, substitution).to_string();
        }
        for (regexp, substitution) in punctuation() {
            text = regexp.replace_all(&text, substitution).to_string();
        }

        // Handles parentheses.
        for (regexp, substitution) in parens_brackets() {
            text = regexp.replace_all(&text, substitution).to_string();
        }
        
        // Handles double dash.
        for (regexp, substitution) in double_dashes() {
            text = regexp.replace_all(&text, substitution).to_string();
        }

        // add extra space to make things easier
        text = format!(" {} ", text).to_string();

        for (regexp, substitution) in ending_quotes() {
            text = regexp.replace_all(&text, substitution).to_string();
        }

        text.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>()
    
    }
}

pub fn tokenize(sentence: &str) -> Vec<String> {
    TreebankWordTokenizer::tokenize(sentence)
}
