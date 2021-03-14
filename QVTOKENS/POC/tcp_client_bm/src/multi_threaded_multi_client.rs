
// Refs:
// https://stackoverflow.com/questions/61286841/perpetual-tokio-tcp-stream-client
// https://github.com/tokio-rs/tokio/issues/383


use crate::cli_options::CliOpts;
use tokio::runtime::Builder;
use tokio::time::{Duration};  // self, Duration

use tokio::net::{TcpStream };  // TcpListener, 
//use tokio_util::codec::{ Framed, LinesCodec };
//use tokio::stream::StreamExt;
//use std::error::Error;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

use log::*;

// const SRV_IP: (u8, u8, u8, u8) = (127, 0, 0, 1);
const SRV_IP: (u8, u8, u8, u8) = (192, 168, 1, 201);

pub fn run(opts: &CliOpts) {
    match opts.threads {
        // Run with default runtime
        0 => {
            info!("* Running multi-client with default tokio runtime.");
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(run_server(&opts));
        }

        _ => {
            info!("* Running multi-client with custom tokio runtime.");
            let rt = Builder::new_multi_thread()
                .enable_all()
                .worker_threads(opts.threads as usize)
                .thread_stack_size(2 * 1024 * 1024)  // default is 2 Mb
                .build()
                .unwrap();

            rt.block_on(run_server(&opts));
            rt.shutdown_timeout(Duration::from_millis(2000));
        }
    }

    info!("* Multi-client session completed successfully.");

}


async fn run_server(opts: & CliOpts) {
    info!("  -> multi-client started...");
    // time::sleep(Duration::from_millis(1000)).await;
    // time::sleep(opts.session_duration).await;

    let port = 9556_u16;
    // let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port); 
    let addr = SocketAddr::new(IpAddr::V4(
        Ipv4Addr::new(SRV_IP.0, SRV_IP.1, SRV_IP.2, SRV_IP.3)), port); 

    // let conn = match TcpStream::connect(saddr).await {
    //     Ok(val) => val,
    //     Err(e) => {
    //         trace!("  -> couldn't connect to server, {:?}", e);
    //     }
    // };

    // let _conn = TcpStream::connect(saddr).await
    //             .map_err(|e| { trace!("  -> couldn't connect to server, {:?}", e); });

    
    // let mut server = Framed::new(conn, LinesCodec::new_with_max_length(1024));
    // while let Some(Ok(line)) = server.next().await {
    //     println!("{}", line);
    // }

    for conn_id in 0..opts.parallel_conns {


        let parallel_conns = opts.parallel_conns;

        // let mut conn = TcpStream::connect(addr).await
        //             .map_err(|e| { trace!("  -> couldn't connect to server, {:?}", e); })
        //             .unwrap();

        // let mut joined_task_handles = vec!();

        let mut conn = match TcpStream::connect(addr).await {
            Ok(val) => val,
            Err(e) => { warn!("  -> couldn't connect to server, {:?}", e); return; }
        };
                    
        // Create a new atomic reference to global state that will be moved to another thread
        //let gl_state = gl_state.clone();

        // Spawn a new tokio task for each new connection. Tasks in Tokio are very lightweight. 
        // Under the hood, they require only a single allocation and 64 bytes of memory.
        tokio::spawn(async move {  // let h = 

            let mut buf: [u8; 16] = [0; 16];

            if conn_id == parallel_conns - 1 {
                buf = [0xFF; 16];
            }
            trace!("  * Conn #: {}", &conn_id);

            let mut req_count = 0;

            // Read socket data in a loop
            loop {

                // Write the token
                if let Err(e) = conn.write_all(&buf).await {
                    eprintln!("  -> (conn #{}) failed to write to socket; err = {:?}", 
                              conn_id, e);
                    return;
                }

                req_count += 1;

                let n = match conn.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                println!("  * (conn #{}) received request #{}, {} byte(s).", &conn_id, &req_count, n);
                
            }
        });

        // joined_task_handles.push(h);

        // for t in joined_task_handles {
        //     t.await.expect("The task being joined has panicked");
        // }



        // https://github.com/tokio-rs/tokio/issues/383

        // let client = TcpStream::connect(&addr).await
        // .map_err(|e| eprintln!("Connection Error: {:?}",e))
        // .and_then(|socket| {
        //     // tokio::io::write_all(socket, b"hey\n\n")
        //     // .map_err(|e| eprintln!("Write error: {}",e))
        //     // .and_then(|(socket, _x)| {

        //     tokio::io::read_to_end(socket, vec![]).map(|(_, v)| {
        //         let prev = JOKES.fetch_add(1, Ordering::Relaxed);
        //         BYTES.fetch_add(v.len(), Ordering::Relaxed);
        //         println!("Got joke  {}", prev);
        //         })
        //         .map_err(|e| eprintln!("Read Error: {:?}",e))
        // // })
        // });
        // rt.spawn(client);

    }

    info!("  -> multi-client stopped.");
}
