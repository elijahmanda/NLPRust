use std::collections::HashMap;
use regex::escape;
use fancy_regex::Regex;

use crate::parsers::number::{tokenize, Data};
use crate::parsers::number::constants::_REPLACEMENT;


fn _normalize_and_inner(numbers: Vec<Vec<String>>, data: &mut Data) -> Vec<Vec<String>> {
    let mut lasts: Vec<String> = Vec::new();
    lasts.extend(data.ands());
    lasts.extend(data.points());
    lasts.extend(data.negatives());
    let ands = data.ands();
    let zeros = data.zeros();
    let mut n_len: usize;
    let mut insertion: Vec<String>;
    let mut nums = numbers.clone();
    for i in 0..numbers.len() {
        let n = &nums[i];
        n_len = n.len();
        if n_len > 1 {
            let first = &n[0];
            let last = &n[n_len - 1];
            let second_last = &n[n_len - 2];
            if lasts.contains(&last) {
                insertion = [n[n_len - 1].clone()].to_vec();
                nums.insert(i + 1, insertion);
                nums[i].pop();
            } else if ands.contains(&second_last) && zeros.contains(&last) {
                insertion = [nums[i][n_len - 2].clone()].to_vec();
                nums.insert(i + 1, insertion);
                insertion = [nums[i][n_len - 1].clone()].to_vec();
                nums.insert(i + 2, insertion);
                nums[i].remove(n_len - 2);
                nums[i].pop();
            } else if ands.contains(&first) {
                insertion = [nums[i][0].clone()].to_vec();
                nums.insert(i, insertion);
                nums[i].remove(0);
            }
        }
    }
    numbers.to_vec()
}

pub fn normalize_and(numbers: Vec<Vec<String>>, data: &mut Data) -> Vec<Vec<String>> {
    let nums = _normalize_and_inner(numbers, data);
    let mut final_: Vec<Vec<String>> = Vec::new();
    for n in nums {
        if n.len() > 1 {
            final_.push(n);
        } else if check_valid(n[0].clone(), data) {
            final_.push(n);
        }
    }
    final_
}

pub fn check_valid(text: String, data: &mut Data) -> bool {
    let regex_pipes = vec![
        data.ordinal_numeral_regex(),
        data.number_followed_by_suffix_regex(),
        data.suffix_name_regex(),
        data.hex_regex(),
        data.oct_regex(),
        data.binary_regex(),
        data.any_number_regex(),
    ];
    for _pipe in regex_pipes {
        let re = Regex::new(&format!("^{_pipe}$")).unwrap();
        let result = re.is_match(&text);
        if result.is_ok() {
            return true;
        }
    }
    let valid: bool =  data.all_nums().contains_key(&text.to_lowercase()) || data.informal_all().contains_key(&text.to_lowercase());
    valid
}

pub fn recover_real_indices_and_match(
    text: String,
    nums: Vec<Vec<String>>,
    _data: Option<&mut Data>,
) -> (Vec<(String, (usize, usize))>, String) {
    let mut last_start = 0;
    let mut real: Vec<(String, (usize, usize))> = Vec::new();
    let mut temp_text = text.clone();
    for n in nums.iter() {
        let escaped: String = n
            .iter()
            .map(|s| escape(s).to_string())
            .collect::<Vec<String>>()
            .join(" ")
            .replace(" ", r"\s*[,\-]?\s*");
        let re = Regex::new(&escaped).unwrap();
        for cap in re.captures_iter(&temp_text[last_start..temp_text.len()]) {
            if let Some(mat) = cap.expect("No group").get(0) {
                let mut start = mat.start();
                let mut end = mat.end();
                start = start + last_start;
                end = end + last_start;
                real.push((mat.as_str().to_string(), (start, end))); 
                last_start = start;
                temp_text.replace_range(start..end, &_REPLACEMENT.repeat(end - start));
                break;
            }
        }
    }
    (real, temp_text)
}

fn _detokenize(tokens: Vec<String>) -> String {
    tokens.join(" ")
}

fn _normalize_hyphen(text: String, data: &mut Data) -> String {
    /*
    Normalize numbers such as: "twenty-five" to "twenty five", "seventy-nine" to "seventy nine" not 
    "re-enroll", "up-front", "made-up"
    */
    let mut rtokens: Vec<String> = Vec::new();
    let tokens = tokenize(&text);
    let hyphen_re: Regex = Regex::new(&data.hyphen()).unwrap();
    for n in &tokens {
        if hyphen_re.is_match(n).is_ok() {
            let ts: Vec<String> = n.split("-").map(|s| s.to_string()).collect();
            rtokens.extend(ts);
        } else {
            rtokens.push(n.to_string());
        }
    }
    _detokenize(rtokens)
}

