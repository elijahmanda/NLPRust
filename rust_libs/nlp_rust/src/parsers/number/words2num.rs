use std::collections::HashMap;
use fancy_regex::Regex;
use rayon::prelude::*;

use crate::parsers::number::*;
use crate::parsers::number::utils::*;

fn convert_to_number(tokens: Vec<DataHolder>, data: &mut Data) -> Vec<DataHolder> {
    tokens
        .par_iter()
        .map(|t| {
            match t.data {
                DataT::Text(ref text) => {
                        let text_token = text.clone();
                        string_to_num(text_token)
                    },
                _ => t.clone(),
            }
        })
        .collect::<Vec<DataHolder>>()
}

fn _word_to_number(tokens: Vec<DataHolder>, data: &mut Data) -> Vec<DataHolder> {
    let mut all_n = data.all_nums().clone();
    all_n.extend(data.informal_all().clone());

    tokens
        .par_iter()
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let mut text_token: String = match cleaned_token.data {
                    DataT::Text(ref text) => {
                        text.clone()
                    }
                };

                if let Some(&val) = all_n.get(&text_token) {
                    DataHolder {
                        type_: "integer",
                        data: DataT::Int(val as i64),
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

fn convert_suffixes(tokens: Vec<DataHolder>, data: &mut Data) -> Vec<DataHolder> {
    let pattern = Regex::new(&data.number_followed_by_suffix_regex()).unwrap();

    tokens
        .par_iter()
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let mut text_token: String = match cleaned_token.data {
                    DataT::Text(ref text) => {
                        text.clone()
                    }
                };

                if let Some(caps) = pattern.captures(&text_token).unwrap() {
                    let num_str = caps.name("number").unwrap().as_str();
                    let suffix = caps.name("suffix").unwrap().as_str();

                    let num_holder = convert_to_number(vec![DataHolder {
                        type_: "text",
                        data: DataT::Text(num_str.to_string()),
                    }], data)[0];

                    let mut num = num_holder.data;

                    if let Some(multiplier) = data.get_suffix_value(suffix) {
                        num = match num {
                            DataT::Float(ref val) => DataT::Float(val * multiplier),
                            DataT::Int(ref val) => DataT::Int((val as f64 * multiplier) as i64),
                            _ => num,
                        };
                    }

                    DataHolder {
                        type_: "float",
                        data: num,
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

fn convert_ordinals(tokens: Vec<DataHolder>, data: &mut Data) -> Vec<DataHolder> {
    let pattern = Regex::new(&data.ordinal_numeral_regex()).unwrap();

    tokens
        .into_par_iter()
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let mut text_token = match cleaned_token.data {
                    DataT::Text(ref text) => {
                        let text_token = text.clone();
                    }
                };

                if let Some(caps) = pattern.captures(text_token).unwrap() {
                    let num_str = caps.name("number").unwrap().as_str();
                    let ordinal = caps.name("ordinal").unwrap().as_str().to_lowercase();

                    let mut num = convert_to_number(vec![DataHolder {
                        type_: "text",
                        data: DataT::Text(num_str.to_string()),
                    }], data)[0].data;

                    if data.ordinal_suffixes().contains(&ordinal) {
                        num = convert_to_number(vec![DataHolder {
                            type_: "text",
                            data: DataT::Text(num_str.to_string()),
                        }], data)[0].data;
                    } else if let Some(multiplier) = data.ordinals().get(ordinal.as_str()) {
                        num = match num {
                            DataT::Float(ref val) => DataT::Float(val * multiplier),
                            DataT::Int(ref val) => DataT::Int((val as f64 * multiplier) as i64),
                            _ => num,
                        };
                    }

                    DataHolder {
                        type_: "float",
                        data: num,
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
        .par_iter()
        .map(|t| {
            if t.type_ == "text" {
                let cleaned_token = _clean(t.clone(), Some(true));
                let mut text_token = match cleaned_token.data {
                    DataT::Text(ref text) => {
                        let text_token = text.clone();
                    }
                };

                if let Some(val) = text_token.chars().map(|c| data.superscript_ones().get(&c)).collect::<Option<String>>() {
                    DataHolder {
                        type_: "integer",
                        data: DataT::Int(val.parse::<i64>().unwrap()),
                    }
                } else if let Some(val) = text_token.chars().map(|c| data.subscript_ones().get(&c)).collect::<Option<String>>() {
                    DataHolder {
                        type_: "integer",
                        data: DataT::Int(val.parse::<i64>().unwrap()),
                    }
                } else if let Some(&val) = data.superscript_fractions().get(text_token) {
                    DataHolder {
                        type_: "float",
                        data: DataT::Float(val),
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

struct ConversionPipe {
    data: Data,
    pipes: Vec<Box<dyn Fn(Vec<DataHolder>, &mut Data) -> Vec<DataHolder>>>,
}

impl ConversionPipe {
    pub fn new(data: &mut Data) -> Self {
        let pipes: Vec<Box<dyn Fn(Vec<DataHolder>, &mut Data) -> Vec<DataHolder>>> = vec![
            Box::new(convert_to_number) as Box<dyn Fn(Vec<DataHolder>, &mut Data) -> Vec<DataHolder>>,
            Box::new(convert_ordinals) as Box<dyn Fn(Vec<DataHolder>, &mut Data) -> Vec<DataHolder>>,
            Box::new(convert_suffixes) as Box<dyn Fn(Vec<DataHolder>, &mut Data) -> Vec<DataHolder>>,
            Box::new(convert_supersubscript) as Box<dyn Fn(Vec<DataHolder>, &mut Data) -> Vec<DataHolder>>,
            Box::new(_word_to_number) as Box<dyn Fn(Vec<DataHolder>, &mut Data) -> Vec<DataHolder>>,
        ];

        ConversionPipe { data: data.clone(), pipes }
    }

    pub fn call(&mut self, tokens: Vec<DataHolder>) -> Vec<DataHolder> {
        let mut tokens = tokens;
        for pipe in &self.pipes {
            tokens = pipe(tokens, &mut self.data);
        }
        tokens
    }
}

fn _pair_tokens(tokens: Vec<f64>) -> Vec<Vec<f64>> {
    let mut build = Vec::new();
    let mut final_ = Vec::new();

    for token in tokens {
        if token <= 100.0 {
            build.push(token);
        } else {
            if !build.is_empty() {
                final_.push(build.clone());
            }
            build.clear();
            final_.push(vec![token]);
        }
    }
    if !build.is_empty() {
        final_.push(build);
    }
    final_
}

fn _sum_nums(tokens: Vec<Vec<f64>>) -> f64 {
    let mut total = Vec::new();
    let mut hundred = 0.0;

    for token in tokens {
        if token.len() == 1 && token[0] % 1000.0 == 0.0 {
            total.push(token[0]);
        } else {
            hundred = 0.0;
            for (j, n) in token.iter().enumerate() {
                if j == 0 {
                    hundred += n;
                } else if *n == 100.0 {
                    hundred *= 100.0;
                } else {
                    hundred += n;
                }
            }
            total.push(hundred);
        }
    }
    total.into_iter().sum()
}

fn _find_total(tokens: Vec<(f64, f64)>) -> f64 {
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


pub fn ensure_iterable(obj: &dyn std::any::Any) -> Vec<String> {
    if let Some(iterable) = obj.downcast_ref::<Vec<String>>() {
        return iterable.clone();
    }
    vec![obj.to_string()]
}

pub fn try_power(n: Vec<String>, data: &mut Data) -> Option<f64> {
    let mut neg = 1.0;
    if n.len() == 3 {
        if data.negatives().contains(&n[0]) {
            return None;
        }
        let mut n = n;
        n.remove(0);
        neg = -1.0;
    }
    let res = ConversionPipe::new(data).call(n);
    let mult = match &res.get(0) {
        Some(holder) => match &holder.data {
            DataT::Float(value) => value,
            _ => return None,
        },
        None => return None,
    };
    let num = match &res.get(1) {
        Some(holder) => match &holder.data {
            DataT::Float(value) => value,
            _ => return None,
        },
        None => return None,
    };

    let op: fn(f64, f64) -> f64 = if num > mult && data.informal_all().contains_key(&n[1].to_lowercase()) {
        add
    } else {
        mul
    };

    Some(op(neg * num, mult))
}

fn point_num(tokens: Vec<DataHolder>) -> f64 {
    let mut last = 1.0;
    let last_token = tokens.last().unwrap();
    if let DataT::Float(val) = &last_token.data {
        if val > 9.0 {
            last = val;
            tokens.pop();
        }
    }

    let point_index = tokens.iter().position(|x| matches!(x.data, DataT::Text(s) if s == "point"));
    let (whole, dec) = match point_index {
        Some(index) => tokens.split_at(index),
        None => (tokens, vec![]),
    };

    let whole: Vec<f64> = whole.iter()
        .filter_map(|x| match &x.data {
            DataT::Float(val) => Some(val),
            _ => None,
        })
        .filter(|&x| x != "and".parse::<f64>().unwrap_or(0.0))
        .collect();
    
    let paired = _pair_tokens(whole);
    let summed = _sum_nums(paired);
    
    let new_tokens: Vec<f64> = summed.into_iter()
        .filter_map(|x| match x {
            x if x.is_numeric() => Some(x),
            _ => None,
        })
        .collect();

    let paired = pair(new_tokens, Some(0.0));
    let whole = _find_total(paired);

    let dec: f64 = dec.into_iter()
        .filter_map(|x| match &x.data {
            DataT::Text(s) => s.parse::<f64>().ok(),
            _ => None,
        })
        .collect::<String>()
        .parse::<f64>()
        .unwrap_or(0.0);

    let num = whole + dec;
    num * last
}

fn filter_tokens(tokens: &mut Vec<DataHolder>, unwanted: &str, leave_last: Option<bool>) -> Vec<DataHolder> {
    let mut i = tokens.len() as isize - 1;
    let leave_last = leave_last.unwrap_or(false);

    while i >= 0 {
        let token = tokens.get(i as usize);
        if token.is_none() {
            break;
        }
        if let DataT::Text(x) = &token.unwrap().data {
            if x.to_lowercase() == unwanted {
                if leave_last {
                    if tokens.iter().skip(i as usize).any(|t| match &t.data {
                        DataT::Text(s) if s.to_lowercase() == unwanted => true,
                        _ => false,
                    }) {
                        tokens.remove(i as usize);
                    }
                } else {
                    tokens.remove(i as usize);
                }
            }
        }
        i -= 1;
    }
    tokens.to_vec()
}

pub fn words2num(dataholder: DataHolder, data: &mut Data) -> Option<f64> {
    if dataholder.type_ != "text" {
        return None;
    }
    
    let mut number = if let DataT::Text(text) = dataholder.data {
        text
    } else {
        return None;
    };
    
    let tokens: Vec<DataHolder> = _filter(
        tokenize(&number)
            .iter()
            .map(|t| DataHolder {
                type_: "text",
                data: DataT::Text(t.clone()),
            })
            .collect(),
        "a",
    );
    
    if tokens.len() == 1 {
        number = if let DataT::Text(text) = tokens[0].data {
            text
        } else {
            return None;
        };
    }
    
    let mut num = ConversionPipe::new(data).call(vec![DataHolder {
        type_: "text",
        data: DataT::Text(number),
    }]);
    
    if num.len() == 1 {
        if let DataT::Float(ref value) | DataT::Int(ref value) = num[0].data {
            return Some(value);
        }
    }
    
    let cleaned = Pipe::normalize(number.to_lowercase(), data);
    let tokens: Vec<DataHolder> = _filter(tokenize(&cleaned), "a");
    let tokens = _filter(tokens, "and", Some(true));
    let tokens: Vec<String> = tokens.into_iter()
        .filter_map(|token| {
            if let DataT::Text(text) = token.data {
                Some(_clean(text, true))
            } else {
                None
            }
        })
        .collect();

    let og_tokens = tokens.clone();
    if tokens.len() > 1 && tokens.len() <= 3 {
        if let Some(value) = try_power(tokens.clone(), data) {
            return Some(value);
        }
    }
    
    let string_tokens = tokens.clone();
    let tokens: Vec<DataHolder> = ConversionPipe::new(data).call(tokens);

    if tokens.iter().any(|t| match &t.data {
        DataT::Text(text) if text == "point" => true,
        _ => false,
    }) {
        if let Some(value) = _point_num(tokens) {
            return Some(value);
        }
    }

    if tokens.len() == 2 && tokens.iter().all(|t| matches!(t.data, DataT::Float(_) | DataT::Int(_))) {
        let first = if let DataT::Text(text) = string_tokens[0].data {
            text.replace(" ", "")
        } else {
            return None;
        };
        if first.len() == 5 && &first[1..2] == "," {
            if let DataT::Float(mut val) = tokens[0].data {
                val /= 1000.0;
            }
        }
        let op: fn(f64, f64) -> f64 = if let DataT::Float(val1) = tokens[0].data {
            if let DataT::Float(val2) = tokens[1].data {
                if val1 > val2 {
                    add
                } else {
                    mul
                }
            } else {
                mul
            }
        } else {
            mul
        };
        return Some(op(
            if let DataT::Float(val) = tokens[0].data {
                val
            } else {
                return None;
            },
            if let DataT::Float(val) = tokens[1].data {
                val
            } else {
                return None;
            }
        ));
    }
    
    let tokens: Vec<DataHolder> = ConversionPipe::new(data).call(og_tokens);
    if tokens.is_empty() {
        return None;
    }

    let mut tokens = tokens;
    if let DataT::Text(s) = tokens.last().unwrap().data {
        if s == "." {
            tokens.pop();
        }
    }
    
    let mut points = None;
    let mut negative = 1.0;
    
    if let DataT::Text(s) = tokens.get(0).unwrap().data {
        if data.negatives.contains(s) {
            tokens.remove(0);
            negative = -1.0;
        }
    }

    let mut operation: Option<fn(f64, f64) -> f64> = None;
    let mut fraction = 0.0;

    if tokens.len() >= 2 {
        if let DataT::Float(value) = tokens.last().unwrap().data {
            if value < 1.0 {
                if let DataT::Text(s) = tokens[tokens.len() - 2].data {
                    if s == "and" {
                        operation = Some(add);
                        fraction = value;
                        tokens.pop();
                    }
                } else if let DataT::Float(value) = tokens[tokens.len() - 2].data {
                    operation = Some(mul);
                    fraction = value;
                }
            }
        }
    }

    if operation == Some(mul) {
        fraction = if let DataT::Float(value) = tokens.pop().unwrap().data {
            value
        } else {
            return None;
        };
    }
    
    tokens = _filter(tokens, "and");
    
    if let Some(point_idx) = tokens.iter().position(|t| match &t.data {
        DataT::Text(text) if text == "point" => true,
        _ => false,
    }) {
        points = Some("0.".to_string() + &tokens[(point_idx + 1)..].iter()
            .filter_map(|t| match &t.data {
                DataT::Text(ref text) => Some(text.clone()),
                _ => None,
            })
            .collect::<String>());
        tokens.truncate(point_idx);
    }
    
    let paired = _pair_tokens(tokens);
    let summed = _sum_nums(paired);

    let mut new_tokens = vec![];
    for token in summed {
        if let DataT::Float(val) = token {
            new_tokens.push(val);
        }
    }

    let paired = pair(new_tokens, Some(0.0));
    let mut total = _find_total(paired);

    if let Some(points) = points {
        total += points.parse::<f64>().unwrap_or(0.0);
    }

    if let Some(op) = operation {
        total = op(total, fraction);
    }
    
    Some(negative * total)
}