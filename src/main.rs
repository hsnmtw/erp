// use std::time::Duration;

// use crate::engines::db::{start_db_server, db_query};
// use crate::engines::web::{start_web_server};

use crate::engines::{lexer::Lexer, tokens::TokenKinds};

mod engines;

fn main() -> std::io::Result<()> {
    
    // std::thread::spawn(move || {
    //     start_db_server();
    // });
    // std::thread::spawn(move || {
    //     std::thread::sleep(Duration::from_millis(2000));        
    //     db_query("sql");
    // });

    // start_web_server();

    println!("");
    println!("");
    println!("");
    println!("");
    let expr = "select u.* from [all users] as u where u.[id] > 0.0 ;";
    println!("-----------------------------------------");
    println!("{}",expr);
    println!("-----------------------------------------");

    for token in Lexer::new(expr).tokens {
        if token.kind == &TokenKinds::SPACE { continue; }
        println!("{}",token.to_string());
    }
    
    Ok(())
}




