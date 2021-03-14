/// Single-threaded token checker server



use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::rc::Rc;
use std::cell::Cell;


struct GlobalState {
    conns_total: Cell<u64>,
    requests_total: Cell<u64>,

}

impl GlobalState {

    fn inc_conns_total(&self) -> u64 {
        let mut conns_total = self.conns_total.get();
        conns_total += 1;
        self.conns_total.set(conns_total);
        conns_total
    }

    fn inc_requests_total(&self) -> u64  {
        let mut requests_total = self.requests_total.get();
        requests_total += 1;
        self.requests_total.set(requests_total);
        requests_total
    }
}


pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {

    #[allow(unused)]
    let mut listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    // Create global state and wrap it into Rc so that it can be shared between tasks
    let gl_state = Rc::new( GlobalState{ 
        conns_total: Cell::new(0), 
        requests_total: Cell::new(0), });

    loop {
        let (mut socket, _) = listener.accept().await?;

        // Create a new reference to global state that will be moved to another thread
        let gl_state = gl_state.clone();

        // Spawn a new local task for each new connection. Tasks in Tokio are very lightweight. 
        // Under the hood, they require only a single allocation and 64 bytes of memory.
        tokio::task::spawn_local(async move {
            let mut buf = [0; 1024];

            // Increment conn. counter
            let conn_id = gl_state.inc_conns_total();
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

                // Increment request counter
                let req_total = gl_state.inc_requests_total();
                println!("  * (conn #{}) received request #{}, {} byte(s).", 
                        &conn_id, &req_total, n);

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
                    &conn_id, &req_total, OK_HTML_FOOTER);


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
