

#[allow(unused)]
use const_format::formatcp;


/// Compile-time formatted code location
#[macro_export]
macro_rules! this_code_loc {
    () => {
        const_format::formatcp!("{}:{}", file!(), line!())
    };
}


/// Compile-time formatted proxy code location
#[macro_export]
macro_rules! this_code_loc_proxy {
    () => {
        const_format::formatcp!(" -> {}:{}", file!(), line!())
    };
}


/// Macro that creates app-specific error enum and corresponding error description lookup table;
#[macro_export]
macro_rules! declare_app_errors {
    ( $EnumIdent:ident, $EnumDescIdent:ident, $( $EnumElem:ident, $EnumDescStr:expr ),+ ) => {

        #[derive(Debug, Copy, Clone)]
        #[allow(unused)]
        #[repr(u32)]
        pub enum $EnumIdent {  // error enum
            // Include std::io::ErrorKind at the beginning;
            // !Order matters, change it only along with the error description table below!
            NotFound,
            PermissionDenied,
            ConnectionRefused,
            ConnectionReset,
            ConnectionAborted,
            NotConnected,
            AddrInUse,
            AddrNotAvailable,
            BrokenPipe,
            AlreadyExists,
            WouldBlock,
            InvalidInput,
            InvalidData,
            TimedOut,
            WriteZero,
            Interrupted,
            Other,
            UnexpectedEof,

            // Other error kinds
            FromOtherError,

            // User-defined error kinds
            $(
                $EnumElem,
            )*
        }

        #[allow(unused)]
        #[allow(non_upper_case_globals)]
        pub const $EnumDescIdent: &[&str] = &[  // error description table

            // NotFound
            "an entity (possibly a file) could not be found",

            // PermissionDenied
            "operation lacked necessary privileges to complete",

            // ConnectionRefused
            "connection was refused by the remote server",

            // ConnectionReset
            "connection was reset by the remote server",

            // ConnectionAborted
            "connection was aborted (terminated) by the remote server",

            // NotConnected
            "network operation failed because it was not connected yet",

            // AddrInUse
            "socket address could not be bound because the address is already in use elsewhere", 
            
            // AddrNotAvailable
            "nonexistent interface was requested or the requested address was not local",
            
            // BrokenPipe
            "operation failed because a pipe was closed",
            
            // AlreadyExists
            "entity (possibly a file) already exists",

            // WouldBlock
            "operation needs to block to complete, but the blocking operation was requested to not occur", 
            
            // InvalidInput
            "parameter was incorrect",  
            
            // InvalidData
            "data not valid for the operation were encountered",
            
            // TimedOut
            "I/O operation's timeout expired, causing it to be canceled",
            
            // WriteZero
            "operation could not be completed because a call to write returned Ok(0)",
            
            // Interrupted
            "operation was interrupted",

            // Other
            "",

            // UnexpectedEof
            "operation could not be completed because 'end of file' was reached prematurely",

            // FromOther
            "",  // occurred due to another error

            // Append user-defined error descriptions (the order will exactly match the enum order)
            $(
                $EnumDescStr,
            )*
        ];


        #[allow(unused)]
        #[derive(Debug)]
        pub struct AppErr {  // 80 bytes on 64-bit machine

            /// Error kind enum
            pub kind: $EnumIdent,  // 4 bytes

            /// Platform-specific error code
            pub code: Option<i32>,  // 8 bytes
            pub at: String,  // 24 bytes
            pub msg: Option<String>,  // 24 bytes
            pub source: Option<Box<dyn std::error::Error>>  // 16 bytes
        }


        // Light-weight error definition
        #[allow(unused)]
        #[derive(Debug, Copy, Clone)]
        pub struct LwErr {  // 24 bytes on 64-bit machine
            pub kind: $EnumIdent,  // 4 bytes + align 4 bytes
            pub code: Option<i32>,  // 8 bytes
            pub at: &'static &'static str,  // reference to static string: 8 bytes
        }

        // Custom AppResult type
        pub type AppResult<T> = Result<T, AppErr>;


        #[allow(unused)]
        pub fn app_err_desc(e: &$EnumIdent) -> &'static str {
            let i = *e as usize;
            assert!(i < $EnumDescIdent.len());
            $EnumDescIdent[i]
        }
        
        
        #[allow(unused)]
        impl std::fmt::Display for AppErr {
            
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match &self.msg {
                    Some(m) =>  match &self.source {
                                    // user msg and source
                                    Some(src) => write!(f, "{:?} at {}: {}, [{}], source: {:?}", &self.kind, &self.at, 
                                                        app_err_desc(&self.kind), &self.msg.as_ref().unwrap(), &self.source.as_ref().unwrap()),
                                    // user msg, no source
                                    None => write!(f, "{:?} at {}: {}, [{}]", &self.kind, &self.at, 
                                                    app_err_desc(&self.kind), &self.msg.as_ref().unwrap()),
                                }
                    None => match &self.source {
                                // no user msg but source
                                Some(src) => write!(f, "{:?} at {}: {}, source: {:?}", &self.kind, &self.at, 
                                                    app_err_desc(&self.kind), &self.source.as_ref().unwrap()),
                                // no user msg, no source
                                None => write!(f, "{:?} at {}: {}", &self.kind, &self.at, app_err_desc(&self.kind))
                            }
                }
            }
        }


        impl AppErr  {  // implement some convenience methods

            #[allow(unused)]
            pub fn new(kind: $EnumIdent, at: &str, msg: Option<String>) -> AppErr {
                AppErr {
                    kind: kind,
                    code: None,
                    at: at.to_owned(),
                    msg: msg,
                    source: None,
                }
            }
        
            #[allow(unused)]
            pub fn from_other(kind: $EnumIdent, at: &str, msg: Option<String>, 
                              source: Box<dyn std::error::Error>) -> AppErr {

                // Try getting platform-specific code if any
                let mut other_code = None;
                if let Some(e) = source.downcast_ref::<std::io::Error>() {
                    other_code = e.raw_os_error();
                } 

                AppErr {
                    at: at.to_owned(),
                    kind: kind,
                    code: other_code,
                    msg: msg,
                    source: Some(source),
                }
            }

            #[allow(unused)]
            pub fn from_std(at: &str, msg: Option<String>, 
                            source: std::io::Error) -> AppErr {

                use std::io::ErrorKind;

                let translated_kind = match (source.kind()) {
                    ErrorKind::NotFound => ErrList::NotFound,
                    ErrorKind::PermissionDenied => ErrList::PermissionDenied,
                    ErrorKind::ConnectionRefused => ErrList::ConnectionRefused,
                    ErrorKind::ConnectionReset => ErrList::ConnectionReset,
                    ErrorKind::ConnectionAborted => ErrList::ConnectionAborted,
                    ErrorKind::NotConnected => ErrList::NotConnected,
                    ErrorKind::AddrInUse => ErrList::AddrInUse,
                    ErrorKind::AddrNotAvailable => ErrList::AddrNotAvailable,
                    ErrorKind::BrokenPipe => ErrList::BrokenPipe,
                    ErrorKind::AlreadyExists => ErrList::AlreadyExists,
                    ErrorKind::WouldBlock => ErrList::WouldBlock,
                    ErrorKind::InvalidInput => ErrList::InvalidInput,
                    ErrorKind::InvalidData => ErrList::InvalidData,
                    ErrorKind::TimedOut => ErrList::TimedOut,
                    ErrorKind::WriteZero => ErrList::WriteZero,
                    ErrorKind::Interrupted => ErrList::Interrupted,
                    ErrorKind::Other => ErrList::Other,
                    ErrorKind::UnexpectedEof => ErrList::UnexpectedEof,
                    _ => ErrList::Other,
                };             

                // let code = match(source.raw_os_error()) {
                //     Some(val) => val,
                //     None => 0
                // };   

                AppErr {
                    at: at.to_owned(),
                    kind: translated_kind,
                    code: source.raw_os_error(),
                    msg: msg,
                    source: Some(source.into()),
                }
            }

            // brief error description as String
            #[allow(unused)]
            pub fn brief(&self) -> String {
                match &self.msg {
                    Some(m) =>  match &self.source {
                                    // user msg and source
                                    Some(src) => format!("{:?} at {}: [{}], source: {:?}", &self.kind, &self.at, 
                                                        &self.msg.as_ref().unwrap(), &self.source.as_ref().unwrap()),
                                    // user msg and no source
                                    None => format!("{:?} at {}: [{}]", &self.kind, &self.at, 
                                                    &self.msg.as_ref().unwrap()),
                                }
                    None => match &self.source {
                                // no user msg but source
                                Some(src) => format!("{:?} at {}: source: {:?}", &self.kind, &self.at, 
                                                    &self.source.as_ref().unwrap()),
                                // no user msg and no source
                                None => format!("{:?} at {}", &self.kind, &self.at)
                            }
                }
            }

            // TODO: implement as a trait
            // Append proxy code location to the `at` field
            #[allow(unused)]
            #[inline]
            pub fn append_code_loc(&mut self, proxy_at: &str) {
                match self {
                    AppErr {kind: _, code: _, at, msg: _, source: _} => *at += proxy_at,
                    _ => return
                }
            }
        
        }

    };
}

