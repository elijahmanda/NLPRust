
pub struct Token {
    pub text: String,
    pub entity: Option<String>,
    pub span: Option<(usize, usize)>,
}


impl Token {
    pub fn new(
        text: String,
        entity: Option<String>,
        span: Option<(usize, usize)>,
    ) -> Self {
        Token {
            text: text,
            entity: entity,
            span: span,
        }
    }

    fn __hash__(&self) -> u64 {
        let _tuple_hash = (self.text.clone(), self.span, self.entity.clone());
        std::hash::Hasher::finish(&mut std::collections::hash_map::DefaultHasher::new())
    }

    fn __repr__(&self) -> String {
        let mut msg = format!("Token(text={:?}", self.text);
        if let Some(entity) = &self.entity {
            msg.push_str(&format!(", entity={:?}", entity));
        }
        if self.span.is_some() {
            msg.push_str(&format!(", span={:?}", self.span));
        }
        msg.push(')');
        msg
    }

    fn __str__(&self) -> String {
        self.text.clone().to_string()
    }
}
