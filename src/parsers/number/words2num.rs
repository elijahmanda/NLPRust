use fancy_regex::Regex;

use crate::parsers::number::*;
use crate::parsers::number::utils::*;

fn convert_to_number(tokens: Vec<DataHolder>, _data: &mut Data) -> Vec<DataHolder> {
    tokens
        .iter()
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let text_token = cleaned_token.text.unwrap();
                string_to_num(text_token)
            } else {
                t.clone()
            }
        })
        .collect::<Vec<DataHolder>>()
}

fn _word_to_number(tokens: Vec<DataHolder>, data: &mut Data) -> Vec<DataHolder> {
    let mut all_n = data.all_nums().clone();
    all_n.extend(data.informal_all().clone());
    tokens
        .iter()
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let text_token = cleaned_token.text.unwrap();
                if let Some(&val) = all_n.get(&text_token) {
                    DataHolder {
                        type_: "float",
                        float: Some(val),
                        text: None,
                        int: None,
                    }
                } else {
                    t.clone()
                }
            } else {
                t.clone()
            }
        })
        .collect::<Vec<DataHolder>>()
}

fn convert_suffixes(tokens: Vec<DataHolder>, data: &mut Data) -> Vec<DataHolder> {
    let pattern = Regex::new(&data.number_followed_by_suffix_regex()).unwrap();

    tokens
        .iter()
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let text_token = cleaned_token.text.unwrap_or_default();

                if let Some(caps) = pattern.captures(&text_token).unwrap() {
                    let num_str = caps.name("number").unwrap().as_str();
                    let suffix = caps.name("suffix").unwrap().as_str();

                    let num_holder = convert_to_number(vec![DataHolder {
                        type_: "text",
                        text: Some(num_str.to_string()),
                        int: None,
                        float: None,
                    }], data)[0]
                        .clone();

                    let mut num = num_holder.float.or(num_holder.int.map(|v| v as f64));

                    if let Some(multiplier) = data.get_suffix_value(suffix) {
                        num = num.map(|n| n * multiplier);
                    }

                    DataHolder {
                        type_: "float",
                        text: None,
                        int: None,
                        float: num,
                    }
                } else {
                    t.clone()
                }
            } else {
                t.clone()
            }
        })
        .collect::<Vec<DataHolder>>()
}


fn convert_ordinals(tokens: Vec<DataHolder>, data: &mut Data) -> Vec<DataHolder> {
    let pattern = Regex::new(&data.ordinal_numeral_regex()).unwrap();

    tokens
        .into_iter()  // Changed from into_par_iter to into_iter
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let text_token = cleaned_token.text.unwrap();

                if let Some(caps) = pattern.captures(&text_token).unwrap() {
                    let num_str = caps.name("number").unwrap().as_str();
                    let ordinal = caps.name("ordinal").unwrap().as_str().to_lowercase();

                    let mut num = convert_to_number(vec![DataHolder {
                        type_: "text",
                        text: Some(num_str.to_string()),
                        int: None,
                        float: None,
                    }], data)[0]
                        .clone()
                        .float
                        .or(Some(0.0))
                        .unwrap();

                    if data.ordinal_suffixes().contains(&ordinal) {
                        num = convert_to_number(vec![DataHolder {
                            type_: "text",
                            text: Some(num_str.to_string()),
                            int: None,
                            float: None,
                        }], data)[0]
                            .clone()
                            .float
                            .unwrap();
                    } else if let Some(multiplier) = data.ordinals().get(ordinal.as_str()) {
                        num *= multiplier;
                    }

                    DataHolder {
                        type_: "float",
                        text: None,
                        int: None,
                        float: Some(num),
                    }
                } else {
                    t
                }
            } else {
                t
            }
        })
        .collect::<Vec<DataHolder>>()
}

