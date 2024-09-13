use std::collections::HashMap;

use crate::tokenizers::{DEFAULT_RE_FLAGS, EXTENDED, IGNORECASE};
use crate::utils::regex_::{RegexProcessor};
use crate::parsers::number::config::Config;


const B_LEFT: &str = r"(?<![a-zA-Z_])";
const B_RIGHT: &str = r"(?![a-zA-Z_])";

#[derive(Clone)]
pub struct Data {
    pub config: Config,
    pub flags: HashMap<String, String>,
}

impl Data {
    pub fn new(config: Config) -> Self {
        let data = Self {
            config,
            flags: HashMap::new(),
        };
        data
    }

    pub fn default_re_flags(&mut self) -> String {
        DEFAULT_RE_FLAGS.to_string()
    }

    pub fn a(&mut self) -> Vec<String> {
        vec!["a".to_string()]
    }

    pub fn ands(&mut self) -> Vec<String> {
        vec!["and".to_string()]
    }

    pub fn points(&mut self) -> Vec<String> {
        vec!["point".to_string(), ".".to_string()]
    }

    pub fn negatives(&mut self) -> Vec<String> {
        vec!["negative".to_string(), "neg".to_string(), "minus".to_string()]
    }

    pub fn zeros(&mut self) -> Vec<String> {
        vec!["zero".to_string(), "0".to_string()]
    }

    pub fn ones(&mut self) -> HashMap<String, f64> {
        let mut ones = HashMap::new();
        ones.insert("zero".to_string(), 0.0);
        ones.insert("one".to_string(), 1.0);
        ones.insert("two".to_string(), 2.0);
        ones.insert("three".to_string(), 3.0);
        ones.insert("four".to_string(), 4.0);
        ones.insert("five".to_string(), 5.0);
        ones.insert("six".to_string(), 6.0);
        ones.insert("seven".to_string(), 7.0);
        ones.insert("eight".to_string(), 8.0);
        ones.insert("nine".to_string(), 9.0);
        ones.extend(self.ordinal_ones());
        ones
    }

    pub fn ordinal_ones(&mut self) -> HashMap<String, f64> {
        let mut ordinal_ones = HashMap::new();
        ordinal_ones.insert("first".to_string(), 1.0);
        ordinal_ones.insert("second".to_string(), 2.0);
        ordinal_ones.insert("third".to_string(), 3.0);
        ordinal_ones.insert("fourth".to_string(), 4.0);
        ordinal_ones.insert("fifth".to_string(), 5.0);
        ordinal_ones.insert("sixth".to_string(), 6.0);
        ordinal_ones.insert("seventh".to_string(), 7.0);
        ordinal_ones.insert("eighth".to_string(), 8.0);
        ordinal_ones.insert("ninth".to_string(), 9.0);
        ordinal_ones
    }
    
