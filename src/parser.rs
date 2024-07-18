use crate::expr::{Expr, UnaryOp, Literal};
use crate::parse_error::ParseError;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

#[derive(Default, Copy)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl<'a> Parser {
    fn expression(mut self) -> Expr<'a> {
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

    fn primary(&self) -> Result<Expr, ParseError> {
        if self.match_types(vec![TokenType::FALSE]) {
            return Ok(Expr::Literal(Literal::False))
        }
        if self.match_types(vec![TokenType::TRUE]) {
            return Ok(Expr::Literal(Literal::True))
        }
        if self.match_types(vec![TokenType::NIL]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if (self.match_types(vec![TokenType::NUMBER, TokenType::STRING])) {
            match self.previous().literal {
                Some(Literal::Number(n)) => return Ok(Expr::Literal(Literal::Number(*n))),
                Some(other) => panic!("internal error!"),
                None => panic!("internal error!"),
            }
        }

        if (self.match_types(vec![TokenType::LEFT_PAREN])) {
            let expr = self.expression();
            self.consume(
                TokenType::RIGHT_PAREN,
                String::from("Expect ')' after expression."),
            );
            return Expr::Grouping(expr);
        }
    }

    fn factor(mut self) -> Expr {
        let mut expr = self.unary();

        while self.match_types(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous();
            let right = self.unary();

            let unary_op_maybe = Parser::token_to_unary_operator(operator);

           return match
            expr = Expr::Unary(
                Box::new(expr),
                Parser::token_to_unary_operator(operator),
                Box::new(right)
            );
        }

        return expr;
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous();
            let right = self.unary()?;

            let unary_operator_maybe = Parser::token_to_unary_operator(operator);

            return match unary_operator_maybe {
                Ok(unary_operator) => Ok(
                    Expr::Unary(unary_operator, Box::new(right))
                ),
                Err(err) => Err(err)
            };
        }

        return self.primary();
    }


    pub fn token_to_unary_operator(token: &'a Token) -> Result<UnaryOp, ParseError<'a>> {
        match token.token_type {
            TokenType::MINUS => Ok(UnaryOp::Minus),
            TokenType::BANG => Ok(UnaryOp::Bang),
            _ => Err(ParseError { peek: token, message: "Error converting a token to unary operator" })
        }
    }

    fn term(mut self) -> Expr {
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

    fn comparison(mut self) -> Expr {
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

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let expr = self.comparison();

        while self.match_types(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
                    let operator = self.previous();
                    let right = Box::new(self.comparison());

                    expr = Expr {
                        left: Some(Box::new(expr)),
                        operator,
                        right: Some(right),
                    };
        }

        return Ok(expr);
    }

    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == token_type;
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        };
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        return self.peek().token_type == TokenType::EOF;
    }

    fn peek(&self) -> &Token {
        return &self.tokens[self.current];
    }

    fn previous(&self) -> &Token {
        return &self.tokens[self.current - 1];
    }
}
