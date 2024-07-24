#![allow(clippy::needless_return)]

use crate::expr::{BinaryOp, Expr, Literal, LogicalOp, UnaryOp};
use crate::parse_error::ParseError;
use crate::statement::Statement;
use crate::token::token;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

#[derive(Default)]
pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        return Ok(statements);
    }

    fn declaration(&mut self) -> Result<Statement, ParseError> {
        if self.match_token_type(TokenType::Var) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn consume_identifier(&mut self) -> Result<Token, ParseError> {
        let token = self.consume(TokenType::Identifier, "Expect variable name.")?;
        Ok(token.clone())
    }

    fn var_declaration<'a>(&mut self) -> Result<Statement, ParseError> {
        let name = self.consume_identifier()?;

        let initializer = if self.match_token_type(TokenType::Equal) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;

        Ok(Statement::Var(name, initializer))
    }

    fn statement(&mut self) -> Result<Statement, ParseError> {
        if self.match_token_type(TokenType::For) {
            return self.for_statement();
        }

        if self.match_token_type(TokenType::If) {
            return self.if_statement();
        };

        if self.match_token_type(TokenType::Print) {
            return self.print_statement();
        }

        if self.match_token_type(TokenType::While) {
            return self.while_statement();
        };

        if self.match_token_type(TokenType::LeftBrace) {
            return Ok(Statement::Block(self.block()?));
        }

        return self.expression_statement();
    }

    fn for_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.match_token_type(TokenType::Semicolon) {
            None
        } else if self.match_token_type(TokenType::Var) {
            Some(self.var_declaration())
        } else {
            Some(self.expression_statement())
        };

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;

        if increment.is_some() {
            body = Statement::Block(vec![body, Statement::Expression(increment.unwrap())]);
        }

        if condition.is_none() {
            condition = Some(Expr::Literal(Literal::True));
        }
        body = Statement::While(condition.unwrap(), Box::new(body));

        match initializer {
            Some(initializer) => body = Statement::Block(vec![initializer.unwrap(), body]),
            None => (),
        }

        return Ok(body);
    }

    fn while_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        return Ok(Statement::While(condition, Box::new(body)));
    }

    fn if_statement(&mut self) -> Result<Statement, ParseError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_token_type(TokenType::Else) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::If(condition, Box::new(then_branch), else_branch))
    }

    fn block(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}' after block.");
        return Ok(statements);
    }

    fn print_statement(&mut self) -> Result<Statement, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Statement::Print(value))
    }

    fn expression_statement(&mut self) -> Result<Statement, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Statement::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        return self.assignment();
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_token_type(TokenType::And) {
            let operator = self.previous();
            let maybe_logical_operator = Parser::token_to_logical_operator(operator);
            let right = self.equality()?;

            match maybe_logical_operator {
                Ok(operator) => expr = Expr::Logical(Box::new(expr), operator, Box::new(right)),
                Err(err) => return Err(err),
            }
        }

        return Ok(expr);
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_token_type(TokenType::Or) {
            let operator = self.previous();
            let maybe_logical_operator = Parser::token_to_logical_operator(operator);
            let right = self.and()?;

            match maybe_logical_operator {
                Ok(operator) => expr = Expr::Logical(Box::new(expr), operator, Box::new(right)),
                Err(err) => return Err(err),
            }
        }

        return Ok(expr);
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or();

        if self.match_token_type(TokenType::Equal) {
            return match expr {
                Ok(Expr::Variable(name)) => {
                    let value = self.assignment()?;
                    Ok(Expr::Assign(name, Box::new(value)))
                }
                _ => {
                    let equals = self.previous();
                    Err(ParseError::UnexpectedTokenError(equals.token_type))
                }
            };
        }

        return expr;
    }

    fn match_equality_token(&mut self) -> bool {
        if self.check(TokenType::BangEqual) || self.check(TokenType::EqualEqual) {
            self.advance();
            return true;
        }

        return false;
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_equality_token() {
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

    fn match_comparision_token(&mut self) -> bool {
        if self.check(TokenType::Greater)
            || self.check(TokenType::GreaterEqual)
            || self.check(TokenType::Less)
            || self.check(TokenType::LessEqual)
        {
            self.advance();
            return true;
        }

        return false;
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_comparision_token() {
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

    fn match_term_token(&mut self) -> bool {
        if self.check(TokenType::Minus) || self.check(TokenType::Plus) {
            self.advance();
            return true;
        }

        return false;
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_term_token() {
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

    fn match_factor_token(&mut self) -> bool {
        if self.check(TokenType::Slash) || self.check(TokenType::Star) {
            self.advance();
            return true;
        }

        return false;
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_factor_token() {
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

    fn match_unary_token(&mut self) -> bool {
        if self.check(TokenType::Bang) || self.check(TokenType::Minus) {
            self.advance();
            return true;
        }

        return false;
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_unary_token() {
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

    fn match_token_type(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            return true;
        }

        return false;
    }

    fn match_literal_token_type(&mut self) -> bool {
        if self.check(TokenType::Number) || self.check(TokenType::String) {
            self.advance();
            return true;
        }

        return false;
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_token_type(TokenType::False) {
            return Ok(Expr::Literal(Literal::False));
        }
        if self.match_token_type(TokenType::True) {
            return Ok(Expr::Literal(Literal::True));
        }
        if self.match_token_type(TokenType::Nil) {
            return Ok(Expr::Literal(Literal::Nil));
        }

        if self.match_literal_token_type() {
            match &self.previous().literal {
                Some(token::Literal::Number(n)) => return Ok(Expr::Literal(Literal::Number(*n))),
                Some(token::Literal::Str(string)) => {
                    return Ok(Expr::Literal(Literal::Str(string.to_string())));
                }
                Some(other) => panic!("Failed to parse expected number: {}", other),
                None => panic!("Failed to parse number"),
            }
        }
        if self.match_token_type(TokenType::Identifier) {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.match_token_type(TokenType::LeftParen) {
            let expr = Box::new(self.expression()?);
            return match self.consume(TokenType::RightParen, "Expect ')' after expression.") {
                Ok(_) => Ok(Expr::Grouping(expr)),
                Err(err) => Err(err),
            };
        }

        Err(ParseError::UnexpectedTokenError(self.peek().token_type))
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

    fn token_to_logical_operator(token: &Token) -> Result<LogicalOp, ParseError> {
        match token.token_type {
            TokenType::Or => Ok(LogicalOp::Or),
            TokenType::And => Ok(LogicalOp::And),
            _ => Err(ParseError::UnexpectedTokenError(token.token_type)),
        }
    }

    pub fn token_to_binary_operator(token: &Token) -> Result<BinaryOp, ParseError> {
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
            _ => Err(ParseError::UnexpectedTokenError(token.token_type)),
        }
    }

    pub fn token_to_unary_operator(token: &Token) -> Result<UnaryOp, ParseError> {
        match token.token_type {
            TokenType::Minus => Ok(UnaryOp::Minus),
            TokenType::Bang => Ok(UnaryOp::Bang),
            _ => Err(ParseError::UnexpectedTokenError(token.token_type)),
        }
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token, ParseError> {
        if self.check(token_type) {
            let token = self.advance();
            return Ok(token);
        }

        eprintln!("{}", message);
        return Err(ParseError::UnexpectedTokenError(self.peek().token_type));
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
