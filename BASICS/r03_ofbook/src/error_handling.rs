// https://nick.groenen.me/posts/rust-error-handling/

// anyhow: https://github.com/dtolnay/anyhow
// thiserr: https://github.com/dtolnay/thiserror

// From impl docs
// https://doc.rust-lang.org/std/convert/trait.From.html


use thiserror::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// Creating constant compile-time strings
use const_format::formatcp;


#[macro_export]
macro_rules! loc {
    () => {
        formatcp!("{}:{}", file!(), line!())
    };
}

#[macro_export]
macro_rules! proxy_loc {
    () => {
        formatcp!("-> {}:{}", file!(), line!())
    };
}




/// WordCountError enumerates all possible errors returned by our library.
#[derive(Error, Debug)]
#[allow(unused)]
pub enum WordCountError {
    /// Represents an empty source. For example, an empty text file being given
    /// as input to `count_words()`.
    #[error("Source contains no data")]
    EmptySource,

    /// Represents a failure to read from input.
    #[error("Read error")]
    ReadError { 
        // source is a special field for the source of this error,
        // see https://github.com/dtolnay/thiserror
        source: std::io::Error 
    },

    // // .0 refers to the first parameter
    // #[error("WordCountError::WrongCase: word has wrong case, first letter must be upper case in word '{}'", .0)]
    // WrongCase(String),

    #[error("(chain: {}) WordCountError::WrongCase: first letter must be upper case in word '{}", .at, .msg)]
    WrongCase {at: String, msg: String},

    /// Represents all other cases of `std::io::Error`.
    #[error(transparent)]
    IOError(#[from] std::io::Error),
}


impl WordCountError {

    fn add_loc(&mut self, proxy_at: &str) {
        match self {
            WordCountError::WrongCase {at, msg: _} => *at += proxy_at,
            _ => return
        }
    }
}


/// Error dispatching macro that works with WordCountError type: 
/// x: Result; 
/// if x is Ok: unwrap it
/// if x is Err: propagate it with added file and line number;
#[macro_export]
macro_rules! ensure {
    ( $x:expr ) => {
        $x.map_err(|mut e| { e.add_loc(&proxy_loc!()); e } )?
    };
}


// pub fn app_err(msg: &str) {

// }

// This must be a macro in order to use loc!() macros
#[macro_export]
macro_rules! app_err {
    ( $msg:expr ) => {
        WordCountError::WrongCase { at: loc!().to_owned(), msg: $msg.to_owned() }
    };
}

#[macro_export]
macro_rules! app_err_result {
    ( $msg:expr ) => {
        Err(WordCountError::WrongCase { at: loc!().to_owned(), msg: $msg.to_owned() })
    };
}


/// Dummy library function
fn count_words<R: Read>(input: &mut R) -> Result<u32, WordCountError> {

    let reader = BufReader::new(input);
    let mut wordcount = 0;

    /// Return WordCountError if s is not uppercase
    fn is_uppercase(s: &str) -> Result<(), WordCountError> {
        // This is how to convert Option<> to Err<>
        // let c = s.chars().nth(0).ok_or(WordCountError::WrongCase {at: loc!().to_owned(), msg: s.to_owned()})?;
        let c = s.chars().nth(0).ok_or(app_err!(s))?;
        if c.is_lowercase() {
            //return Err(WordCountError::WrongCase{at: loc!().to_owned(), msg: s.to_owned()});
            return app_err_result!(s);
        }
        Ok(())
    }

    // TODO: create macro prop_err!() <- propagates error and hides map_err along with closure and ?
    // process_err  dispatch_err  disp_err()

    for line in reader.lines() {
        // In case of error, 'map_err' converts error from 'std::io::Error' to our 'ReadError'
        let line = line.map_err(|source| WordCountError::ReadError { source })?;
        for word in line.split_whitespace() {
            // Ensure word is uppercase, otherwise return WrongCase error
            //is_uppercase(&word).map_err(|mut e| { e.add_loc(&proxy_loc!()); e } )?;
            ensure!(is_uppercase(&word));
            wordcount += 1;
        }
    }

    if wordcount == 0 {
        return Err(WordCountError::EmptySource);
    }

    Ok(wordcount)
}

use anyhow::Context;

pub fn demo() {


    println!(" == error handling demo begin ==");

    // With anyhow, we can provide error context
    let filenames = vec!["hello.txt"];  // , "nofile.txt"

    for f in filenames {
        let mut reader = File::open(&f).context(format!("unable to open '{}'", f)).unwrap();
        let wordcount =
            count_words(&mut reader).context(format!("unable to count words in '{}'", f)).unwrap();
        println!("* Total words in '{}': {}", f, wordcount);
    }

    println!(" == error handling demo end ==");
    println!();
}
