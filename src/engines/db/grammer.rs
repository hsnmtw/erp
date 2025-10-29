use std::{collections::HashMap, fmt::Debug, path::Iter};

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

    if lex.tokens.len() < 3 {
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
                          _ => None
    }
        
    // None
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
        1. INSERT INTO [TABLE] {Optional}([c1],[c2],[c3],...) SELECT  v1,v2,v3,...;
    */
    None
}

#[allow(unused)]
fn validate_delete(lex: &Lexer) -> Option<String> {
    None
}

#[allow(unused)]
fn validate_create(lex: &Lexer) -> Option<String> {
    None
}