// use std::time::Duration;

// use crate::engines::db::{start_db_server, db_query};
// use crate::engines::web::{start_web_server};

use std::{io::{Write, stdin, stdout}, path::Path};

use crate::engines::db::{lexer::Lexer, sql, tokens::TokenKinds};

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
    // let expr = "insert into [all users] (id,name) values (@0,@1,@3);";
    // println!("-----------------------------------------");
    // println!("{}",expr);
    // println!("-----------------------------------------");


    let mut db = String::new();

    loop {
        print!("{}> ", db);
        stdout().flush()?;
        let mut buf : String = String::new();
        stdin().read_line(&mut buf)?;
        let lex = Lexer::new(&buf);
        // if let Some(err) = grammer::get_error(db.as_str(), &lex) {
        //     println!("[FAILED] {err}");
        //     continue;
        // }
        match lex.tokens[0].kind {
            &TokenKinds::USE => {
                let folder = format!("{}/{}", sql::HOME_DIR, lex.tokens[1].val);
                let path = Path::new(folder.as_str());
                if !path.exists() {
                    eprintln!("SQL ERROR: incorrect db [{}]", lex.tokens[1].val);
                    continue;
                }
                db = format!("{}", lex.tokens[1].val);
                continue;
            },
            &TokenKinds::QUIT|&TokenKinds::EXIT => {
                break;
            },
            _ => {}
        }        
        for token in &lex.tokens {
            println!("{}", token.to_string());
        }
        println!("{}", sql::execute_sql(&db,&lex).unwrap_or("failed !!! for some unknown reason".into()));
    }
    
    println!("good bye !");
    Ok(())
}




