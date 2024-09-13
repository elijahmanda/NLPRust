use criterion::{criterion_group, criterion_main, Criterion, Bencher};

use nlp_rust::tokenizers::RegexTokenizer;

fn tokenizer_one_pattern(b: &mut Bencher<'_>) {
    let mut tokenizer = RegexTokenizer::new(Some(vec![(r"word", r"\w+")].to_vec()));
    let text = &"The quick brown fox jumped over the lazy brown dog ".repeat(1_000_000);
    b.iter(| | {
        tokenizer.tokenize(text, true);
    });
}

fn criterion_benckmark(c: &mut Criterion){
    c.bench_function("tokenizer_one_pattern", tokenizer_one_pattern);
}

criterion_group!(benches, criterion_benckmark);
criterion_main!(benches);
