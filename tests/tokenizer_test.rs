use loxc::token::token_type::TokenType;
use loxc::tokenizer::Tokenizer;

#[test]
fn tokenizes_a_empty_source() {
    let mut tokenizer = Tokenizer {
        source: "",
        ..Default::default()
    };

    assert_eq!("0", tokenizer.scan_tokens().to_string());
    assert_eq!(TokenType::EOF, tokenizer.tokens[0].token_type);
}

#[test]
fn tokenizes_numbers() {
    let mut tokenizer = Tokenizer {
        source: "2",
        ..Default::default()
    };

    assert_eq!("0", tokenizer.scan_tokens().to_string());

    assert_eq!(TokenType::Number, tokenizer.tokens[0].token_type);
}
