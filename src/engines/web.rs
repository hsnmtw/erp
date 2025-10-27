use std::fmt::{Display, Result, Formatter};
use std::io::{ErrorKind, Read, Write};
use std::fs::{File};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream};
use std::path::{Path};
use std::time::{Duration};


const HTTP_VERSION: &'static str = "HTTP/1.1";
#[derive(Debug)]
struct HttpStatus {
    code: usize,
    name: &'static str
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let code = self.code;
        let name = self.name;
        write!(f, "{code} {name}")
    }
}

impl HttpStatus {
    fn ok() -> HttpStatus { HttpStatus { code: 200, name: "OK" } }
    fn not_found() -> HttpStatus { HttpStatus { code: 404, name: "NOT FOUND" } }
    fn server_error() -> HttpStatus { HttpStatus { code: 500, name: "SERVER ERROR" } }
}

pub fn start_web_server() {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([127, 0, 0, 1], 443)),
    ];

    let listener = TcpListener::bind(&addrs[..]).unwrap();
    let _ = listener.set_nonblocking(true);

    loop {
        match listener.accept() {
            Ok((_stream,_address)) => {
                //let handle = 
                std::thread::spawn(move || handle_http_client_request(&_stream));
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
}

fn handle_http_client_request(mut stream: &TcpStream) {
    let mut content: String = String::new();
    let _ = stream.read_to_string(&mut content);
    //println!("content: {content}");
    let lines : Vec<&str> = content.split('\n').collect();
    let words : Vec<&str> = lines[0].split(' ').collect();
    if words.len() < 2 {
        stream.write_all(format!("{} {}\n\nincorrect request !",HTTP_VERSION,HttpStatus::server_error()).as_bytes()).unwrap();
        stream.flush().unwrap();
        stream.shutdown(Shutdown::Write).unwrap();
        stream.read(&mut [0; 128]).unwrap_or(0);
        return;
    }
    let method: &str = words[0];
    let path: &str = words[1];

    println!("[inf] new request {method} {path}");

    let data = match (method,path) {
        ("GET","/") => get_file_contents(&format!("./assets/html/index.html")),
        ("GET",x) if x.starts_with("/assets/") => get_file_contents(&format!(".{x}")),
        _ => HttpStatus::not_found().to_string()
    };

    let content_type: String = match data.as_str() {
        "404 NOT FOUND" => String::from("text/plain"),
        _ => get_content_type(path)
    };

    let status = match data.as_str() {
        "404 NOT FOUND" => HttpStatus::not_found(),
        _ => HttpStatus::ok()
    };

    let response: &str = &format!(
    "{HTTP_VERSION} {status}
Connection: close    
Content-Type: {content_type}

{data}");

    stream.write_all( response.as_bytes()).unwrap();
    stream.flush().unwrap();
    stream.shutdown(Shutdown::Write).unwrap();
    stream.read(&mut [0; 128]).unwrap_or(0);
}

fn get_content_type(path: &str) -> String {
    let _ex = path.split('.').last().unwrap_or(".").to_lowercase();
    let ext = _ex.as_str();//.unwrap().as_str();
    match ext {
        "js"    | "mjs"     => String::from("text/javascript"),
        "css"               => String::from("text/css"),
        "eot"   | "woff"     | 
        "woff2" | "ttf"     => format!("font/{ext}"), 
        "png"|"gif"|"bmp"|
        "tif"|"tiff"|"jpg"|
        "jpeg"|"bnp"|"ico"  => format!("image/{ext}"),
                          _ => String::from("text/html")
    }
}

fn get_file_contents(path: &str) -> String {
    if !Path::new(path).exists() {
        return HttpStatus::not_found().to_string();
    }
    let mut content : String = String::new();
    let _ = File::open(path).unwrap().read_to_string(&mut content);
    content
}