fn _rep_commas(text: String, data: &mut Data) -> String {
    let binding = data.multiples();
    let multiples_map: HashMap<&String, &f64> = binding
        .iter()
        .filter(|(k, _)| {
            !data.ordinal_multiples().contains_key(*k) && (
                !data.suffixes_by_name().contains_key(*k) ||
                **k != "hundred".to_string()
            )
        })
        .collect::<HashMap<_, _>>();
   let multiples: String = multiples_map
       .iter()
       .map(|(&k, _)| k.to_string())
       .collect::<Vec<String>>()
       .join("|");
    /* orig_text = text
     Can only have a comma after a
     multiple of 1000
     */
    Regex::new(&format!(r"(?m)({multiples})\s?,"))
        .unwrap()
        .replace_all(&text, "$1")
        .to_string()
}

fn _possible_range(text: String) -> String {
    Regex::new(r"(\d)\-(\d)")
        .unwrap()
        .replace_all(&text, "$1  -   $2")
        .to_string()
}

fn _normalize(text: String, data: &mut Data) -> String {
    let mut new_text = text.clone();
    let suffixes: String = data._suffixes();
    //  `two    hundred` -> `two SPACE hundred` 
    new_text = Regex::new(r"\s{4,}")
        .unwrap()
        .replace_all(&new_text, " SPACE ")
        .to_string();
    new_text = Regex::new(r",\s*,")
        .unwrap()
        .replace_all(&new_text, " COMMA ")
        .to_string();
    
     /* `, ,` -> ` COMMA `
            
    replace commas
     5,000 -> 5000 at numbers
     million, -> thousand at
     multiples of 1000
     not => two, -> two
     */
    new_text = _rep_commas(new_text, data);
    /* Normalize where numbers may express a possible range eg 2-3; this may be 2 minus 3, or 2 to 3 to avoid false negatives we remove the hyphen.
     Normalize hyphen concatenated written numbers
     twenty-one -> twenty one
     */
    new_text = _normalize_hyphen(new_text, data);
    // two-two -> two two
    
    new_text = Regex::new(r"(\D)\-(\D)")
        .unwrap()
        .replace_all(&new_text, r"$1 $2")
        .to_string();
    /* 5-7 -> 5 7
     these could mean 5 to 7 or
     5 minus 7
     so we avoid interpreting
     this as a negative
     */
    new_text = Regex::new(r"(\d)\-(\d)")
        .unwrap()
        .replace_all(&new_text, r"$1 - $2")
        .to_string();
    // `3.^w` -> `3  .  SPACE `
    new_text = Regex::new(r"\.(\s+)")
        .unwrap()
        .replace_all(&new_text, "  .  SPACE ")
        .to_string();
    // `thousand.` -> `thousand .`
    new_text = Regex::new(r"\.(\D)")
        .unwrap()
        .replace_all(&new_text, " .  $1")
        .to_string();
    // ` h7` -> ` h 5`
    if !data.config.bounded_numbers.unwrap() {
        new_text = Regex::new(r"(?<=(\s))([^\-\+\.\d])(\d)")
            .unwrap()
            .replace_all(&new_text, "$1  $2   $3")
            .to_string();
    }
    new_text = _possible_range(new_text);
    // 5^10 -> 5 ^ 10
    // 5'272' -> 5'272 '
    new_text = Regex::new(r"([`',\.])(\D)")
        .unwrap()
        .replace_all(&new_text, r" $1$2")
        .to_string();
    new_text = Regex::new(r"(\d)([',])(\d{4,})")
         .unwrap()
         .replace_all(&new_text, " $1 $2 $3")
         .to_string();
    new_text = Regex::new(r"(?<!\d[eE])([-+])")
        .unwrap()
        .replace_all(&new_text, " SPACE  $1")
        .to_string();
    new_text = Regex::new(&format!(r"\d(^[eE',\d]|{suffixes})(?=>[\W\b])"))
        .unwrap()
        .replace_all(&new_text, " SPACE $1")
        .to_string();
    new_text = Regex::new(r"([\-\+])([a-df-zA-DF-Z])")
        .unwrap()
        .replace_all(&new_text, "$1 $2")
        .to_string(); 
    new_text
}


pub struct Pipe;

impl Pipe {
        
    pub fn normalize(text: String, data: &mut Data) -> String {
        let mut txt = _normalize(text, data);
        txt = _detokenize(
            txt
            .split_whitespace()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        );
        txt.trim().to_string()
    }
    
}