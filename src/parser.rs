#![allow(clippy::needless_return)]
use crate::expr::{BinaryOp, Expr, Literal, UnaryOp};
use crate::parse_error::ParseError;
use crate::token::token;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

#[derive(Default)]
pub struct Parser<'a> {
    pub tokens: Vec<Token<'a>>,
    pub current: usize,
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        return self.expression();
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.equality();
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_types(vec![TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let maybe_binary_operator = Parser::token_to_binary_operator(operator);

            let right = self.comparison()?;

            match maybe_binary_operator {
                Ok(binary_operator) => {
                    expr = Expr::Binary(Box::new(expr), binary_operator, Box::new(right))
                }
                Err(err) => return Err(err),
            }
        }

        return Ok(expr);
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_types(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let maybe_binary_operator = Parser::token_to_binary_operator(operator);

            let right = self.term()?;

            match maybe_binary_operator {
                Ok(binary_operator) => {
                    expr = Expr::Binary(Box::new(expr), binary_operator, Box::new(right));
                }
                Err(err) => return Err(err),
            }
        }

        return Ok(expr);
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_types(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let maybe_binary_operator = Parser::token_to_binary_operator(operator);

            let right = self.factor()?;

            match maybe_binary_operator {
                Ok(binary_operator) => {
                    expr = Expr::Binary(Box::new(expr), binary_operator, Box::new(right))
                }
                Err(err) => return Err(err),
            }
        }

        return Ok(expr);
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let maybe_binary_operator = Parser::token_to_binary_operator(operator);
            let right = self.unary()?;

            match maybe_binary_operator {
                Ok(binary_operator) => {
                    expr = Expr::Binary(Box::new(expr), binary_operator, Box::new(right))
                }
                Err(err) => return Err(err),
            }
        }

        return Ok(expr);
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(vec![TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let maybe_binary_operator = Parser::token_to_unary_operator(operator);

            let right = self.unary()?;

            return match maybe_binary_operator {
                Ok(binary_operator) => Ok(Expr::Unary(binary_operator, Box::new(right))),
                Err(err) => Err(err),
            };
        }

        return self.primary();
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(vec![TokenType::False]) {
            return Ok(Expr::Literal(Literal::False));
        }
        if self.match_types(vec![TokenType::True]) {
            return Ok(Expr::Literal(Literal::True));
        }
        if self.match_types(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_types(vec![TokenType::Number, TokenType::String]) {
            match &self.previous().literal {
                Some(token::Literal::Number(n)) => return Ok(Expr::Literal(Literal::Number(*n))),
                Some(token::Literal::Str(string)) => {
                    return Ok(Expr::Literal(Literal::Str(string.clone())));
                }
                Some(other) => panic!("Failed to parse number"),
                None => panic!("Failed to parse number"),
            }
        }
        if self.match_types(vec![TokenType::LeftParen]) {
            let expr = Box::new(self.expression()?);
            self.consume(TokenType::RightParen, "Expect ')' after expression.");
            return Ok(Expr::Grouping(expr));
        }

        Err(ParseError {})
    }

    fn synchronize(mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return;
            }

            if matches!(
                &self.peek().token_type,
                TokenType::Class
                    | TokenType::Fun
                    | TokenType::Var
                    | TokenType::For
                    | TokenType::If
                    | TokenType::While
                    | TokenType::Print
                    | TokenType::Return
            ) {
                return;
            }

            self.advance();
        }
    }

    pub fn token_to_binary_operator(token: &'a Token) -> Result<BinaryOp, ParseError> {
        match token.token_type {
            TokenType::Plus => Ok(BinaryOp::Plus),
            TokenType::Minus => Ok(BinaryOp::Minus),
            TokenType::Star => Ok(BinaryOp::Star),
            TokenType::Slash => Ok(BinaryOp::Slash),
            TokenType::EqualEqual => Ok(BinaryOp::EqualEqual),
            TokenType::BangEqual => Ok(BinaryOp::BangEqual),
            TokenType::Less => Ok(BinaryOp::Less),
            TokenType::LessEqual => Ok(BinaryOp::LessEqual),
            TokenType::Greater => Ok(BinaryOp::Greater),
            TokenType::GreaterEqual => Ok(BinaryOp::GreaterEqual),
            _ => Err(ParseError {}),
        }
    }

    pub fn token_to_unary_operator(token: &'a Token) -> Result<UnaryOp, ParseError> {
        match token.token_type {
            TokenType::Minus => Ok(UnaryOp::Minus),
            TokenType::Bang => Ok(UnaryOp::Bang),
            _ => Err(ParseError {}),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            return Ok(self.advance());
        }

        return Err(ParseError {});
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

    fn previous(&mut self) -> &Token {
        return &self.tokens[self.current - 1];
    }
}
