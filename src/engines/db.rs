use std::{io::{ErrorKind, Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, time::Duration};

#[allow(unused)]
pub fn db_query(sql: &str) -> () {
    let database : &'static str  = "hamte";
    let username : &'static str  = "HOS";
    let password : &'static str  = "H0S@hamte";
    let server   : &'static str  = "127.0.0.1";
    let port     : usize         = 3300;
    let url: String = format!("db://{username}:{password}@{server}:{port}/{database}");
    println!("{}",url);
    println!("{}",sql);
    let mut stream = TcpStream::connect(format!("{server}:{port}")).unwrap();
    stream.write_all(sql.as_bytes()).unwrap();
    stream.flush().unwrap();
    let mut result : String = String::new();
    stream.read_to_string(&mut result).unwrap();
    println!("query> {}", result);
    ()
}

#[allow(unused)]
pub fn handle_db_client_request(mut stream: &TcpStream) {
    let mut sql: String = String::new();
    let _ = stream.read_to_string(&mut sql);

    stream.write_all(parse_sql(&sql)).unwrap();
    stream.flush().unwrap();
}


#[allow(unused)]
pub fn parse_sql(sql: &str) -> &'static [u8] {
    "".as_bytes()
}

#[allow(unused)]
pub fn start_db_server() -> () {
    println!("start db server...");

    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 3300)),
    ];

    let listener = TcpListener::bind(&addrs[..]).unwrap();
    let _ = listener.set_nonblocking(true);

    loop {
        match listener.accept() {
            Ok((_stream,_address)) => {
                //let handle = 
                std::thread::spawn(move || handle_db_client_request(&_stream));
                //handle.join().unwrap();                
            }
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // No incoming connection yet, continue or perform other tasks
                // You might want to add a small delay here to avoid busy-waiting
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
                break; // Exit loop on other errors
            }
        }
    }
    ()
}