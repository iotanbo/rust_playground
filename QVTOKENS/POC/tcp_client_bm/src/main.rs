
// clap
use clap::App;

use flexi_logger::{Logger, LogTarget, opt_format};  // detailed_format
// detailed_format, ReconfigurationHandle

// LocalSet allows to create async tasks on a single thread 
// use tokio::task::LocalSet;

// Import modules of this project
//mod single_thread_http_srv_demo;
// mod single_thread_token_checker;
// mod token_checker_srv_for_bench;
mod cli_options;
mod single_threaded_multi_client;
mod multi_threaded_multi_client;
use cli_options::{CliOpts};


fn main() {

    // Init logger
    let logger = Logger::with_env_or_str("info")
       .log_target(LogTarget::StdErr)
       // .buffer_and_flush()  // This is required only for buffered file write
       //.adaptive_format_for_stderr(AdaptiveFormat::Default)
       //.format(detailed_format)
       .format(opt_format)
       .start().unwrap();

    let mut opts = CliOpts::default();

    // Parse CLI options
    let matches = App::new("tcp_client_bm")
        .version("v 1.0")
        .author("Author: iotanbo <yurizappo@gmail.com>")
        .about("A custom TCP client for testing and benchmarking.")
        .arg("-c, --connections=[INT] 'total number of parallel TCP connections to keep open with each thread handling N = connections/threads'")
        .arg("-d, --duration=[INT] 'test session duration in seconds'")
        .arg("-t, --threads=[INT] 'total number of threads to use; 0 (default) - number of threads corresponds to number of CPU cores'")
        .arg("-r, --requests=[INT] 'total number of requests to do'")
        .get_matches();

    opts.parse(&matches);

    println!("* Client will run with the following options: {:?}", opts);

    // Start different flavours of client depending on options specified
    match opts.threads {
        1 => {
            single_threaded_multi_client::run(&opts);
        }

        _ => {
            multi_threaded_multi_client::run(&opts);
        }
    }


    // Shutdown logger task
    logger.shutdown();
}
