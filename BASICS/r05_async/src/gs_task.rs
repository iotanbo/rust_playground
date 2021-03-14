/// Demonstrates how a dedicated task can be used for sharing global state,
/// the concept is similar to the actor model.
/// Shortcomings:
/// - the most complicated implementation
/// - the slowest performance
/// - requires more resources, especially memory allocations
/// Benefits:
/// + is easily scalable
/// + is easy to use on the client side
/// + API shim (possibly) makes it easier to add new functionality


use tokio::sync::{oneshot, mpsc};  // One-shot SPSC queue for returning results from task
use std::sync::{Arc};
use std::io::{Error, ErrorKind};


// Define commands to interact with the global state manager
#[allow(unused)]
#[derive(Debug)]
enum GsCmd {

    // Increment connections total, return new value
    IncConnsTotal {
        //resp: Option<OneShotResponder<u64>>,
        resp: OneShotResponder<u64>,
    },

    // Increment requests total, return new value
    IncRequestsTotal {
        resp: OneShotResponder<u64>,
    },

    // Quit the task
    Quit {},
}


pub type GsResult<T> = Result<T, std::io::Error>;
pub type OneShotResponder<T> = oneshot::Sender<GsResult<T>>;


// Global state data is a private struct, 
// outer world interacts with it only by sending messages
#[allow(unused)]
struct GlobalState {
    requests_total: u64,
    conns_total: u64,
}


/// Global state manager
pub struct GsManager {
    // Whether or not messages can be processed
    pub is_active: bool,  
    // Task handle
    #[allow(unused)]
    task_handler: tokio::task::JoinHandle<()>,
    // Send handle of the input channel
    send_handler: tokio::sync::mpsc::Sender<GsCmd>,
}


impl GsManager {

    /// Create a global state manager task;
    /// Return task handle and tx queue handle as a tuple.
    pub fn create() -> Arc<GsManager> {

        // Create a channel to communicate with global state manager that can hold up to 32 messages
        let (tx, mut rx) = mpsc::channel(32);

        let gs_task = tokio::spawn(async move {

            // Create global state
            let mut gs = GlobalState{conns_total: 0, requests_total: 0};

            // Start receiving messages
            // This cycle will break if either all TX handlers in other threads are destroyed (no senders left)
            // or Quit message received
            while let Some(cmd) = rx.recv().await {
                use GsCmd::*;
        
                // Process each message
                match cmd {

                    IncConnsTotal { resp } => {
                        gs.conns_total += 1;
                        let _ = resp.send(Ok(gs.conns_total));
                    },

                    IncRequestsTotal { resp } => {
                        gs.requests_total += 1;
                        let _ = resp.send(Ok(gs.requests_total));
                    },

                    Quit{} => break
                }
            }
        });

        Arc::new(GsManager{ task_handler: gs_task, send_handler: tx, is_active: true })
    }


    #[allow(unused)]
    async fn process<T>(&self, cmd: GsCmd, resp_rx: oneshot::Receiver<T>) -> GsResult<T> {

        if !self.is_active { return Err(Error::new(ErrorKind::Other, "GSManager inactive")); }
        
        match self.send_handler.send(cmd).await {
            Err(_) => return Err(Error::new(ErrorKind::Other, "failed to send command to GSManager")),
            _ => ()
        }

        match  resp_rx.await {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::new(ErrorKind::Other, "failed to receive response from GSManager")),
        }
    }


    // #[allow(unused)]
    // pub async fn inc_conns_total(&self) -> GsResult<u64> {

    //     if !self.is_active { return Err(Error::new(ErrorKind::Other, "GSManager inactive")); }
    //     let (resp_tx, resp_rx) = oneshot::channel();

    //     let cmd = GsCmd::IncConnsTotal {resp: resp_tx};

    //     match self.send_handler.send(cmd).await {
    //         Err(_) => return Err(Error::new(ErrorKind::Other, "failed to send command to GSManager")),
    //         _ => ()
    //     }

    //     match  resp_rx.await {
    //         Ok(result) => result,
    //         Err(_) => Err(Error::new(ErrorKind::Other, "failed to receive response from GSManager")),
    //     }
    // }

    #[allow(unused)]
    pub async fn inc_conns_total(&self) -> GsResult<u64> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = GsCmd::IncConnsTotal {resp: resp_tx};
        self.process(cmd, resp_rx).await.unwrap()
    }

    #[allow(unused)]
    pub async fn inc_requests_total(&self) -> GsResult<u64> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = GsCmd::IncRequestsTotal {resp: resp_tx};
        self.process(cmd, resp_rx).await.unwrap()
    }

    // TODO: add quit() method
    
    
}