    pub fn multiples(&mut self) -> HashMap<String, f64> {
        let mut multiples = HashMap::new();
        multiples.insert("hundred".to_string(), 100.0);
        multiples.insert("thousand".to_string(), 1000.0);
        multiples.insert("million".to_string(), 1000000.0);
        multiples.insert("billion".to_string(), 1000000000.0);
        multiples.insert("trillion".to_string(), 1000000000000.0);
        multiples.insert("quadrillion".to_string(), 1000000000000000.0);
        multiples.insert("quintillion".to_string(), 1e+18);
        multiples.insert("sextillion".to_string(), 1e+21);
        multiples.insert("septillion".to_string(), 1e+24);
        multiples.insert("octillion".to_string(), 1e+27);
        multiples.insert("nonillion".to_string(), 1e+30);
        multiples.insert("decillion".to_string(), 1e+33);
        multiples.insert("undecillion".to_string(), 1e+36);
        multiples.insert("duodecillion".to_string(), 1e+39);
        multiples.insert("tredecillion".to_string(), 1e+42);
        multiples.insert("quattuordecillion".to_string(), 1e+45);
        multiples.insert("quinquadecillion".to_string(), 1e+48);
        multiples.insert("sedecillion".to_string(), 1e+51);
        multiples.insert("septendecillion".to_string(), 1e+54);
        multiples.insert("octodecillion".to_string(), 1e+57);
        multiples.insert("novendecillion".to_string(), 1e+60);
        multiples.insert("vigintillion".to_string(), 1e+63);
        multiples.insert("unvigintillion".to_string(), 1e+66);
        multiples.insert("uuovigintillion".to_string(), 1e+69);
        multiples.insert("tresvigintillion".to_string(), 1e+72);
        multiples.insert("quattuorvigintillion".to_string(), 1e+75);
        multiples.insert("quinquavigintillion".to_string(), 1e+78);
        multiples.insert("qesvigintillion".to_string(), 1e+81);
        multiples.insert("septemvigintillion".to_string(), 1e+84);
        multiples.insert("octovigintillion".to_string(), 1e+87);
        multiples.insert("novemvigintillion".to_string(), 1e+90);
        multiples.insert("trigintillion".to_string(), 1e+93);
        multiples.insert("untrigintillion".to_string(), 1e+96);
        multiples.insert("duotrigintillion".to_string(), 1e+99);
        multiples.insert("trestrigintillion".to_string(), 1e+102);
        multiples.insert("quattuortrigintillion".to_string(), 1e+105);
        multiples.insert("quinquatrigintillion".to_string(), 1e+108);
        multiples.insert("sestrigintillion".to_string(), 1e+111);
        multiples.insert("septentrigintillion".to_string(), 1e+114);
        multiples.insert("octotrigintillion".to_string(), 1e+117);
        multiples.insert("noventrigintillion".to_string(), 1e+120);
        multiples.insert("quadragintillion".to_string(), 1e+123);
        multiples.extend(self.ordinal_multiples());
        multiples
    }
    
    pub fn teens_and_ten(&mut self) -> HashMap<String, f64> {
        let mut teens_and_tens = HashMap::new();
        teens_and_tens.insert("ten".to_string(), 10.0);
        teens_and_tens.insert("eleven".to_string(), 11.0);
        teens_and_tens.insert("twelve".to_string(), 12.0);
        teens_and_tens.insert("thirteen".to_string(), 13.0);
        teens_and_tens.insert("fourteen".to_string(), 14.0);
        teens_and_tens.insert("fifteen".to_string(), 15.0);
        teens_and_tens.insert("sixteen".to_string(), 16.0);
        teens_and_tens.insert("seventeen".to_string(), 17.0);
        teens_and_tens.insert("eighteen".to_string(), 18.0);
        teens_and_tens.insert("nineteen".to_string(), 19.0);
        teens_and_tens.extend(self.ordinal_teens_and_ten());
        teens_and_tens
    }
    
    pub fn ordinal_teens_and_ten(&mut self) -> HashMap<String, f64> {
        let mut ordinal_teens_and_ten = HashMap::new();
        ordinal_teens_and_ten.insert("tenth".to_string(), 10.0);
        ordinal_teens_and_ten.insert("eleventh".to_string(), 11.0);
        ordinal_teens_and_ten.insert("twelfth".to_string(), 12.0);
        ordinal_teens_and_ten.insert("thirteenth".to_string(), 13.0);
        ordinal_teens_and_ten.insert("fourteenth".to_string(), 14.0);
        ordinal_teens_and_ten.insert("fifteenth".to_string(), 15.0);
        ordinal_teens_and_ten.insert("sixteenth".to_string(), 16.0);
        ordinal_teens_and_ten.insert("seventeenth".to_string(), 17.0);
        ordinal_teens_and_ten.insert("eighteenth".to_string(), 18.0);
        ordinal_teens_and_ten.insert("nineteenth".to_string(), 19.0);
        ordinal_teens_and_ten
    }
    
    pub fn ordinal_tens(&mut self) -> HashMap<String, f64> {
        let mut ordinal_tens = HashMap::new();
        ordinal_tens.insert("twentieth".to_string(), 20.0);
        ordinal_tens.insert("thirtieth".to_string(), 30.0);
        ordinal_tens.insert("fortieth".to_string(), 40.0);
        ordinal_tens.insert("fiftieth".to_string(), 50.0);
        ordinal_tens.insert("sixtieth".to_string(), 60.0);
        ordinal_tens.insert("seventieth".to_string(), 70.0);
        ordinal_tens.insert("eightieth".to_string(), 80.0);
        ordinal_tens.insert("ninetieth".to_string(), 90.0);
        ordinal_tens
    }
    
