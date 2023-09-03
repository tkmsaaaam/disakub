#[derive(Clone)]
pub struct Token {
    pub kind: i64,
}

pub struct Tokenizer {
    input: String,
}

pub fn new_tokenizer(input: String) -> Tokenizer {
    return Tokenizer { input };
}

impl Tokenizer {
    pub fn tokenize(self) -> Vec<Token> {
        let _token = Token { kind: 0 };
        let mut tokens = Vec::new();
        if self.input.starts_with("create") {
            let token = Token { kind: 0 };
            tokens.push(token);
        } else if self.input.starts_with("select") {
            let token = Token { kind: 1 };
            tokens.push(token);
        } else if self.input.starts_with("insert") {
            let token = Token { kind: 2 };
            tokens.push(token);
        }

        return tokens;
    }
}
