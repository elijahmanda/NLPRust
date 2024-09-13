// src/tokenizer/utils.rs

pub fn text_span_replace(text: &str, replacement: &str, start: usize, end: usize) -> String {
    format!("{}{}{}", &text[0..start], replacement, &text[end..])
}

// Any other utility functions you have...