    pub fn tens(&mut self) -> HashMap<String, f64> {
        let mut tens = HashMap::new();
        tens.insert("twenty".to_string(), 20.0);
        tens.insert("thirty".to_string(), 30.0);
        tens.insert("forty".to_string(), 40.0);
        tens.insert("fifty".to_string(), 50.0);
        tens.insert("sixty".to_string(), 60.0);
        tens.insert("seventy".to_string(), 70.0);
        tens.insert("eighty".to_string(), 80.0);
        tens.insert("ninety".to_string(), 90.0);
        tens.extend(self.ordinal_tens());
        tens
    }
    
    pub fn ordinal_multiples(&mut self) -> HashMap<String, f64> {
        let mut ordinal_multiples = HashMap::new();
        ordinal_multiples.insert("hundredth".to_string(), 100.0);
        ordinal_multiples.insert("thousandth".to_string(), 1000.0);
        ordinal_multiples.insert("millionth".to_string(), 1_000_000.0);
        ordinal_multiples.insert("billionth".to_string(), 1_000_000_000.0);
        ordinal_multiples.insert("trillionth".to_string(), 1_000_000_000_000.0);
        ordinal_multiples.insert("quadrillionth".to_string(), 1e24);
        ordinal_multiples.insert("quintillionth".to_string(), 1e30);
        ordinal_multiples.insert("sextillionth".to_string(), 1e36);
        ordinal_multiples.insert("septillionth".to_string(), 1e42);
        ordinal_multiples.insert("octillionth".to_string(), 1e48);
        ordinal_multiples.insert("nonillionth".to_string(), 1e54);
        ordinal_multiples.insert("decillionth".to_string(), 1e60);
        ordinal_multiples.extend(self.suffixes_by_name());
        ordinal_multiples
    }
    
    pub fn suffixes(&mut self) -> HashMap<String, f64> {
        let mut suffixes = HashMap::new();
        suffixes.insert("y".to_string(), 1e-24); //  Yocto
        suffixes.insert("z".to_string(), 1e-21); //  Zepto
        suffixes.insert("a".to_string(), 1e-18); //  Atto
        suffixes.insert("f".to_string(), 1e-15); //  Femto
        suffixes.insert("p".to_string(), 1e-12); //  Pico
        suffixes.insert("n".to_string(), 1e-9); //  Nano
        //  suffixes.insert(chr(181), 1e-6); //  Micro μ
        //  suffixes.insert(chr(956), 1e-6); //  Micro
        suffixes.insert("m".to_string(), 0.001); //  Milli
        suffixes.insert("c".to_string(), 0.01); //  Centi
        suffixes.insert("d".to_string(), 0.1); //  Deci
        suffixes.insert("da".to_string(), 10.0); //  Deca
        suffixes.insert("h".to_string(), 100.0); //  Hecto
        suffixes.insert("k".to_string(), 1000.0); //  Kilo
        suffixes.insert("M".to_string(), 1_000_000.0); //  Mega
        suffixes.insert("G".to_string(), 1_000_000_000.0); //  Giga
        suffixes.insert("B".to_string(), 1_000_000_000.0); //  Billion
        suffixes.insert("T".to_string(), 1_000_000_000_000.0); //  Tera
        suffixes.insert("P".to_string(), 1e15); //  Peta
        //  suffixes.insert("E".to_string(), 1e18); //  Exa 
        suffixes.insert("Z".to_string(), 1e21); //  Zera
        suffixes.insert("Y".to_string(), 1e24); //  Yotta
        let exclude_suffixes = self.config.exclude_suffixes.clone().unwrap_or_else(Vec::new);
        if !exclude_suffixes.is_empty() {
            if (exclude_suffixes.len() == 1) && (exclude_suffixes[0] == "all".to_string()) {
                return HashMap::new();
            }
            for suffix in &exclude_suffixes {
                suffixes.remove(suffix);
            }
        }
        suffixes
    }
    
