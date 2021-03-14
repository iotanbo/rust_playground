/// Async TCP server demo
/// 
// https://docs.rs/tokio/1.2.0/tokio/
// https://tokio.rs/tokio/tutorial
// https://github.com/tokio-rs/tokio


/*
TOKIO basics.
* 


Shared state.
https://tokio.rs/tokio/tutorial/shared-state

* 3 basic strategies to eliminate contention when accessing shared data:
  1) Sharding: e.g., instead of locking the entire vector with mutex, 
     better to create few vectors and lock only the required one.
     Works well when data that can be split into few independent sets.

  2) Create a dedicated task and use message passing (actor model).

  3) Restructure the code to avoid the mutex.


*/


use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc};  // , Mutex

// type Db = Arc<Mutex<HashMap<String, Bytes>>>;


struct GlobalState {

    requests_total: AtomicUsize,
    conns_total: AtomicUsize,

}


pub async fn tcp_srv() -> Result<(), Box<dyn std::error::Error>> {

    #[allow(unused)]
    let mut listener = TcpListener::bind("0.0.0.0:9555").await?;

    // Wrap global state in ARC
    let gl_state = Arc::new( GlobalState{ 
        conns_total: AtomicUsize::new(0), 
        requests_total: AtomicUsize::new(0), });

    loop {
        let (mut socket, _) = listener.accept().await?;

        // Create a new atomic reference to global state that will be moved to another thread
        let gl_state = gl_state.clone();

        // Spawn a new tokio task for each new connection. Tasks in Tokio are very lightweight. 
        // Under the hood, they require only a single allocation and 64 bytes of memory.
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // Increment atomic conn. counter
            // How it works:
            //   1. Load current shared value
            //   2. Try atomically modifying it if its value is unchanged
            //   3. If it has been changed by another thread, retry
            let mut conn_id = gl_state.conns_total.load(Ordering::Relaxed);
            loop {
                let new_conn_count = conn_id + 1;
                match gl_state.conns_total.compare_exchange_weak(conn_id, new_conn_count, Ordering::SeqCst, Ordering::Relaxed) {
                    Ok(_) => { conn_id = new_conn_count; break; },
                    Err(x) => conn_id = x,
                }
            }
            println!("  * Conn #: {}", &conn_id);

            // Read socket data in a loop
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Increment atomic req. counter
                let mut req_count = gl_state.requests_total.load(Ordering::Relaxed);
                loop {
                    let new_req_count = req_count + 1;
                    match gl_state.requests_total.compare_exchange_weak(req_count, new_req_count, Ordering::SeqCst, Ordering::Relaxed) {
                        Ok(_) => { req_count = new_req_count; break; },
                        Err(x) => req_count = x,
                    }
                }

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
        });
    }
}
