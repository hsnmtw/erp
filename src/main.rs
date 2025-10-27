// use std::time::Duration;

// use crate::engines::db::{start_db_server, db_query};
// use crate::engines::web::{start_web_server};

use crate::engines::{lexer::Lexer, tokens};

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
    let expr = "1     +    [2+3]   +   (4*6)";
    println!("-----------------------------------------");
    println!("{}",expr);
    println!("-----------------------------------------");

    for token in Lexer::new(expr).tokens {
        println!("{}",token.to_string());
    }
    
    Ok(())
}