    pub fn suffixes_by_name(&mut self) -> HashMap<String, f64> {
        let mut suffixes_by_name = HashMap::new();
        suffixes_by_name.insert("yocto".to_string(),1e-24); //  y
        suffixes_by_name.insert("zepto".to_string(), 1e-21); //  z
        suffixes_by_name.insert("atto".to_string(), 1e-18); //  a
        suffixes_by_name.insert("femto".to_string(), 1e-15); //  f
        suffixes_by_name.insert("pico".to_string(), 1e-12); //  p
        suffixes_by_name.insert("nano".to_string(), 1e-9); //  n
        suffixes_by_name.insert("micro".to_string(), 1e-6); //  μ
        suffixes_by_name.insert("milli".to_string(), 0.001); //  m
        suffixes_by_name.insert("centi".to_string(), 0.01); //  c
        suffixes_by_name.insert("deci".to_string(), 0.1); //  d
        suffixes_by_name.insert("deca".to_string(), 10.0); //  da
        suffixes_by_name.insert("hecto".to_string(), 100.0); //  h
        suffixes_by_name.insert("kilo".to_string(), 1000.0); //  k
        suffixes_by_name.insert("mega".to_string(), 1_000_000.0); //  M
        suffixes_by_name.insert("giga".to_string(), 1_000_000_000.0); //  G
        suffixes_by_name.insert("tera".to_string(), 1_000_000_000_000.0); //  T
        suffixes_by_name.insert("peta".to_string(), 1e15); //  P
        suffixes_by_name.insert("exa".to_string(), 1e18); //  E
        suffixes_by_name.insert("zetta".to_string(), 1e21); //  Z
        suffixes_by_name.insert("yotta".to_string(), 1e24); //  Y
        suffixes_by_name
    }
    
    pub fn informal_exact(&mut self) -> HashMap<String, f64> {
        let mut informal_exact = HashMap::new();
        //  informal_exact.insert("single".to_string(), 1.0);
        //  informal_exact.insert("couple".to_string(), 2.0);
        informal_exact.insert("half".to_string(), 0.5);
        informal_exact.insert("quarter".to_string(), 0.25);
        //  informal_exact.insert("pair".to_string(), 2.0);
        //  informal_exact.insert("few".to_string(), 3.0);
        informal_exact.insert("dozen".to_string(), 12.0);
        informal_exact
    }
    
    pub fn informals_multiplyable(&mut self) -> HashMap<String, f64> {
        let mut infomals_multiplyable = HashMap::new();
        infomals_multiplyable.insert("couples".to_string(), 2.0);
        infomals_multiplyable.insert("pairs".to_string(), 2.0);
        infomals_multiplyable.insert("dozens".to_string(), 12.0);
        infomals_multiplyable.insert("quarters".to_string(), 0.25);
        infomals_multiplyable.insert("halves".to_string(), 0.5);
        infomals_multiplyable
    }
    
    pub fn superscript_ones(&mut self) -> HashMap<char, f64> {
        let mut superscript_ones = HashMap::new();
        superscript_ones.insert('⁰', 0.0);
        superscript_ones.insert('¹', 1.0);
        superscript_ones.insert('²', 2.0);
        superscript_ones.insert('³', 3.0);
        superscript_ones.insert('⁴', 4.0);
        superscript_ones.insert('⁵', 5.0);
        superscript_ones.insert('⁶', 6.0);
        superscript_ones.insert('⁷', 7.0);
        superscript_ones.insert('⁸', 8.0);
        superscript_ones.insert('⁹', 9.0);
        superscript_ones
    }
    
    pub fn superscript_ones_regex(&mut self) -> String {
        format!("(?:[{}])+", self.superscript_ones().iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>().join(""))
    }
    
    pub fn subscript_ones(&mut self) -> HashMap<char, f64> {
        let mut subscript_ones = HashMap::new();
        subscript_ones.insert('₀', 0.0);
        subscript_ones.insert('₁', 1.0);
        subscript_ones.insert('₂', 2.0);
        subscript_ones.insert('₃', 3.0);
        subscript_ones.insert('₄', 4.0);
        subscript_ones.insert('₅', 5.0);
        subscript_ones.insert('₆', 6.0);
        subscript_ones.insert('₇', 7.0);
        subscript_ones.insert('₈', 8.0);
        subscript_ones.insert('₉', 9.0);
        subscript_ones
    }
    
    pub fn subscript_ones_regex(&mut self) -> String {
        format!("(?:[{}])+", self.subscript_ones().iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>().join(""))
    }
    
