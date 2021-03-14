// Reference: 
// https://github.com/clap-rs/clap/blob/master/examples/07_option_args.rs


use clap::{ArgMatches};
use tokio::time::{Duration};


#[derive(Debug)]
pub struct CliOpts {
    /// Number of parallel connections
    pub parallel_conns: u32,

    /// Number of threads (0 means as number of CPU cores)
    pub threads: u32,

    /// Session duration;
    /// session will end either if this time elapses
    /// or number of requests exceeds the limit.
    pub session_duration: Duration,

    /// Number of requests to do;
    /// session will end either if this number is reached
    /// or session_duration exceeded.
    pub requests_to_do: u32,

}


impl CliOpts {

    /// Create new default options
    pub fn default() -> CliOpts {
        CliOpts {
            parallel_conns: 400,
            threads: 0,
            session_duration: Duration::from_secs(10),
            requests_to_do: 10,
        }
    }

    pub fn parse(&mut self, matches: &ArgMatches) {
        if let Some(c) = matches.value_of("connections") {
            self.parallel_conns = c.parse::<u32>().unwrap();
            // println!("  * parsed value for `connections`: {}", c);
        }

        if let Some(d) = matches.value_of("duration") {
            let dur_sec = d.parse::<u64>().unwrap();
            self.session_duration = Duration::from_secs(dur_sec);
        }

        if let Some(t) = matches.value_of("threads") {
            self.threads = t.parse::<u32>().unwrap();
        }

        if let Some(r) = matches.value_of("requests") {
            self.requests_to_do = r.parse::<u32>().unwrap();
        }

    }

}
