use nlp_rust::parsers::number::*;



fn test_tokenization() {
    let mut text = "10+7 hsjs fifty-six point five 1,287 90'672";
    let config = Config::default();
    let mut data = Data::new(config);
    let binding = Pipe::normalize(text.to_string(), &mut data);
    text = &binding;
    let tokens = tokenize(text);
    for token in tokens {
        println!("{}", token);
    }
}

fn text_to_num(text: &str, data: &mut Data) -> Option<f64> {
    let holder = DataHolder { type_: "text", text: Some(text.to_string()), int: None, float: None};
    words2num(holder, data)
}

fn test_words2num() {
    let text = "¾";
    let config = Config::default();
    let mut data = Data::new(config);
    let num = text_to_num(text, &mut data);
    println!("{}", num.unwrap());
}

fn main() {
    test_tokenization();
    test_words2num();
}