    pub fn superscript_fractions(&mut self) -> HashMap<char, f64> {
        let mut superscript_fractions = HashMap::new();
        superscript_fractions.insert('½', 0.5);
        superscript_fractions.insert('⅓', 1.0 / 3.0);
        superscript_fractions.insert('¼', 1.0 / 4.0);
        superscript_fractions.insert('⅕', 1.0 / 5.0);
        superscript_fractions.insert('⅙', 1.0 / 6.0);
        superscript_fractions.insert('⅐', 1.0 / 7.0);
        superscript_fractions.insert('⅛', 1.0 / 8.0);
        superscript_fractions.insert('⅑', 1.0 / 9.0);
        superscript_fractions.insert('⅒', 1.0 / 10.0);
        superscript_fractions.insert('⅖', 2.0 / 5.0);
        superscript_fractions.insert('⅔', 2.0 / 3.0);
        superscript_fractions.insert('¾', 3.0 / 4.0);
        superscript_fractions.insert('⅗', 3.0 / 5.0);
        superscript_fractions.insert('⅜', 3.0 / 8.0);
        superscript_fractions.insert('⅘', 4.0 / 5.0);
        superscript_fractions.insert('⅚', 5.0 / 6.0);
        superscript_fractions.insert('⅝', 5.0 / 8.0);
        superscript_fractions.insert('⅞', 7.0 / 8.0);
        superscript_fractions
    }
    
    pub fn superscript_fractions_regex(&mut self) -> String {
        format!("(?:[{}])", self.superscript_fractions().iter().map(|(k, _)| k.to_string()).collect::<Vec<_>>().join(""))
    }
    
    pub fn ordinal_suffixes(&mut self) -> Vec<String> {
        vec!["st".to_string(),"nd".to_string(), "rd".to_string(), "th".to_string()]
    }
    
    pub fn ordinals(&mut self) -> HashMap<String, f64> {
        let mut ordinals = HashMap::new();
        ordinals.extend(self.ordinal_ones());
        ordinals.extend(self.ordinal_teens_and_ten());
        ordinals.extend(self.ordinal_tens());
        ordinals.extend(self.ordinal_multiples());
        ordinals
    }
    
    pub fn informal_all(&mut self) -> HashMap<String, f64> {
        let mut informal_all = HashMap::new();
        informal_all.extend(self.informal_exact());
        informal_all.extend(self.informals_multiplyable());
        let copy = informal_all.clone();
        for (_, v) in &copy {
            informal_all.insert(v.to_string(), *v);
        }
        informal_all
    }
    
    pub fn all_nums(&mut self) -> HashMap<String, f64> {
        let mut all_nums = HashMap::new();
        all_nums.extend(self.ones());
        all_nums.extend(self.teens_and_ten());
        all_nums.extend(self.tens());
        all_nums.extend(self.multiples());
        all_nums.extend(self.informal_all());
        let copy = all_nums.clone();
        for (_, v) in &copy {
            all_nums.insert(v.to_string(), *v);
        }
        let mut other: Vec<String> = Vec::new();
        other.extend(self.a());
        other.extend(self.ands());
        other.extend(self.points());
        other.extend(self.negatives());
        for (i, &ref item) in other.iter().enumerate() {
            all_nums.insert(item.clone(), i as f64);
        }
        all_nums.extend(self.ordinals());
        all_nums
    }
    
    pub fn _tens(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
        .join(
            self.tens()
            .iter()
            .map(|(k, _)| k.to_string())
            .collect::<Vec<String>>(),
            None
        )
    }
    
    pub fn _ones(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
        .join(
            self.ones()
            .iter()
            .filter(|(k, _)| {
                **k != "zero".to_string()
            })
            .map(|(k, _)| k.to_string())
            .collect::<Vec<String>>(),
            None
        )
    }
    
    pub fn _ordinal_ones(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
        .join(
            self.ordinal_ones()
            .iter()
            .map(|(k, _)| k.to_string())
            .collect::<Vec<String>>(),
            None
        )
    }
    
    pub fn _teens(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
        .join(
            self.teens_and_ten()
            .iter()
            .filter(|(k, _)| {
                **k != "ten".to_string()
            })
            .map(|(k, _)| k.to_string())
            .collect::<Vec<String>>(),
            None
        )
    }
    
