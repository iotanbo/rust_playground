
// This file declares custom app errors

// Macros exported with '#[macro_export]' are imported from the package root
use apptools::declare_app_errors;
use std::ops::Deref;


declare_app_errors!(ErrList, ErrListLookupTable,
    UserHasNoProfile, "user has no profile",
    RequestLimitExceeded, "user exceeded the number of requests per minute"
);



// User defines custom error type
#[allow(unused)]
pub type AppErr = apptools::err::AppErr<ErrList>;

pub struct AppErrWrapper(AppErr);

#[allow(unused)]
pub type AppLwErr = apptools::err::LwErr<ErrList>;

// #[allow(unused)]
// pub fn get_error_msg(e: &ErrList) -> &'static str {
//     let i = *e as usize;
//     assert!(i < ErrListLookupTable.len());
//     ErrListLookupTable[i]
// }


// #[allow(unused)]
// impl std::fmt::Display for AppErr {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         // get_app_err_desc(&self.kind)
//         write!(f, "{:?} at {}: ", self.kind, self.at, )
//     }
// }


impl Deref for AppErrWrapper {
    type Target = apptools::err::AppErr<ErrList>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}


#[allow(unused)]
impl std::fmt::Display for AppErrWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // get_app_err_desc(&self.kind)
        write!(f, "{:?} at {}: {}", self.0.kind, self.0.at, get_app_err_desc(&self.0.kind))
    }
}