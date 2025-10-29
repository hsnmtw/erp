use std::path::{Path, PathBuf};

use crate::engines::db::{filemgr::{fs_type, list_fs}, grammer::{self, join}, lexer::Lexer, tokens::TokenKinds};

pub const HOME_DIR : &'static str = "./databases"; 

pub fn execute_sql(db: &str, lex: &Lexer) -> Option<String> {
    //let lex = Lexer::new(sql);
    if let Some(err) = grammer::get_error(&lex) {
        return Some(err);
    }
    match lex.tokens[0].kind {
        &TokenKinds::SHOW   => execute_show(lex), 
        &TokenKinds::CREATE => execute_create(lex), 
        _ => None
    }
}

fn execute_create(lex: &Lexer) -> Option<String> {
    match lex.tokens[1].kind {
        &TokenKinds::DATABASE => execute_create_database(lex),
        &TokenKinds::TABLE    => execute_create_table(lex),
        &TokenKinds::VIEW     => execute_create_view(lex),
        _ => None
    }
}

fn execute_create_view(lex: &Lexer) -> Option<String> {
    todo!()
}

fn execute_create_table(lex: &Lexer) -> Option<String> {
    todo!()
}

fn execute_create_database(lex: &Lexer) -> Option<String> {
    // database could exists before
    // name could be invalid
    let db = lex.tokens[2].val.as_str();
    let folder = format!("{}/{}", HOME_DIR, db);
    let path = Path::new(folder.as_str());
    if path.exists() {
        return Some(format!("SQL ERROR: failed to create database [{}] because it exists already with same name",db));
    }
    if let Err(err) = std::fs::create_dir(path) {
        return Some(err.to_string());
    }
    Some("database was created !".into())
}


fn execute_show(lex: &Lexer) -> Option<String> {
    match lex.tokens[1].kind {
        &TokenKinds::DATABASES => get_databases(),
        &TokenKinds::TABLES    => get_tables(),
        &TokenKinds::VIEWES    => get_views(),
        _ => Some(format!("ERROR: unknown command SHOW {:?}", lex.tokens[1].kind))
    }
}

fn get_views() -> Option<String> {
    todo!()
}

fn get_tables() -> Option<String> {
    todo!()
}

/* 
    a database is a folder in the home directory         
*/
fn get_databases() -> Option<String> {
    // let home = Path::new(HOME_DIR);
    let folders = list_fs(HOME_DIR, &fs_type::DIRECTORY);
    Some(folders.join("\n"))
}

