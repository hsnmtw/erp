use crate::engines::db::{tokens::{Token, TokenKinds}};

pub struct Lexer {
    pub tokens : Vec<Token>,
    source     : Vec<char>,
    sql        : String,
    pos        : usize
}


impl Lexer {
    fn current(&self) -> char {
        if self.pos < self.source.len() {
            return self.source[self.pos];
        }
        char::MAX
    }

    pub fn to_string(&self) -> &str {
        &self.sql
    }

    fn next(&self) -> char {
        if self.has_next() {
            return self.source[self.pos + 1];
        }
        char::MAX
    }

    fn has_next(&self) -> bool {
        self.pos + 1 < self.source.len()
    }

    fn done(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn advance(&mut self) {
        let mut c : char = self.current();
        let mut s : String = format!("{}",c);

        if c.is_whitespace() {
            while self.next().is_whitespace() {
                self.pos += 1;
            }
            return;
        }
        
        if c.is_ascii_digit() || (c == '.' && self.next().is_ascii_digit()) {
            s = "".into();
            while self.has_next() {
                c = self.current();
                if !(c.is_ascii_digit()|| (c == '.' && self.next().is_ascii_digit())) { 
                    self.pos -= 1;
                    break; 
                }
                s = format!("{s}{c}");                
                self.pos += 1;
            }
            self.tokens.push(Token::new(&TokenKinds::NUMBER, s));
            return;
        }
        
        let is_bracket = c == '[';
        if c.is_ascii_lowercase() || c.is_ascii_uppercase() || c=='_' || is_bracket {
            if is_bracket {
                self.pos += 1;
            }
            s = "".into();
            while self.has_next() {
                c = self.current();
                if (!(is_bracket && c==' ') && !(c=='_'||c.is_ascii_alphanumeric())) || (is_bracket && self.current() == ']') { 
                    if self.current() != ']' {
                        self.pos -= 1;
                    }
                    break; 
                }
                s = format!("{s}{c}");
                self.pos += 1;
            }
            //self.tokens.push(Token::new(&TokenKinds::STRING, s));
            match s.to_uppercase().as_str() {
                "EXIT"       => self.tokens.push(Token::new(&TokenKinds::EXIT    , s)),
                "QUIT"       => self.tokens.push(Token::new(&TokenKinds::QUIT    , s)),
                "DEFAULT"    => self.tokens.push(Token::new(&TokenKinds::DEFAULT , s)),
                "UNIQUE"     => self.tokens.push(Token::new(&TokenKinds::UNIQUE  , s)),
                "USE"        => self.tokens.push(Token::new(&TokenKinds::USE     , s)),
                "SELECT"     => self.tokens.push(Token::new(&TokenKinds::SELECT  , s)),
                "UPDATE"     => self.tokens.push(Token::new(&TokenKinds::UPDATE  , s)),
                "INSERT"     => self.tokens.push(Token::new(&TokenKinds::INSERT  , s)),
                "INTO"       => self.tokens.push(Token::new(&TokenKinds::INTO    , s)),
                "VALUES"     => self.tokens.push(Token::new(&TokenKinds::VALUES  , s)),
                "DELETE"     => self.tokens.push(Token::new(&TokenKinds::DELETE  , s)),
                "CREATE"     => self.tokens.push(Token::new(&TokenKinds::CREATE  , s)),
                "TABLE"      => self.tokens.push(Token::new(&TokenKinds::TABLE   , s)),
                "COLUMN"     => self.tokens.push(Token::new(&TokenKinds::COLUMN  , s)),
                "ALTER"      => self.tokens.push(Token::new(&TokenKinds::ALTER   , s)),
                "PRIMARY"    => self.tokens.push(Token::new(&TokenKinds::PRIMARY , s)),
                "KEY"        => self.tokens.push(Token::new(&TokenKinds::KEY     , s)),
                "NOT"        => self.tokens.push(Token::new(&TokenKinds::NOT     , s)),
                "NULL"       => self.tokens.push(Token::new(&TokenKinds::NULL    , s)),
                "INT"        => self.tokens.push(Token::new(&TokenKinds::INT     , s)),
                "VARCHAR"    => self.tokens.push(Token::new(&TokenKinds::VARCHAR , s)),
                "AS"         => self.tokens.push(Token::new(&TokenKinds::AS      , s)),
                "JOIN"       => self.tokens.push(Token::new(&TokenKinds::JOIN    , s)),
                "FROM"       => self.tokens.push(Token::new(&TokenKinds::FROM    , s)),
                "ON"         => self.tokens.push(Token::new(&TokenKinds::ON      , s)),
                "LEFT"       => self.tokens.push(Token::new(&TokenKinds::LEFT    , s)),
                "RIGHT"      => self.tokens.push(Token::new(&TokenKinds::RIGHT   , s)),
                "WHERE"      => self.tokens.push(Token::new(&TokenKinds::WHERE   , s)),
                "AND"        => self.tokens.push(Token::new(&TokenKinds::AND     , s)),
                "OR"         => self.tokens.push(Token::new(&TokenKinds::OR      , s)),
                "IN"         => self.tokens.push(Token::new(&TokenKinds::IN      , s)),
                "COUNT"      => self.tokens.push(Token::new(&TokenKinds::COUNT   , s)),
                "SUM"        => self.tokens.push(Token::new(&TokenKinds::SUM     , s)),
                "AVG"        => self.tokens.push(Token::new(&TokenKinds::AVG     , s)),
                "GROUP"      => self.tokens.push(Token::new(&TokenKinds::GROUP   , s)),
                "BY"         => self.tokens.push(Token::new(&TokenKinds::BY      , s)),
                "ORDER"      => self.tokens.push(Token::new(&TokenKinds::ORDER   , s)),
                "ASC"        => self.tokens.push(Token::new(&TokenKinds::ASC     , s)),
                "DESC"       => self.tokens.push(Token::new(&TokenKinds::DESC    , s)),
                "DISTINCT"   => self.tokens.push(Token::new(&TokenKinds::DISTINCT, s)),
                "LIMIT"      => self.tokens.push(Token::new(&TokenKinds::LIMIT   , s)),
                "OFFSET"     => self.tokens.push(Token::new(&TokenKinds::OFFSET  , s)),
                "SHOW"       => self.tokens.push(Token::new(&TokenKinds::SHOW    , s)),
                "DATABASES"  => self.tokens.push(Token::new(&TokenKinds::DATABASES, s)),
                "TABLES"     => self.tokens.push(Token::new(&TokenKinds::TABLES  , s)),
                "VIEWES"     => self.tokens.push(Token::new(&TokenKinds::VIEWES  , s)),
                "DATABASE"   => self.tokens.push(Token::new(&TokenKinds::DATABASE, s)),
                "VIEW"       => self.tokens.push(Token::new(&TokenKinds::VIEW    , s)),
                "HAVING"     => self.tokens.push(Token::new(&TokenKinds::HAVING  , s)),
                           _ => self.tokens.push(Token::new(&TokenKinds::IDENTIFIER  , s)),
            }
            return;
        }

        if c == '\'' {
            // if we get to a single quote, we keep reading until we get matching closing single quote
            // the token is string, and should not include quotes
            //self.tokens.push(Token::new(&TokenKinds::SINGLE_QUOTE, s));
            self.pos += 1;
            s = "".into();
            while self.has_next() {
                c = self.current();
                if c == '\'' { 
                    // self.pos += 1;
                    break; 
                }
                s = format!("{s}{c}");
                self.pos += 1;
            }
            // self.pos -= 1;
            self.tokens.push(Token::new(&TokenKinds::STRING, s));
            return;
        }

        match c {
            '[' => self.tokens.push(Token::new(&TokenKinds::OPEN_BRACKET , s)),
            ']' => self.tokens.push(Token::new(&TokenKinds::CLOSE_BRACKET, s)),
            '(' => self.tokens.push(Token::new(&TokenKinds::OPEN_PAREN   , s)),
            ')' => self.tokens.push(Token::new(&TokenKinds::CLOSE_PAREN  , s)),
            '-' => self.tokens.push(Token::new(&TokenKinds::MINUS        , s)),
            '+' => self.tokens.push(Token::new(&TokenKinds::PLUS         , s)),
            '*' => self.tokens.push(Token::new(&TokenKinds::ASTRISKS     , s)),
            '/' => self.tokens.push(Token::new(&TokenKinds::DIVIDE       , s)),
            '.' => self.tokens.push(Token::new(&TokenKinds::DOT          , s)),
            ';' => self.tokens.push(Token::new(&TokenKinds::SEMI_COLON   , s)),
            ':' => self.tokens.push(Token::new(&TokenKinds::COLON        , s)),
            ',' => self.tokens.push(Token::new(&TokenKinds::PERIOD       , s)),
            '=' => self.tokens.push(Token::new(&TokenKinds::EQUALS       , s)),
            '@' => {
                while self.next().is_ascii_digit() {
                    self.pos += 1;                    
                    c = self.current();
                    s = format!("{s}{c}");
                }
                self.tokens.push(Token::new(&TokenKinds::PARAMETER, s));
            },
            '!' | '>' | '<' => {
                //if next char is equal, then it is not-equal token
                if self.next() == '=' {
                    let kind = match c {
                        '!' => &TokenKinds::NOT_EQUALS,
                        '>' => &TokenKinds::MORE_THAN_EQUAL,
                        '<' => &TokenKinds::LESS_THAN_EQUAL,
                          _ => &TokenKinds::NOT
                    };
                    self.tokens.push(Token::new(kind, format!("{c}=")));
                    self.pos += 1;
                } else {
                    let kind = match c {
                        '>' => &TokenKinds::MORE_THAN,
                        '<' => &TokenKinds::LESS_THAN,
                          _ => &TokenKinds::NOT
                    };
                    self.tokens.push(Token::new(kind, s))
                }
            },
            _ => {}
        };
        return;
    }

    pub fn new(source : &str) -> Lexer {
        let mut lex = Lexer { 
            tokens: Vec::new(), 
            source: source.chars().collect(), 
            sql: source.into(),
            pos: 0 
        };
        while !lex.done() {
            lex.advance();
            lex.pos += 1;
        }
        lex
    }
}