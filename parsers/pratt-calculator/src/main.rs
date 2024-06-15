#[derive(Clone, Debug)]
enum Token {
    EOF,
    Literal(i32),
    Plus,
    Times,
    LeftParen,
    RightParen,
}

struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, index: 0 }
    }

    fn current_precedence(&self) -> usize {
        match self.current_token() {
            Token::EOF => 0,
            Token::LeftParen => 0,
            Token::RightParen => 0,
            Token::Literal(_) => 3,
            Token::Plus => 4,
            Token::Times => 5,
        }
    }

    fn null_denotation(&mut self, cur_token: Token) -> i32 {
        match cur_token {
            Token::Literal(n) => self.parse_literal(n),
            Token::EOF => -1,
            Token::LeftParen => self.parse_group(),
            _ => panic!("Expected literal, found: {:?}", cur_token),
        }
    }

    fn left_denotation(&mut self, left: i32) -> i32 {
        match self.current_token() {
            Token::Plus => self.parse_addition(left),
            Token::Times => self.parse_multiplication(left),
            t => panic!("Expected EOF, Literal or Operator -- found {t:?}"),
        }
    }

    fn parse_expr(&mut self, precedence: usize) -> i32 {
        let cur_token = self.current_token();
        let mut left = self.null_denotation(cur_token);
        while precedence < self.current_precedence() {
            left = self.left_denotation(left)
        }
        left
    }

    fn parse_group(&mut self) -> i32 {
        self.advance();
        let left = self.parse_expr(3);
        match self.current_token() {
            Token::RightParen => {
                self.advance();
                left
            }
            t => panic!("Expected ')', found {t:?}"),
        }
    }

    fn parse_literal(&mut self, n: i32) -> i32 {
        self.advance();
        n
    }

    fn current_token(&self) -> Token {
        if let Some(token) = self.tokens.get(self.index) {
            token.clone()
        } else {
            Token::EOF
        }
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn parse_multiplication(&mut self, left: i32) -> i32 {
        self.advance();
        let right = self.parse_expr(self.current_precedence());
        left * right
    }

    fn parse_addition(&mut self, left: i32) -> i32 {
        self.advance();
        let right = self.parse_expr(self.current_precedence());
        left + right
    }
}

fn main() {
    let mut parser = Parser::new(vec![]);
    parser.parse_expr(0);
}

#[cfg(test)]
mod tests {
    use crate::{Parser, Token};

    #[test]
    fn test_empty() {
        let mut parser = Parser::new(vec![]);
        let value = parser.parse_expr(0);
        assert!(matches!(parser.current_token(), Token::EOF));
        assert_eq!(value, -1);
    }

    #[test]
    fn test_literal() {
        let mut parser = Parser::new(vec![Token::Literal(1)]);
        let value = parser.parse_expr(0);
        assert!(matches!(parser.current_token(), Token::EOF));
        assert_eq!(value, 1);
    }

    #[test]
    fn test_plus() {
        let mut parser = Parser::new(vec![Token::Literal(1), Token::Plus, Token::Literal(2)]);
        let value = parser.parse_expr(0);
        assert!(matches!(parser.current_token(), Token::EOF));
        assert_eq!(value, 1 + 2);
    }

    #[test]
    fn test_times() {
        let mut parser = Parser::new(vec![Token::Literal(1), Token::Times, Token::Literal(2)]);
        let value = parser.parse_expr(0);
        assert!(matches!(parser.current_token(), Token::EOF));
        assert_eq!(value, 1 * 2);
    }

    #[test]
    fn test_plus_times() {
        let mut parser = Parser::new(vec![
            Token::Literal(1),
            Token::Plus,
            Token::Literal(2),
            Token::Times,
            Token::Literal(3),
        ]);
        let value = parser.parse_expr(0);
        assert!(matches!(parser.current_token(), Token::EOF));
        assert_eq!(value, 1 + 2 * 3);
    }

    #[test]
    fn test_times_plus() {
        let mut parser = Parser::new(vec![
            Token::Literal(1),
            Token::Times,
            Token::Literal(2),
            Token::Plus,
            Token::Literal(3),
        ]);
        let value = parser.parse_expr(0);
        assert!(matches!(parser.current_token(), Token::EOF));
        assert_eq!(value, 1 * 2 + 3);
    }

    #[test]
    fn test_group() {
        let mut parser = Parser::new(vec![
            Token::Literal(1),
            Token::Times,
            Token::LeftParen,
            Token::Literal(2),
            Token::Plus,
            Token::Literal(3),
            Token::RightParen,
        ]);
        let value = parser.parse_expr(0);
        assert!(matches!(parser.current_token(), Token::EOF));
        assert_eq!(value, 1 * (2 + 3));
    }

    #[test]
    fn test_nested_groups() {
        let mut parser = Parser::new(vec![
            Token::Literal(1),
            Token::Times,
            Token::LeftParen,
            Token::Literal(2),
            Token::Plus,
            Token::LeftParen,
            Token::Literal(1),
            Token::Times,
            Token::LeftParen,
            Token::Literal(2),
            Token::Plus,
            Token::Literal(3),
            Token::RightParen,
            Token::RightParen,
            Token::RightParen,
        ]);
        let value = parser.parse_expr(0);
        assert!(matches!(parser.current_token(), Token::EOF));
        assert_eq!(value, 1 * (2 + (1 * (2 + 3))));
    }
}
