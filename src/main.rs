// use std::time::Duration;

// use crate::engines::db::{start_db_server, db_query};
// use crate::engines::web::{start_web_server};

use crate::engines::db::{grammer::get_error, lexer::Lexer};

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
    // let expr = "select [u].* from [all users] as [u] where [u].[id] >= @0 ;";
    let expr = "insert into [all users] (id,name) values (@0,@1,@3);";
    println!("-----------------------------------------");
    println!("{}",expr);
    println!("-----------------------------------------");

    let lex = Lexer::new(expr);

    // for token in &lex.tokens {
    //     println!("{}",token.to_string());
    // }
    
    if let Some(x) = get_error(&lex) {
        println!("{x}");
    }
    
    Ok(())
}




