

use flexi_logger::{Logger, LogTarget, opt_format};  // detailed_format
// detailed_format, ReconfigurationHandle

// LocalSet allows to create async tasks on a single thread 
use tokio::task::LocalSet;

// Import modules of this project
//mod single_thread_http_srv_demo;
// mod single_thread_token_checker;
mod token_checker_srv_for_bench;


async fn dummy_async_app() {
    //println!("== Single Thread Token Checker demo start ==");
    //println!("== Single Thread Token Checker demo end ==");
    
    //single_thread_http_srv_demo::run_server(9555).await.unwrap();
    //single_thread_token_checker::run_server(9556).await.unwrap();
    token_checker_srv_for_bench::run_server(9556).await.unwrap();
    println!("== Token Checker Server shutdown complete ==");

    
}


fn main() {

    // Init logger first
    let logger = Logger::with_env_or_str("info")
       .log_target(LogTarget::StdErr)
       // .buffer_and_flush()  // This is required only for buffered file write
       //.adaptive_format_for_stderr(AdaptiveFormat::Default)
       //.format(detailed_format)
       .format(opt_format)
       .start().unwrap();

    // Create single-threaded runtime, enable_all() enables I/O and time drivers.
    let mut rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    // Local set always creates tasks on a single thread 
    // and allows !Sync types to be shared between tasks
    let local = LocalSet::new();

    local.block_on(&mut rt, dummy_async_app());

    // Shutdown logger task
    logger.shutdown();

}