fn convert_supersubscript(tokens: Vec<DataHolder>, data: &mut Data) -> Vec<DataHolder> {
    tokens
        .iter()
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let text_token = cleaned_token.text.unwrap();
                let first_char = text_token.chars().nth(0).unwrap();
                if data.superscript_ones().contains_key(&first_char) {
                    let n = text_token.chars()
                    .map(|c| (*data.superscript_ones().get(&c).unwrap() as i64).to_string())
                    .collect::<Vec<String>>()
                    .join("");
                    DataHolder {
                        type_: "int",
                        int: Some(n.clone().parse::<i64>().unwrap()),
                        float: None,
                        text: None,
                    }
                } else if data.subscript_ones().contains_key(&first_char) {
                    let n = text_token.chars()
                    .map(|c| (*data.subscript_ones().get(&c).unwrap() as i64).to_string())
                    .collect::<Vec<String>>()
                    .join("");
                    DataHolder {
                        type_: "int",
                        int: Some(n.clone().parse::<i64>().unwrap()),
                        float: None,
                        text: None,
                    }
                } else if data.superscript_fractions().contains_key(&first_char) {
                    DataHolder {
                        type_: "float",
                        float: data.superscript_fractions().get(&first_char).copied(),
                        int: None,
                        text: None,
                    }
                } else {
                    t.clone()
                }
            } else {
                t.clone()
            }
        })
        .collect::<Vec<DataHolder>>()
}


struct ConversionPipe {
    data: Data,
}

impl ConversionPipe {
    pub fn new(data: &mut Data) -> Self {

        ConversionPipe { data: data.clone()}
    }

    pub fn call(&mut self, tokens: Vec<DataHolder>) -> Vec<DataHolder> {
        let mut tokens = tokens;
        tokens = convert_to_number(tokens, &mut self.data);
        tokens = convert_ordinals(tokens, &mut self.data);
        tokens = convert_suffixes(tokens, &mut self.data);
        tokens = convert_supersubscript(tokens, &mut self.data);
        tokens = _word_to_number(tokens, &mut self.data);
        tokens
    }
}

fn pair_tokens(tokens: Vec<f64>) -> Vec<Vec<f64>> {
    let mut build = Vec::new();
    let mut final_tokens = Vec::new();

    for token in tokens {
        if token <= 100.0 {
            build.push(token);
        } else {
            if !build.is_empty() {
                final_tokens.push(build);
                build = Vec::new();
            }
            final_tokens.push(vec![token]);
        }
    }

    if !build.is_empty() {
        final_tokens.push(build);
    }

    final_tokens
}

fn sum_nums(tokens: Vec<Vec<f64>>) -> Vec<f64> {
    let mut total = Vec::new();

    for token in tokens {
        if token.len() == 1 && token[0] % 1000.0 == 0.0 {
            total.push(token[0]);
        } else {
            let mut hundred = 0.0;
            for (j, &n) in token.iter().enumerate() {
                if j == 0 {
                    hundred += n;
                } else if n == 100.0 {
                    hundred *= 100.0;
                } else {
                    hundred += n;
                }
            }
            total.push(hundred);
        }
    }

    total
}

fn find_total(tokens: Vec<(f64, f64)>) -> f64 {
    let mut total = 0.0;

    for (num, multiplier) in tokens {
        if num == 0.0 {
            total += multiplier;
        } else if multiplier == 0.0 {
            total += num;
        } else {
            total += num * multiplier;
        }
    }

    total
}

pub fn try_power(n: Vec<DataHolder>, data: &mut Data) -> Option<f64> {
    let mut neg = 1.0;
    let mut n = n.clone();

    if n.len() == 3 {
        if !data.negatives().contains(&n[0].text.clone().unwrap()) {
            return None;
        }
        n.remove(0);
        neg = -1.0;
    }

    let res = ConversionPipe::new(data).call(n.clone());
    let mult = match res.get(0) {
        Some(holder) if holder.type_ == "float" => holder.float?,
        Some(holder) if holder.type_ == "int" => (holder.int?) as f64,
        _ => return None,
    };

    let num = match res.get(1) {
        Some(holder) if holder.type_ == "float" => holder.float?,
        Some(holder) if holder.type_ == "int" => (holder.int?) as f64,
        _ => return None,
    };

    let op: fn(f64, f64) -> f64 = if num > mult && data.informal_all().contains_key(&n[1].text.clone().unwrap().to_lowercase()) {
        add
    } else {
        mul
    };

    Some(op(neg * num, mult))
}