    pub fn _multiples(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
        .join(
            self.multiples()
            .iter()
            .map(|(k, _)| k.to_string())
            .collect::<Vec<String>>(),
            None
        )
    }
    
    pub fn _suffixes(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
        .join(
            self.suffixes()
            .iter()
            .map(|(k, _)| k.to_string())
            .collect::<Vec<String>>(),
            None
        )
    }
    
    pub fn _negs(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
        .join(
            self.negatives()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
            None
        )
    }
    
    pub fn _points(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
        .join(
            self.points()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>(),
            None
        )
    }
    
    pub fn hyphen(&mut self) -> String {
        format!(r#"
            {EXTENDED}{IGNORECASE}\b(
            ({tens})-({ones}|{ordinal_ones}|{multiples})
            |({ones})-({multiples}|hundred)
            |({teens})-({multiples})
            )\b
        "#,
            tens=self._tens(), ones=self._ones(),
            ordinal_ones=self._ordinal_ones(),
            multiples=self._multiples(),
            teens=self._teens()
        )
    }
    
    pub fn integer_regex(&mut self) -> String {
        let mut seps = vec![".to_string(),".to_string(), "_".to_string(), "'".to_string(), " ".to_string()].to_vec(); // space Seperators doesn't work!
        let mut exclude_separators = self.config.exclude_separators.clone().unwrap_or_else(Vec::new);
        exclude_separators.dedup();
        if !exclude_separators.is_empty() {
            for separator in exclude_separators.iter() {
                let index = seps
                    .iter()
                    .position(|x| x == separator)
                    .unwrap();
                seps.remove(index);
            }
        }
        let mut patterns: Vec<String> = Vec::new();
        for sep in seps {
            patterns.push(format!(r"\d{{,3}}(?:{sep}\d{{3}})+"));
        }
        patterns.push(r"\d+".to_string());
        let mut pattern: String = patterns.join("|");
        pattern = format!("(?:{})", pattern);
        if self.config.signs_allowed.unwrap() {
            pattern =  format!(r"(?:[\-\+])?{}", pattern);
        }
        if self.config.bounded_numbers.unwrap() {
            pattern = format!(r"\b{}", pattern);
        }
        pattern
    }
    
