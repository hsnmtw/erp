use std::fmt::{Debug};

use crate::engines::db::{lexer::Lexer, tokens::TokenKinds};

struct exp_next {
    show : &'static[&'static TokenKinds],
} 

const fn get_accepted_next(kind : &TokenKinds) -> &[&TokenKinds] {
    match kind {
        &TokenKinds::SHOW => &[
            &TokenKinds::DATABASES,
            &TokenKinds::TABLES,
            &TokenKinds::VIEWES,
        ],
        &TokenKinds::USE => &[
            &TokenKinds::IDENTIFIER,
        ],
        &TokenKinds::EXIT|&TokenKinds::QUIT => &[
            &TokenKinds::SEMI_COLON,
        ],
        &TokenKinds::DATABASES|
        &TokenKinds::TABLES   |
        &TokenKinds::VIEWES   => &[ 
            &TokenKinds::SEMI_COLON 
        ],

        &TokenKinds::SELECT => &[
            &TokenKinds::ASTRISKS,
            &TokenKinds::IDENTIFIER,
        ],
        &TokenKinds::TABLE|
        &TokenKinds::VIEW|
        &TokenKinds::UPDATE|
        &TokenKinds::FROM|
        &TokenKinds::INTO => &[
            &TokenKinds::IDENTIFIER,
        ],
        &TokenKinds::CREATE => &[
            &TokenKinds::DATABASE,
            &TokenKinds::TABLE,
            &TokenKinds::VIEW,
        ],
        &TokenKinds::VALUES => &[
            &TokenKinds::OPEN_PAREN,
        ],
        &TokenKinds::INSERT => &[
            &TokenKinds::INTO,
        ],
        &TokenKinds::DELETE => &[
            &TokenKinds::FROM,
        ],
        &TokenKinds::WHERE => &[
            &TokenKinds::IDENTIFIER,
            &TokenKinds::OPEN_PAREN,
        ],
        _ => &[]
    }
}

pub fn join<T:Debug>(sep : &str, values : &[&T]) -> String {
    let mut result = String::new();
    for item in values {
        result += &format!("{:?}{}", item, sep);
    }

    result
}

pub fn get_error(lex: &Lexer) -> Option<String> {

    if lex.tokens.len() < 2 {
        return Some("the minimum number of tokens for a valid sql is 3".into());
    }

    for idx in 0..lex.tokens.len()-1 {
        let curr = lex.tokens[idx+0].kind;
        let next = lex.tokens[idx+1].kind;
        let accepted_next_term = get_accepted_next(curr);
        
        if accepted_next_term.len()> 0 && !accepted_next_term.contains(&next) {
            let accepted_tokens = join(",",accepted_next_term);
            return Some(format!("SQL ERROR: pos[{}] expected [{}] after {:?}, but got <{}> ", idx, accepted_tokens, curr, lex.tokens[idx+1].to_string()));
        }
    }

    match lex.tokens[0].kind {
        &TokenKinds::SELECT => validate_select(lex),
        &TokenKinds::UPDATE => validate_update(lex),
        &TokenKinds::INSERT => validate_insert(lex),
        &TokenKinds::DELETE => validate_delete(lex),
        &TokenKinds::CREATE => validate_create(lex),
        &TokenKinds::SHOW   => validate_show(lex),
        &TokenKinds::USE    => validate_use(lex),
                          _ => None
    }
        
    // None
}

fn validate_use(lex: &Lexer) -> Option<String> {
    /* 
        use [database name];
    */
    if lex.tokens.len() != 3 
    || lex.tokens[0].kind != &TokenKinds::USE
    || lex.tokens[1].kind != &TokenKinds::IDENTIFIER
    || lex.tokens[1].val.len() == 0
    || lex.tokens[2].kind != &TokenKinds::SEMI_COLON {
        return Some(format!("SQL ERROR: the use statement contains invalid argument '{}'", lex.to_string()));
    }

    None
}

#[allow(unused)]
fn validate_show(lex: &Lexer) -> Option<String> {
    None
}

#[allow(unused)]
fn validate_select(lex: &Lexer) -> Option<String> {
    None
}

#[allow(unused)]
fn validate_update(lex: &Lexer) -> Option<String> {
    None
}

