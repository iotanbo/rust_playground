/// Async TCP server demo
/// 
// https://docs.rs/tokio/1.2.0/tokio/
// https://tokio.rs/tokio/tutorial
// https://github.com/tokio-rs/tokio


/*



Demonstrates shared state usage:
https://tokio.rs/tokio/tutorial/channelss

  Create a dedicated task and use message passing (actor model).

  1. Create separate task `gl_state`
  2. Create message enum:
     IncConnsTotal -> new conn id (usize)
     IncRequestsTotal -> new req count (usize)

  3. Pass tx handler of the gl_state's MPSC queue to every newly created task

*/



use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

//use std::sync::atomic::{AtomicUsize, Ordering};
// use std::sync::{Arc};  // , Mutex

use crate::gs_task::{GsManager};


pub async fn tcp_srv() -> Result<(), Box<dyn std::error::Error>> {

    #[allow(unused)]
    let mut listener = TcpListener::bind("0.0.0.0:9555").await?;

    // Create global state manager task
    let gl_state = GsManager::create(); 

    loop {
        let (mut socket, _) = listener.accept().await?;

        let gl_state = gl_state.clone();

        // Spawn a new tokio task for each new connection. Tasks in Tokio are very lightweight. 
        // Under the hood, they require only a single allocation and 64 bytes of memory.
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            let conn_id = gl_state.inc_conns_total().await.unwrap();
            println!("  + Conn #{} established.", &conn_id);

            // Read socket data in a loop
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => { break },
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        break;
                    }
                };

                let req_count = gl_state.inc_requests_total().await.unwrap();
                println!("  * (conn #{}) received request #{}, {} byte(s).", &conn_id, &req_count, n);

                const OK_HTML_HEADER: &str = r#"<!DOCTYPE html>
                <html lang="en">
                <head>
                    <meta charset="utf-8">
                    <title>Привітання від Раста.</title>
                </head>
                <body>
                    <h1>Асинхронний Раст сервер вас привітав!</h1>
                    <p>"#;

                const OK_HTML_FOOTER: &str = r#"</p>
                </body>
                </html>"#;

                let generated_ok_html = format!("{}Conn #{}, Req #{}{}", OK_HTML_HEADER, 
                    &conn_id, &req_count, OK_HTML_FOOTER);


                let ok_resp = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    generated_ok_html.len(),
                    generated_ok_html
                );

                // Write the response
                if let Err(e) = socket.write_all(ok_resp.as_bytes()).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }

            println!("  - Conn #{} closed.", &conn_id);
        });
    }
}