fn point_num(tokens: Vec<DataHolder>) -> f64 {
    let mut tokens = tokens;
    let mut last = 1.0;
    if let Some(last_token) = tokens.last() {
        if last_token.type_ == "float" && last_token.float.unwrap() > 9.0 {
            last = last_token.float.unwrap();
            tokens.pop();
        }
    }
    let point_index = tokens.iter().position(|x| x.type_ == "text" && x.text.as_deref() == Some("point"));
    let (whole_tokens, dec_tokens) = match point_index {
        Some(index) => (&tokens[..index], &tokens[index + 1..]),
        None => (&tokens[..], &[][..]),
    };
    let whole_numbers: Vec<f64> = whole_tokens.iter()
        .filter_map(|x| x.float)
        .collect();

    let paired = pair_tokens(whole_numbers);
    let summed = sum_nums(paired);

    let mut new_tokens = summed.clone();

    let paired = pair(&mut new_tokens, Some(0.0));
    let whole = find_total(paired);
    let mut dec_string: String = dec_tokens.iter()
        .map(|x| {
            if x.type_ == "float" {
                x.float.clone().unwrap().to_string()
            } else {
                ((x.int.clone().unwrap()) as f64).to_string()
            }
        }
        )
        .collect::<Vec<String>>()
        .join("");
    dec_string = format!("0.{dec_string}");
    let dec = dec_string.parse::<f64>().unwrap();
    let num = whole + dec;
    num * last
}

fn filter_tokens(tokens: &mut Vec<DataHolder>, unwanted: &str, leave_last: Option<bool>) -> Vec<DataHolder> {
    let leave_last = leave_last.unwrap_or(false);
    let mut i = (tokens.len() as isize) - 1;

    while i >= 0 {
        if let Some(token) = tokens.get(i as usize) {
            if let Some(text) = &token.text {
                if text.to_lowercase() == unwanted {
                    if !leave_last || tokens.iter().skip(i as usize).any(|t| t.text.as_ref().map_or(false, |s| s.to_lowercase() == unwanted)) {
                        tokens.remove(i as usize);
                    }
                }
            }
        }
        i -= 1;
    }
    tokens.clone()
}