    pub fn float_regex(&mut self) -> String {
        let mut intre = self.integer_regex();
        if self.config.bounded_numbers.unwrap() {
            // intre = intre.trim(r"\b");
        }
        let mut pattern = format!(r#"{EXTENDED}{intre}(?:\.\d+(?:[eE][\-\+]?\d+)?)|{intre}(?:(?:\.\d+)?[eE][\-\+]?\d+)|{intre}(?:\.\d+(?:[eE][\-\+]?\d+)?)|{intre}(?:(?:\.\d+)?[eE][\-\+]?\d+)|{intre}(?:\.\d+(?:[eE][\-\+]?\d+)?)|{intre}(?:(?:\.\d+)?[eE][\-\+]?\d+)|{intre}(?:\.\d+)|(?:\.\d+)(?:[eE][\-\+]?\d+)?"#);
        pattern = format!("(?:{})", pattern);
        if self.config.bounded_numbers.unwrap() {
            pattern = format!("{}{}", B_LEFT, pattern);
        }
        pattern
    }
    
    pub fn any_number_regex(&mut self) -> String {
        format!("(?:{}|{})", self.float_regex(), self.integer_regex())
    }
    
    pub fn complex_number_regex(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        processor
            .bound(format!(r"{}[ij]\b", self.any_number_regex()), None)
    }
    
    pub fn binary_regex(&mut self) -> String {
        format!("{}{}{}", B_LEFT, r"0[bB][01]+", B_RIGHT)
    }
    
    pub fn hex_regex(&mut self) -> String {
        format!("{}{}{}", B_LEFT, r"0[xX][0-9a-fA-F]+", B_RIGHT)
    }
    
    pub fn oct_regex(&mut self) -> String {
        format!("{}{}{}", B_LEFT, r"0[oO][0-7]+", B_RIGHT)
    }
    
    pub fn _all_ones(&mut self) -> Vec<String> {
        let mut all_ones = Vec::new();
        for (k, v) in self.ones() {
            all_ones.push(k.to_string());
            all_ones.push(v.to_string());
        }
        all_ones
    }
    
    pub fn number_followed_by_power_regex(&mut self) -> String {
        format!(r"{DEFAULT_RE_FLAGS}(?P<number>{_any_number})\s*(?P<power>{_power_names}){b_right}", 
            _any_number =self.any_number_regex(),
            _power_names=self._multiples(),
            b_right=B_RIGHT,
        )
    }
    
    pub fn suffix_name_regex(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        let suffix_names = processor
        .join(
            self.suffixes_by_name()
                .iter()
                .map(|(k, _)| k.to_string())
                .collect::<Vec<String>>(),
            None
        );
        let pattern = processor.bound(suffix_names, None);
        format!("{IGNORECASE}{EXTENDED}{pattern}")
    }
        
    pub fn number_followed_by_suffix_regex(&mut self) -> String {
        // any Number followed by a multiple suffix
        let mut processor = RegexProcessor::new();
        let mut _suffixes_by_name = processor
        .join(
            self.suffixes_by_name()
            .iter()
            .map(|(k, _)| {
                k.to_string()
            })
            .collect::<Vec<String>>(),
            None
        );
        let pattern = format!(r"{EXTENDED}(?P<number>{_any_number})(?P<suffix>\s*(?:{_suffixes_by_name})|(?:{_suffixes}))\b", 
            _any_number=self.any_number_regex(),
            _suffixes=self._suffixes(),
            _suffixes_by_name=_suffixes_by_name,
        );
        pattern
    }

    pub fn informals_exact_regex(&mut self) -> String {
        // infomals couple, pair, dozen...
        let mut processor = RegexProcessor::new();
        let mut _informal = processor.
            join(
                self.informal_exact()
                .iter()
                .map(|(k, _)| k.to_string())
                .collect::<Vec<String>>(),
                None
            );
        let ones = vec!["1".to_string(), "0".to_string(), "one".to_string(), "zero".to_string()].to_vec();
        let mut _small = processor
            .join(
                ones,
                None
            );
        let pattern = format!(r"{DEFAULT_RE_FLAGS}(?:{_small})\s+(?:{_informal}){B_RIGHT}");
        pattern
    }

    pub fn informals_multiplyable_regex(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        let mut _informals_multiplyable = processor
            .join(
                self.informals_multiplyable()
                .iter()
                .map(|(k, _)| k.to_string())
                .collect::<Vec<String>>(),
                None
            );
        let mut pattern = format!(r"{DEFAULT_RE_FLAGS}(?:{_any_number})\s+(?:{_informals_multiplyable})",
            _any_number=self.any_number_regex(),
            _informals_multiplyable=_informals_multiplyable,
        );
        pattern = processor.bound(pattern, None);
        pattern
    }

    pub fn ordinal_numeral_regex(&mut self) -> String {
        let mut processor = RegexProcessor::new();
        let mut _ordinal_suffixes = processor.retrie(
            self.ordinal_suffixes()
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
        );
        let mut pattern = format!(r"{EXTENDED}(?P<number>{integer_regex})(?P<ordinal>{_ordinal_suffixes})",
            integer_regex=self.integer_regex(),
            _ordinal_suffixes=_ordinal_suffixes,
        );
        pattern = processor.bound(pattern, None);
        pattern
    }

    pub fn first_extraction_regexes(&mut self) -> Vec<String> {
        let mut regexes = vec![
            self.number_followed_by_suffix_regex(), // 0
            self.superscript_ones_regex(), // 1
            self.subscript_ones_regex(),
            self.superscript_fractions_regex(), // 2
            self.hex_regex(), // 3
            self.oct_regex(), // 4
            self.binary_regex(), // 5
            self.ordinal_numeral_regex(), // 6
            self.number_followed_by_power_regex(), // 7
            self.informals_multiplyable_regex(), // 8
        ].to_vec();
        if self.config.parse_complex.unwrap() {
            regexes.insert(6, self.complex_number_regex());
        }
        regexes
    }
         
    pub fn last_extraction_regexes(&mut self) -> Vec<String> {
        vec![self.any_number_regex()].to_vec()
    }
    
    pub fn get_suffix_value(&mut self, suffix: &str) -> Option<f64> {
        let mut map = HashMap::<String, f64>::new();
        map.extend(self.suffixes());
        map.extend(self.suffixes_by_name());
        map.get(suffix).copied()
    }
}

