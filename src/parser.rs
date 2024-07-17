use crate::expr::Expr;
use crate::parse_error::ParseError;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

#[derive(Default)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl<'a> Parser {
    fn expression(self) -> Expr<'a> {
        return self.equality();
    }

    pub fn parse(self) -> u8 {
        todo!();
    }

    fn synchronize(mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().unwrap().token_type == TokenType::SEMICOLON {
                return;
            }

            if matches!(
                &self.peek().unwrap().token_type,
                TokenType::CLASS
                    | TokenType::FUN
                    | TokenType::VAR
                    | TokenType::FOR
                    | TokenType::IF
                    | TokenType::WHILE
                    | TokenType::PRINT
                    | TokenType::RETURN
            ) {
                return;
            }

            self.advance();
        }
    }

    fn consume(&mut self, token_type: TokenType, message: String) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance().unwrap());
        }

        let peek = self.peek().unwrap();

        return Err(ParseError { peek, message });
    }

    fn primary(&self) -> Expr {
        //if (self.match_types(vec![TokenType::FALSE])) {
        //    return Expr { Some(false)};
        //}
        //if (self.match_types(vec![TokenType::TRUE])) {
        //    return Expr::Literal(Some(true));
        //}
        //if (self.match_types(vec![TokenType::NIL])) {
        //    return Expr::Literal(None);
        //}

        //if (self.match_types(vec![TokenType::NUMBER, TokenType::STRING])) {
        //    return Expr::Literal(self.previous().unwrap().literal);
        //}

        //if (self.match_types(vec![TokenType::LEFT_PAREN])) {
        //    let expr = self.expression();
        //    self.consume(
        //        TokenType::RIGHT_PAREN,
        //        String::from("Expect ')' after expression."),
        //    );
        //    return Expr::Grouping(expr);
        //}
        todo!();
    }

    fn unary(mut self) -> Expr<'a> {
        if self.match_types(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary();
            return Expr {
                left: None,
                operator,
                right: Some(Box::new(right)),
            };
        }

        return self.primary();
    }

    fn factor(mut self) -> Expr<'a> {
        let mut expr = self.unary();

        while self.match_types(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();
            expr = Expr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right)),
            };
        }

        return expr;
    }

    fn term(mut self) -> Expr<'a> {
        let mut expr = self.factor();

        while self.match_types(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous();
            let right = self.factor();
            expr = Expr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right)),
            }
        }

        return expr;
    }

    fn comparison(mut self) -> Expr<'a> {
        let mut expr = self.term();

        while self.match_types(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous();
            let right = self.term();
            expr = Expr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right)),
            }
        }

        return expr;
    }

    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }

        return false;
    }

    fn equality(mut self) -> Expr<'a> {
        let mut expr = self.comparison();

        while self.match_types(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr {
                left: Some(Box::new(expr)),
                operator,
                right: Some(Box::new(right)),
            }
        }

        return expr;
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().unwrap().token_type == token_type;
    }

    fn advance(&mut self) -> Option<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        };
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().unwrap().token_type == TokenType::EOF;
    }

    fn peek(&self) -> Option<&Token> {
        return self.tokens.iter().nth(self.current);
    }

    fn previous(&self) -> Option<&Token> {
        return self.tokens.iter().nth(self.current - 1);
    }
}
