// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/define_error_type.html
// https://learning-rust.github.io/docs/e7.custom_error_types.html


// + custom error types can be used for masking different errors with a single type of error. 
//  This is my experimentation. For production error handling scheme see error_handling.rs


use std::fmt;


#[derive(Debug, Clone)]
struct AuthSrvOffline;
impl fmt::Display for AuthSrvOffline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Auth server is offline")
    }
}

#[derive(Debug, Clone)]
struct BadPassword;
impl fmt::Display for BadPassword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Bad password")
    }
}

// Enumerate all app error types
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum AppError {
    AuthSrvOfflineError,
    BadPassword,
}

// Custom Result type that utilizes AppError (optional)
type AppResult<T> = std::result::Result<T, AppError>;

fn authenticate_dummy(password: &str) -> AppResult<()> {

    if password != "valid_password" { 
        // use explicit return so that there is no need to use 'else' clause
        return Err(AppError::BadPassword)
    }

    let online = true;
    if !online {
        return Err(AppError::AuthSrvOfflineError)
    }
    Ok(())
}




pub fn demo() {

    println!("== custom_error_type demo begin ==");

    fn handle_auth_success() {
        println!("* Dummy authentication SUCCESS");
    }

    fn handle_auth_error(error: AppError) {
        println!("* Dummy authentication failed (intentionally): {:?}", error);
    }

    // Because it is a top decision-making level, we have to process both possible outcomes
    match authenticate_dummy("wrong_password") {
        Ok(_) => handle_auth_success(),
        Err(e) => handle_auth_error(e)
    }

    match authenticate_dummy("valid_password") {
        Ok(_) => handle_auth_success(),
        Err(e) => handle_auth_error(e)
    }

    println!("== custom_error_type demo end ==");
    println!();
}