pub fn words2num(dataholder: DataHolder, data: &mut Data) -> Option<f64> {
    if dataholder.type_ == "integer" || dataholder.type_ == "octal" || dataholder.type_ == "binary" || dataholder.type_ == "hexadecimal" {
        return Some(dataholder.int.clone().unwrap() as f64)
    }
    if dataholder.type_ == "float" {
        return dataholder.float.clone();
    }


    let mut number = dataholder.text?;
    
    // Tokenize and filter the number
    let tokens: Vec<DataHolder> = filter_tokens(&mut tokenize(&number)
        .iter()
        .map(|t| DataHolder {
            type_: "text",
            text: Some(t.clone()),
            int: None,
            float: None,
        })
        .collect(), "a", None);
    if tokens.len() == 1 {
        number = tokens.get(0)?.text.clone()?;
    }

    let num = ConversionPipe::new(data).call(vec![DataHolder {
        type_: "text",
        text: Some(number.clone()),
        int: None,
        float: None,
    }]);
    
    if num.len() == 1 {
        if let Some(DataHolder { float: Some(value), .. }) = num.get(0) {
            return Some(*value);
        } else if let Some(DataHolder { int: Some(value), .. }) = num.get(0) {
            return Some((*value) as f64);
        }
    }

    let cleaned = Pipe::normalize(number.to_lowercase(), data);
    let mut tokens: Vec<DataHolder> = filter_tokens(&mut tokenize(&cleaned)
            .iter()
            .map(|t| DataHolder {
                 type_: "text",
                 text: Some(t.clone()),
                 int: None,
                 float: None
             })
             .collect(), "a", None);
    let tokens = filter_tokens(&mut tokens, "and", Some(true));
    let tokens: Vec<String> = tokens.into_iter()
        .filter_map(|token| token.text)
        .collect();
    let original_tokens = tokens.clone();
    if tokens.len() > 1 && tokens.len() <= 3 {
        let value = try_power(
            tokens
            .clone()
            .iter()
            .map(|t| DataHolder {
                 type_: "text",
                 text: Some(t.clone()),
                 int: None,
                 float: None
             })
             .collect(), data);
             if value.is_some(){
                return Some(value?);
             }
    }
    let string_tokens = tokens.clone();
    let tokens: Vec<DataHolder> = ConversionPipe::new(data).call(tokens
            .clone()
            .iter()
            .map(|t| DataHolder {
                 type_: "text",
                 text: Some(t.clone()),
                 int: None,
                 float: None
             })
             .collect());
    if tokens.iter().any(|t| t.text.as_ref() == Some(&"point".to_string())) {
        let value = point_num(tokens.clone());
        return Some(value)
    }

    if tokens.len() == 2 && tokens.iter().all(|t| t.float.is_some() || t.int.is_some()) {
        let first = string_tokens.get(0)?.replace(" ", "");
        if first.len() == 5 && &first[1..2] == "," {
            if let Some(DataHolder { float: Some(val), .. }) = tokens.get(0) {
                return Some(val / 1000.0);
            }
        }
        let op: fn(f64, f64) -> f64 = if let (
            Some(DataHolder { float: Some(val1), .. }),        
            Some(DataHolder { float: Some(val2), .. })
        ) = (tokens.get(0), tokens.get(1)) {
            if val1 > val2 {
                add
            } else {
                mul
            }
        } else {
            mul
        };
        return Some(op(
            tokens.get(0)?.float?,
            tokens.get(1)?.float?,
        ));
    }

    let tokens: Vec<DataHolder> = ConversionPipe::new(data).call(original_tokens
        .clone()
        .iter()
        .map(|t| DataHolder {
             type_: "text",
             text: Some(t.clone()),
             int: None,
             float: None
         })
         .collect());
    if tokens.is_empty() {
        return None;
    }

    let mut tokens = tokens;
    if let Some(DataHolder { text: Some(s), .. }) = tokens.last() {
        if s == "." {
            tokens.pop();
        }
    }
    
    let mut points = None;
    let mut negative = 1.0;

    if let Some(DataHolder { text: Some(s), .. }) = tokens.get(0) {
        if data.negatives().contains(s) {
            tokens.remove(0);
            negative = -1.0;
        }
    }

    let mut operation: Option<fn(f64, f64) -> f64> = None;
    let mut fraction = 0.0;

    if tokens.len() >= 2 {
        if let Some(DataHolder { float: Some(value), .. }) = tokens.last() {
            if *value < 1.0 {
                if let Some(DataHolder { text: Some(s), .. }) = tokens.get(tokens.len() - 2) {
                    if s == "and" {
                        operation = Some(add);
                        fraction = *value;
                        tokens.pop();
                    }
                } else if let Some(DataHolder { float: Some(value), .. }) = tokens.get(tokens.len() - 2) {
                    operation = Some(mul);
                    fraction = *value;
                }
            }
        }
    }

    if operation == Some(mul) {
        fraction = if let Some(DataHolder { float: Some(value), .. }) = tokens.pop() {
            value
        } else {
            return None;
        };
    }
    
    tokens = filter_tokens(&mut tokens, "and",  None);
    
    if let Some(point_idx) = tokens.iter().position(|t| t.text.as_ref() == Some(&"point".to_string())) {
        points = Some("0.".to_string() + &tokens[(point_idx + 1)..].iter()
            .filter_map(|t| t.text.clone())
            .collect::<String>());
        tokens.truncate(point_idx);
    }
    
    let paired = pair_tokens(tokens.into_iter().filter_map(|t| t.float).collect());
    let summed = sum_nums(paired);

    let mut new_tokens: Vec<f64> = summed.clone();
    let paired = pair(&mut new_tokens, Some(0.0));
    let mut total = find_total(paired);

    if let Some(points) = points {
        total += points.parse::<f64>().unwrap_or(0.0);
    }

    if let Some(op) = operation {
        total = op(total, fraction);
    }
    
    Some(negative * total)
}



            