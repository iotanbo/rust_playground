/// Single-threaded token checker server for benchmarking



use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};  // , AsyncWriteExt
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use tokio::time::{self, Duration};

use std::time::{SystemTime};

use log::*;


/// Period of time in millis to poll for shutdown
const SHUTDOWN_POLLING_TIME: u64 = 2000;

/// Maximum number of active connections
const ACTIVE_CONNS_MAX: i64 = 1000;

/// Quit message will init server shutdown if received 
const QUIT_MSG: [u8; 16] = [0xFF_u8; 16];


struct GlobalState {

    /// Id to be assigned to next accepted connection
    next_conn_id: Cell<i64>,

    /// Total of currently active connections
    active_conns_cnt: Cell<i64>,

    /// Active connections value peak ever detected from server start
    pub active_conns_cnt_peak: Cell<i64>,

    /// Total of requests processed from server start
    requests_cnt: Cell<i64>,

    /// True if the server is shutting down;
    /// Each task should periodically poll this value
    /// and gracefully exit if it is `true`.
    #[allow(unused)]
    is_shutting_down: Cell<bool>,

    /// Timestamp of first accepted connection
    pub first_conn_accepted_ts: Cell<SystemTime>,

    /// Timestamp of last accepted connection
    pub last_conn_accepted_ts: Cell<SystemTime>,

    /// Token storage, key is an array of 16 bytes, value is access counter
    token_table: RefCell<HashMap<[u8; 16], u64>>,
}


impl GlobalState {

