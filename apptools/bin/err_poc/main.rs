
#[allow(unused)]
// use apptools;
use apptools::{app_err, neg_result, app_err_from_std, app_err_from_other, succ};

mod app_err_decl;
use app_err_decl::{AppErr, AppResult, LwErr, ErrList, app_err_desc};


#[allow(unused)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}


use std::fs::File;


fn basic_usage_demo() {

    println!(" * Size of AppErr: {} bytes", std::mem::size_of::<AppErr>());

    println!(" * Size of LwErr: {} bytes", std::mem::size_of::<LwErr>());

    println!(" * Size of i32: {} bytes", std::mem::size_of::<i32>());
    println!(" * Size of Option<i32>: {} bytes", std::mem::size_of::<Option<i32>>());

    println!(" * Size of String: {} bytes", std::mem::size_of::<String>());
    println!(" * Size of Option<String>: {} bytes", std::mem::size_of::<Option<String>>());

    println!(" * Size of Vec<usize>: {} bytes", std::mem::size_of::<Vec<usize>>());
    // println!(" * Size of Option<String>: {} bytes", std::mem::size_of::<Option<String>>());


    let e = AppErr::new(ErrList::UserHasNoProfile, "line 1", None);
    println!(" * Error (intended) - {:?} at {}: {}.", &(e.kind), &(e.at), app_err_desc(&(e.kind)));

    let e = AppErr::new(ErrList::NotFound, "line 2", Some("file 'dummy.txt'".to_owned()));
    println!(" * Error (intended) - {:?} at {}: {}.", &(e.kind), &(e.at), app_err_desc(&(e.kind)));

    let e = app_err!(ErrList::PermissionDenied, None);  // Some("file 'dummy.txt'")
    println!(" * Error (intended) - {:?} at {}: {}.", &(e.kind), &(e.at), app_err_desc(&(e.kind)));

    let e = app_err!(ErrList::Interrupted, Some(format!("{} times", 3)));
    println!(" * Error (intended) - {}.", &e);

    let filename = "not_exists.txt";
    let reader = File::open(filename);

    match reader {
        Ok(_) => println!(" * file '{}' opened successfully.", filename),
        Err(source) => {
            let e = app_err_from_std!(Some(format!("failed to open file '{}'", filename)), source);
            // let e = app_err_from_other!(ErrList::NotFound, Some(format!("failed to open file '{}'", filename)), source);
            println!(" * Error (intended) - {:?}.", e);  // :?
        },
    }

    let marcus_id = 202415;
    let marcus_requests_per_minute = 10;

    let e = app_err!(ErrList::RequestLimitExceeded, 
                         Some(format!("user: '{}', id: {}, requests per minute: {} ", "Marcus", 
                         marcus_id, marcus_requests_per_minute)));
    println!(" * Error (intended) - {}.", &e); // .brief()
}


fn dummy_func_returning_app_result(produce_error: bool) -> AppResult<&'static str> {

    static DUMMY_MSG: &'static str = "dummy message";
    if !produce_error {
        return Ok(DUMMY_MSG);
    }

    neg_result!(ErrList::DummyError, Some(format!("produced on purpose")))
}


fn call_dummy_func() -> AppResult<()> {

    let result = succ!(dummy_func_returning_app_result(false));
    // Happy path continues: at this point, result is guaranteed to be Ok
    println!("  * dummy function returned: '{}'", result);

    let result = succ!(dummy_func_returning_app_result(true));
    // Happy path still continues
    println!("  * dummy function returned: '{}'", result);
    Ok(())
}


pub fn main() {

    println!("== apptools::err proof of concept app begin ==");

    basic_usage_demo();

    // This is the highest level of this app's error processing logic, so it must process all errors

    let _ = call_dummy_func().map_err(|e| { 
        println!("  * Error (intended) while executing 'call_dummy_func()':  {}", &e)
    });
    

    println!("== apptools::err proof of concept app end ==");
}
