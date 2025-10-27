use std::time::Duration;

use crate::engines::db::{start_db_server, db_query};
use crate::engines::web::{start_web_server};

mod engines;

fn main() -> std::io::Result<()> {
    
    std::thread::spawn(move || {
        start_db_server();
    });
    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(2000));        
        db_query("sql");
    });

    start_web_server();
    
    Ok(())
}