    /// Initialize global state with default values
    fn init() -> GlobalState {

        let mut gs = GlobalState {
            next_conn_id: Cell::new(0),
            active_conns_cnt: Cell::new(0),
            active_conns_cnt_peak: Cell::new(0),
            requests_cnt: Cell::new(0),
            is_shutting_down: Cell::new(false),
            token_table: RefCell::new(HashMap::new()),

            /// Timestamp of first accepted connection
            first_conn_accepted_ts: Cell::new(SystemTime::now()),

            /// Timestamp of last accepted connection
            last_conn_accepted_ts: Cell::new(SystemTime::now()),

        };

        fn key_from_u128(i: u128) -> [u8; 16] {
            let bytes = i.to_le_bytes();
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

    /// Update (increment) next_conn_id, active_conns_cnt, and active_conns_cnt_peak;
    /// Return next_conn_id
    fn on_new_conn_get_id(&self) -> i64 {
        // Increment next conn id
        let next_conn_id = self.next_conn_id.get();
        self.next_conn_id.set(next_conn_id + 1);

        // Increment active_conns_cnt
        let mut active_conns_cnt = self.active_conns_cnt.get();
        active_conns_cnt += 1;
        self.active_conns_cnt.set(active_conns_cnt);

        // Check active_conns_cnt_peak and update if required
        let mut peak = self.active_conns_cnt_peak.get();
        if peak < active_conns_cnt { peak = active_conns_cnt; self.active_conns_cnt_peak.set(peak);}
        next_conn_id
    }

    fn get_active_conns_cnt(&self) -> i64 {
        self.active_conns_cnt.get()
    }

    fn get_requests_cnt(&self) -> i64 {
        self.requests_cnt.get()
    }

    /// Update (decrement) active_conns_cnt;
    /// Return active_conns_cnt
    #[allow(unused)]
    fn on_conn_closed(&self) -> i64 {
        // Increment active_conns_cnt
        let mut active_conns_cnt = self.active_conns_cnt.get();
        active_conns_cnt -= 1;
        self.active_conns_cnt.set(active_conns_cnt);
        // println!("  -> conn #{} closed. Active conns: {}", &conn_id, active_conns_cnt);
        active_conns_cnt
    }

    /// Increment requests counter
    #[allow(unused)]
    fn inc_requests_cnt(&self) -> i64  {
        let mut requests_cnt = self.requests_cnt.get();
        requests_cnt += 1;
        self.requests_cnt.set(requests_cnt);
        requests_cnt
    }

    /// Allows to check if server is shutting down
    #[allow(unused)]
    fn is_shutting_down(&self) -> bool {
        self.is_shutting_down.get()
    }

    /// Initialize server shutdown
    #[allow(unused)]
    fn init_shutdown(&self) {
        self.is_shutting_down.set(true);
    }

    #[allow(unused)]
    fn inc_token_value(&self, token: &[u8; 16]) -> Option<u64> {
        let mut table = self.token_table.borrow_mut();
        match table.get_mut(token) {
            Some(val) => {*val += 1; return Some(*val)},
            None => return None
        }
    }

    fn store_first_conn_ts(&self) {
        self.first_conn_accepted_ts.set(SystemTime::now());
    }

    fn store_last_conn_ts(&self) {
        self.last_conn_accepted_ts.set(SystemTime::now());
    }

}

/// Accepts new connections in a loop;
/// Returns Ok(()) if accepting can be resumed in the future or
/// Err(Other) if not (e.g. due to shutdown).
async fn accept_new_conns(listener:  &mut tokio::net::TcpListener, gs: &Rc<GlobalState>) ->
                         Result<(), Box<dyn std::error::Error>> {

    trace!("-> accept_new_conn() called...");
    // Accept new connections
    loop {
               
        // Check if server is not shutting down
        if gs.is_shutting_down() { 
            warn!("! accept_new_conn() is going to return Err because shutdown detected.");
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "").into()); 
        }

        // Check if number of active connections limit exceeded
        if gs.get_active_conns_cnt() >= ACTIVE_CONNS_MAX {
            warn!("! Connection limit exceeded. Pausing accepting for 1 second...");
            // Sleep for a while
            time::sleep(Duration::from_millis(10)).await;
            // This is not a fatal error, so return Ok
            return Ok(())
        }

        // eprintln!("  -> new accept loop started");
        let (mut socket, _) = listener.accept().await?;

        // Create a new reference to global state that will be moved to another thread
        let gl_state = gs.clone();

        let conn_id = gl_state.on_new_conn_get_id();

        // If this is the first connection, store its timestamp
        if conn_id == 0 {
            gl_state.store_first_conn_ts();
        }


        // Spawn a new local task for each new connection. Tasks in Tokio are very lightweight. 
        // Under the hood, they require only a single allocation and 64 bytes of memory.
        tokio::task::spawn_local(async move {
            let mut buf = [0; 16];
            debug!("  * Conn #: {}", &conn_id);

            // Read socket data in a loop
            loop {
                match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => {
                        let active_conn_cnt = gl_state.on_conn_closed();
                        trace!("* conn #{} closed by remote peer, active connections: {}", &conn_id, &active_conn_cnt);
                        return;
                    },
                    // received data size is not 16 bytes
                    Ok(n) if n != 16 => {
                        let active_conn_cnt = gl_state.on_conn_closed();
                        trace!("* (conn #{}) Err: request size is not 16 bytes ({}), active connections: {}", 
                                    &conn_id, n, active_conn_cnt);
                        return;
                    },
                    Ok(_) => (),
                    Err(e) => {
                        let active_conn_cnt = gl_state.on_conn_closed();
                        trace!("* (conn #{}) failed to read from socket; err = {:?}, active connections: {}", 
                                    &conn_id, e, active_conn_cnt);
                        return;
                    }
                };

                // Increment request counter
                let _req_total = gl_state.inc_requests_cnt();

                // Check if it is not QUIT_MSG
                if &buf == &QUIT_MSG {
                    let active_conn_cnt = gl_state.on_conn_closed();
                    // Store timestamp when the quit message received
                    gl_state.store_last_conn_ts();
                    trace!("* (conn #{}) QUIT_MSG received, shutting down, active connections: {}", 
                    &conn_id, active_conn_cnt);
                    gl_state.init_shutdown();
                    return;
                }
                
                // check the token
                // (looks like it does not degrade performance much)
                trace!("* (conn #{}) received token {:?}", &conn_id, &buf);
                match gl_state.inc_token_value(&buf) {
                    Some(val) => { trace!("  -> token {:?} was referenced {} times", &buf, val); },
                    None => { trace!("  -> token {:?} not found", &buf); }
                }

                let active_conn_cnt = gl_state.on_conn_closed();
                trace!("  -> conn #{} closed, active connections: {}", &conn_id, active_conn_cnt);
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


pub async fn run_server(port: u16) -> Result<(), Box<dyn std::error::Error>> {

    #[allow(unused)]
    let mut listener = TcpListener::bind(format!("0.0.0.0:{}", &port)).await?;

    info!("== Token Checker Server listening on port {} ==", &port);

    // Create global state and wrap it into Rc so that it can be shared between tasks
    let gl_state = Rc::new( GlobalState::init());

    // Accept new connections in a loop and periodically check for a shutdown request
    loop {

        // Select macro waits for any of the tasks to complete and executes its corresponding handler;
        // Other tasks are dropped.
        // Handlers are never executed simultaneously even in multi-thread environment.
        tokio::select! {
            result = accept_new_conns(&mut listener, &gl_state) => {
                match result {
                    Ok(_) => {}
                    Err(_) => {

                        if gl_state.is_shutting_down() {
                            info!("  * accept_new_conn stopped, exiting ...");
                        break;
                        } else {
                            warn!("  * accept_new_conn error, retrying ...");
                            time::sleep(Duration::from_millis(100)).await;
                        }
                        
                    }

                }
            },
    
            // Accept timeout
            _result = async { trace!("  -> waiting for {} millis started", SHUTDOWN_POLLING_TIME);
                        time::sleep(Duration::from_millis(SHUTDOWN_POLLING_TIME)).await } => {

                if gl_state.is_shutting_down() {
                    info!("  * shutdown detected in run_server() loop, exiting ...");
                    break;
                }

                trace!("  * accept timeout, retrying...");
            },
        }

    }

    
    // Wait until all connections are closed
    for _ in 0..20 {
        if gl_state.get_active_conns_cnt() == 0 { break; }
        time::sleep(Duration::from_millis(100)).await;
    }
    // Assert that all connections are closed
    assert_eq!(gl_state.get_active_conns_cnt(), 0);

    // Check time elapsed between first and last connection
    let first_conn_ts = gl_state.first_conn_accepted_ts.get();
    let last_conn_ts = gl_state.last_conn_accepted_ts.get();

    let mut elapsed_sec = last_conn_ts.duration_since(first_conn_ts).unwrap().as_millis() as f64;
    elapsed_sec /= 1000.0;
    let total_requests_served = gl_state.get_requests_cnt() as f64;
    let requests_per_sec = total_requests_served / elapsed_sec;

    let active_conns_peak = gl_state.active_conns_cnt_peak.get();

    info!("* {} connections served in {} seconds, {} conns/sec, parallel conns peak: {}", 
          gl_state.get_requests_cnt(), &elapsed_sec, &requests_per_sec, active_conns_peak);

    Ok(())

}