#[allow(unused)]
fn validate_insert(lex: &Lexer) -> Option<String> {
    /*
        1. INSERT INTO [TABLE] {Optional}([c1],[c2],[c3],...) VALUES (v1,v2,v3,...);
        2. INSERT INTO [TABLE] {Optional}([c1],[c2],[c3],...) SELECT  v1,v2,v3,...;

        min length: 6
    */
    if lex.tokens.len() < 6 { return Some(format!("SQL ERROR: minimum number of tokens for INSERT statement is {} while the number of provided tokens is {}", 6, lex.tokens.len())); }
    if lex.tokens[0].kind != &TokenKinds::INSERT     { return Some(format!("SQL ERROR: token[{}] first token must be INSERT", 0)); }
    if lex.tokens[1].kind != &TokenKinds::INTO       { return Some(format!("SQL ERROR: token[{}] expected '{:?}' but got '{}'",1, &TokenKinds::INTO,       lex.tokens[1].to_string())); }
    if lex.tokens[2].kind != &TokenKinds::IDENTIFIER { return Some(format!("SQL ERROR: token[{}] expected '{:?}' but got '{}'",2, &TokenKinds::IDENTIFIER, lex.tokens[2].to_string())); }
    if lex.tokens[3].kind != &TokenKinds::OPEN_PAREN 
    && lex.tokens[4].kind != &TokenKinds::SELECT 
    && lex.tokens[4].kind != &TokenKinds::VALUES     { return Some(format!("SQL ERROR: token[{}] expected '{:?}' but got '{}'",3, &[&TokenKinds::VALUES,&TokenKinds::SELECT], lex.tokens[4].to_string())); }
    if lex.tokens[3].kind != &TokenKinds::OPEN_PAREN 
    && lex.tokens[4].kind == &TokenKinds::VALUES     
    && lex.tokens[5].kind != &TokenKinds::OPEN_PAREN { return Some(format!("SQL ERROR: token[{}] expected '{:?}' but got '{}'",5, &TokenKinds::OPEN_PAREN, lex.tokens[5].to_string())); }
    if lex.tokens[3].kind != &TokenKinds::OPEN_PAREN 
    && lex.tokens[4].kind == &TokenKinds::SELECT     
    && lex.tokens[5].kind == &TokenKinds::OPEN_PAREN { return Some(format!("SQL ERROR: token[{}] expected '{:?}' but got '{}'",5, &[&TokenKinds::IDENTIFIER,&TokenKinds::NUMBER,&TokenKinds::STRING], lex.tokens[5].to_string())); }

    let mut periods_col : u8 = 0;
    let mut periods_val : u8 = 0;
    let mut in_middle : bool = false;
    for token in lex.tokens.iter() {
        if token.kind == &TokenKinds::PERIOD {
            if in_middle { periods_val += 1; }
            else         { periods_col += 1; }
        }
        if token.kind == &TokenKinds::SELECT || token.kind ==  &TokenKinds::VALUES { in_middle = true; }
    }

    if periods_col > 0 && periods_col != periods_val {
        return Some(format!("SQL ERROR: the number of columns {} is not same as the number of provided values {}", periods_col+1, periods_val+1));
    }

    None
}

#[allow(unused)]
fn validate_delete(lex: &Lexer) -> Option<String> {
    None
}

#[allow(unused)]
fn validate_create(lex: &Lexer) -> Option<String> {
    if lex.tokens.len() < 4 {
        return Some("SQL ERROR: low number of arguments to create an object !".into());
    }
    /* 
        CREATE DATABASE [db_name];
        CREATE TABLE [table_name] ( [column_name] data_type <not null> <default ?> <primary key> <unique>, ... ); 
        CREATE VIEW [view_name] AS {SELECT * FROM [table] <WHERE condition <AND condition>|OR condition>>}; 
    */
    match lex.tokens[1].kind {
         &TokenKinds::DATABASE => {
            if lex.tokens.len() > 4 {
                return Some("SQL ERROR: incorrect number of arguments to create database !".into());
            }
            if lex.tokens[2].kind != &TokenKinds::IDENTIFIER {
                return Some(format!("SQL ERROR: expected the name of the database but got {}", lex.tokens[1].to_string()));
            }
            None
         },
         &TokenKinds::TABLE
        |&TokenKinds::VIEW => None,
        _ => Some(format!("expected {:?} but got {:?}", &[&TokenKinds::DATABASE,&TokenKinds::TABLE,&TokenKinds::VIEW], lex.tokens[1].kind))
    }
    
    // None
}