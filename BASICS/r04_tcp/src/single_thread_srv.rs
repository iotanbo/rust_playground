
use std::io::prelude::*;  // for stream.read
use std::net:: { TcpListener, TcpStream };


#[allow(unused)]
pub fn run() {
    let listener = TcpListener::bind("127.0.0.1:9555").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        //println!("Connection established!");
        handle_connection(stream);
    }
}


#[allow(unused)]
fn handle_connection(mut stream: TcpStream) {

    const HTML: &str = r#"<!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="utf-8">
        <title>Привітання від Раста.</title>
    </head>
    <body>
        <h1>Раст вас привітав!</h1>
        <p>Радійте!</p>
    </body>
    </html>
    "#;


    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        HTML.len(),
        HTML
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}
