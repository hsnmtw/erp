use std::path::{Path};

use crate::engines::db::{filemgr::{FsType, list_fs}, grammer::{self}, lexer::Lexer, tokens::TokenKinds};

pub const HOME_DIR : &'static str = "./databases"; 

pub fn execute_sql(db: &str, lex: &Lexer) -> Option<String> {
    //let lex = Lexer::new(sql);
    if let Some(err) = grammer::get_error(&lex) {
        return Some(err);
    }
    match lex.tokens[0].kind {
        &TokenKinds::SHOW   => execute_show(lex,db), 
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

#[allow(unused)]
fn execute_create_view(lex: &Lexer) -> Option<String> {
    todo!()
}

#[allow(unused)]
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
    std::fs::create_dir(Path::new((format!("{}/{}/tables",HOME_DIR,db)).as_str())).unwrap();
    std::fs::create_dir(Path::new((format!("{}/{}/views",HOME_DIR,db)).as_str())).unwrap();
    Some("database was created !".into())
}


fn execute_show(lex: &Lexer, db : &str) -> Option<String> {
    match lex.tokens[1].kind {
        &TokenKinds::DATABASES => get_databases(),
        &TokenKinds::TABLES    => get_tables(db),
        &TokenKinds::VIEWES    => get_views(db),
        _ => Some(format!("ERROR: unknown command SHOW {:?}", lex.tokens[1].kind))
    }
}

fn get_views(db : &str) -> Option<String> {
    let folders = list_fs((format!("{}/{}/views",HOME_DIR,db)).as_str(), &FsType::FILE);
    Some(folders.join("\n"))
}

fn get_tables(db : &str) -> Option<String> {
    let folders = list_fs((format!("{}/{}/tables",HOME_DIR,db)).as_str(), &FsType::FILE);
    Some(folders.join("\n"))
}

fn get_databases() -> Option<String> {
    let folders = list_fs(HOME_DIR, &FsType::DIRECTORY);
    Some(folders.join("\n"))
}

