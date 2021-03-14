/// Single-threaded token checker server



use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};  // , AsyncWriteExt
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;


struct GlobalState {
    conns_total: Cell<u64>,
    requests_total: Cell<u64>,

    // Token storage, key is an array of 16 bytes, value is access counter
    token_table: RefCell<HashMap<[u8; 16], u64>>,
}


impl GlobalState {

    /// Initialize global state with default values
    fn init() -> GlobalState {

        let mut gs = GlobalState {
            conns_total: Cell::new(0), 
            requests_total: Cell::new(0), 
            token_table: RefCell::new(HashMap::new()),
        };

        fn key_from_u128(i: u128) -> [u8; 16] {
            let bytes = i.to_le_bytes();
            // println!("  * bytes from {}: {:?}", i, &bytes);
            bytes
        }

        // Insert some dummy values into the token table
        let table = gs.token_table.get_mut();
        for i in 0_u64..1000000 {
            //tt.entry(&[0_u8, 0, 0, 0, 0, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]).or_insert(0);
            table.insert(key_from_u128(i as u128), 0);
        }

        // And 5 valid test keys
        table.insert([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 1], 0);
        table.insert([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 2], 0);
        table.insert([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 3], 0);
        table.insert([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 4], 0);
        table.insert([0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 5], 100);

        assert_eq!(table[&[2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]], 0);
        assert_eq!(table[&[0_u8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 5]], 100);
        gs
    }

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

    #[allow(unused)]
    fn inc_token_value(&self, token: &[u8; 16]) -> Option<u64> {
        let mut table = self.token_table.borrow_mut();
        match table.get_mut(token) {
            Some(val) => {*val += 1; return Some(*val)},
            None => return None
        }
    }

}


pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {

    #[allow(unused)]
    let mut listener = TcpListener::bind(format!("0.0.0.0:{}", port)).await?;

    // Create global state and wrap it into Rc so that it can be shared between tasks
    let gl_state = Rc::new( GlobalState::init());

    loop {
        let (mut socket, _) = listener.accept().await?;

        // Create a new reference to global state that will be moved to another thread
        let gl_state = gl_state.clone();

        let conn_id = gl_state.inc_conns_total();
        if conn_id > 20 {
            eprintln!("* 20 connections served, exiting.");
            return Ok(());
        }

        // Spawn a new local task for each new connection. Tasks in Tokio are very lightweight. 
        // Under the hood, they require only a single allocation and 64 bytes of memory.
        tokio::task::spawn_local(async move {
            let mut buf = [0; 16];

            // Increment conn. counter
            println!("  * Conn #: {}", &conn_id);

            // Read socket data in a loop
            loop {
                match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => {
                        eprintln!("* conn #{} closed by remote peer", &conn_id);
                        return;
                    },
                    // received data size is not 16 bytes
                    Ok(n) if n != 16 => {
                        eprintln!("* (conn #{}) Err: request size is not 16 bytes ({})", &conn_id, n);
                        return;
                    },
                    Ok(_) => (),
                    Err(e) => {
                        eprintln!("* (conn #{}) failed to read from socket; err = {:?}", &conn_id, e);
                        return;
                    }
                };

                // Increment request counter
                let _req_total = gl_state.inc_requests_total();

                
                // check the token
                eprintln!("* (conn #{}) received token {:?}", &conn_id, &buf);
                match gl_state.inc_token_value(&buf) {
                    Some(val) => { eprintln!("  -> token {:?} was referenced {} times", &buf, val); },
                    None => { eprintln!("  -> token {:?} not found", &buf); }
                }
                eprintln!("  -> conn #{} closed", &conn_id);
                return;        

                // // Write the response
                // if let Err(e) = socket.write_all(ok_resp.as_bytes()).await {
                //     eprintln!("failed to write to socket; err = {:?}", e);
                //     return;
                // }
            }
        });
    }
}
