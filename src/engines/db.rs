use std::{io::{ErrorKind, Read, Write}, net::{SocketAddr, TcpListener, TcpStream}, time::Duration};


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

pub fn handle_db_client_request(mut stream: &TcpStream) {
    let mut sql: String = String::new();
    let _ = stream.read_to_string(&mut sql);

    stream.write_all(parse_sql(sql)).unwrap();
    stream.flush().unwrap();
}


type TokenKind = usize;
const SELECT        : TokenKind = 0x01;
const UPDATE        : TokenKind = 0x02;
const INSERT        : TokenKind = 0x03;
const DELETE        : TokenKind = 0x04;
const CREATE        : TokenKind = 0x05;
const TABLE         : TokenKind = 0x06;
const COLUMN        : TokenKind = 0x07;
const ALTER         : TokenKind = 0x08;
const PRIMARY       : TokenKind = 0x09;
const KEY           : TokenKind = 0x0A;
const NOT           : TokenKind = 0x0B;
const NULL          : TokenKind = 0x0C;
const INT           : TokenKind = 0x0D;
const VARCHAR       : TokenKind = 0x0E;
const OPEN_PAREN    : TokenKind = 0x0F;
const CLOSE_PAREN   : TokenKind = 0x10;
const OPEN_BRACKET  : TokenKind = 0x11;
const CLOSE_BRACKET : TokenKind = 0x12;
const SEMI_COLON    : TokenKind = 0x13;
const COLON         : TokenKind = 0x14;
const AS            : TokenKind = 0x15;
const JOIN          : TokenKind = 0x16;
const FROM          : TokenKind = 0x17;
const ON            : TokenKind = 0x18;
const LEFT          : TokenKind = 0x1A;
const RIGHT         : TokenKind = 0x1B;
const WHERE         : TokenKind = 0x1C;
const AND           : TokenKind = 0x1D;
const OR            : TokenKind = 0x1E;
const IN            : TokenKind = 0x1F;
const COUNT         : TokenKind = 0x20;
const SUM           : TokenKind = 0x21;
const AVG           : TokenKind = 0x22;
const ASTRISKS      : TokenKind = 0x23;
const DOT           : TokenKind = 0x24;
const GROUP         : TokenKind = 0x25;
const BY            : TokenKind = 0x26;
const ORDER         : TokenKind = 0x27;
const ASC           : TokenKind = 0x28;
const DESC          : TokenKind = 0x29;
const DISTINCT      : TokenKind = 0x30;
const LIMIT         : TokenKind = 0x31;
const OFFSET        : TokenKind = 0x32;
const HAVING        : TokenKind = 0x33;



struct Token {
    kind: TokenKind,
    val: String
}

fn parse_sql(sql: String) -> &'static [u8] {
    "".as_bytes()
}

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