// TODO: create trait ProxyCodeLocation
// Similar to `try!` but can be used only with AppErr error type.
// If code inside this macro fails, the error will be forwarded to the callee,
// otherwise the successful path will continue execution.
#[macro_export]
macro_rules! succ {
    ( $x:expr ) => {
        $x.map_err(|mut e| { e.append_code_loc(&apptools::this_code_loc_proxy!()); e } )?
    };
}

/// Create a new AppErr
#[macro_export]
macro_rules! app_err {
    ( $kind: expr, $msg:expr ) => {
        AppErr::new($kind, apptools::this_code_loc!(), $msg)
    };
}

/// Create a new AppErr and wrap it into AppResult
#[macro_export]
macro_rules! neg_result {
    ( $kind: expr, $msg:expr ) => {
        Err(AppErr::new($kind, apptools::this_code_loc!(), $msg))
    };
}

/// Create a new AppErr from another error that implements 
/// `std::error::Error` trait, add an optional message of type Option<String>.
#[macro_export]
macro_rules! app_err_from_other {
    ( $kind: expr, $msg:expr, $source: expr ) => {
        AppErr::from_other($kind, apptools::this_code_loc!(), $msg, $source.into())
    };
}


/// Create an AppErr from other error and wrap it into AppResult;
#[macro_export]
macro_rules! neg_result_from_err {
    ( $kind: expr, $msg:expr, $source: expr ) => {
        Err(AppErr::from_other($kind, apptools::this_code_loc!(), $msg, $source.into()))
    };
}


// Create a new AppErr from another error that implements 
// `std::error::Error` trait, add an optional message of type Option<String>.
// #[macro_export]
// macro_rules! app_err_from_std {
//     ( $kind: expr, $msg:expr, $source: expr ) => {
//         match ($kind) {

//         }
//         AppErr::from_std($kind, apptools::this_code_loc!(), $msg, $source)
//     };
// }


/// Create a new AppErr from another error that implements 
/// `std::error::Error` trait, add an optional message of type Option<String>.
#[macro_export]
macro_rules! app_err_from_std {
    ( $msg:expr, $source: expr ) => {
            
      AppErr::from_std(apptools::this_code_loc!(), $msg, $source)
        
    };
}
