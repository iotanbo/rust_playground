

#[allow(unused)]
use const_format::formatcp;
// use std::fmt;


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
        const_format::formatcp!("-> {}:{}", file!(), line!())
    };
}

//#[allow(unused)]
// use this_code_loc;


/// Generic AppErr definition;
/// 'CustomEnumT' must be a user-defined enum based on an integral type (u32 by default).
#[allow(unused)]
// #[derive(Debug)]
pub struct AppErr<CustomEnumT> where CustomEnumT: Send + Sync {  // 72 bytes on 64-bit machines
    pub kind: CustomEnumT,  // 4 bytes (+ 4 bytes align)
    pub at: String,  // 24 bytes
    pub msg: Option<String>,  // 24 bytes
    pub source: Option<Box<dyn std::error::Error>>  // 16 bytes
}


impl<CustomEnumT> AppErr<CustomEnumT> where CustomEnumT: Send + Sync {

    #[allow(unused)]
    pub fn new(kind: CustomEnumT, at: &str, msg: Option<String>) -> AppErr<CustomEnumT> {

        AppErr {
            kind: kind,
            at: at.to_owned(),
            msg: msg,
            source: None,
        }
    }

    #[allow(unused)]
    pub fn from_other(kind: CustomEnumT, at: &str, msg: Option<String>, 
                      source: Box<dyn std::error::Error>) -> AppErr<CustomEnumT> {
        AppErr {
            at: at.to_owned(),
            kind: kind,
            msg: msg,
            source: Some(source),
        }
    }

}

// Light-weight error definition
#[allow(unused)]
#[derive(Debug, Copy, Clone)]
pub struct LwErr<CustomEnumT> {  // 16 bytes
    pub at: &'static &'static str,  // reference to static string: 8 bytes
    pub kind: CustomEnumT,  // 4 bytes + align 4 bytes
}


/// Macro that creates app-specific error enum and corresponding error description lookup table;
#[macro_export]
macro_rules! declare_app_errors {
    ( $EnumName:ident, $EnumNameDesc:ident, $( $EnumElem:ident, $EnumDesc:expr ),+ ) => {

        #[derive(Debug, Copy, Clone)]
        #[allow(unused)]
        #[repr(u32)]
        pub enum $EnumName {  // error enum
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
            FromOther,

            // Then append user-defined errors
            $(
                $EnumElem,
            )*
        }

        #[allow(unused)]
        #[allow(non_upper_case_globals)]
        pub const $EnumNameDesc: &[&str] = &[  // error description table

            // NotFound
            "an entity (possibly a file) could not be found",

            // PermissionDenied
            "operation lacked the necessary privileges to complete",

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
            "entity already exists, often a file",

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
            "other I/O error",

            // UnexpectedEof
            "operation could not be completed because 'end of file' was reached prematurely",

            // ErrorFromOther
            "occurred due to another error",

            // Append user-defined error descriptions (the order will exactly match the enum order)
            $(
                $EnumDesc,
            )*
        ];

        #[allow(unused)]
        pub fn get_app_err_desc(e: &$EnumName) -> &'static str {
            let i = *e as usize;
            assert!(i < $EnumNameDesc.len());
            $EnumNameDesc[i]
        }

        // #[allow(unused)]
        // impl<$EnumName> std::fmt::Display for AppErr where $EnumName: std::fmt::Debug + Send + Sync {
        //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        
        //         write!(f, "{:?} at {}: {}", self.kind, self.at, get_app_err_desc(&self.kind))
        //     }
        // }

        // #[allow(unused)]
        // impl<$EnumName> std::fmt::Display for AppErr where $EnumName: std::fmt::Debug + Send + Sync {
        //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //         // get_app_err_desc(&self.kind)
        //         write!(f, "{:?} at {}: ", self.kind, self.at, )
        //     }
        // }

        // #[allow(unused)]
        // impl std::fmt::Display for AppErr {
        //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //         // get_app_err_desc(&self.kind)
        //         write!(f, "{:?} at {}: ", self.kind, self.at, )
        //     }
        // }

        // #[allow(unused)]
        // impl std::fmt::Display for AppErr {
        //     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        //         // get_app_err_desc(&self.kind)
        //         write!(f, "{:?} at {}: ", self.kind, self.at, )
        //     }
        // }

    };
}


// #[allow(unused)]
// pub fn get_app_err_desc(e: &$EnumName) -> &'static str {
//     let i = *e as usize;
//     assert!(i < $EnumNameDesc.len());
//     $EnumNameDesc[i]
// }


// #[allow(unused)]
// impl<CustomEnumT> std::fmt::Display for AppErr<CustomEnumT> where CustomEnumT: std::fmt::Debug + Send + Sync {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         // get_app_err_desc(&self.kind)
//         write!(f, "{:?} at {}: ", self.kind, self.at, )
//     }
// }

// #[allow(unused)]
// pub fn app_err_desc(e: &CustomEnumT) -> &'static str {
//     let i = *e as usize;
//     assert!(i < ErrListLookupTable.len());
//     ErrListLookupTable[i]
// }

// #[macro_export]
// macro_rules! get_app_err_desc {
//     ( $err_lookup_table: ident, $err:expr ) => {
//         //<$err_enum_type>::new($kind, apptools::this_code_loc!(), $msg)
//         //let i = *e as usize;
//         //assert!(i < $err_lookup_table.len());
//         $err_lookup_table[$err as usize]
//     };
// }



// #[macro_export]
// macro_rules! app_err_new {
//     ( $err_enum_type: ty, $kind: expr, $msg:expr ) => {
//         <$err_enum_type>::new($kind, apptools::this_code_loc!(), $msg)
//     };
// }


#[macro_export]
macro_rules! app_err_new {
    ( $kind: expr, $msg:expr ) => {
        AppErr::new($kind, apptools::this_code_loc!(), $msg)
    };
}

/// Create new AppErr of specified kind with an optional message;
/// The message is of type Option<&str>.
// #[macro_export]
// macro_rules! app_err_new {
//     ( $kind: expr, $msg:expr ) => {
//         match $msg {
//             Some(m) => AppErr::new($kind, apptools::this_code_loc!(), Some(m.to_owned())),
//             None => AppErr::new($kind, apptools::this_code_loc!(), None)
//         }
//     };
// }

/// Create a new AppErr from another error that implements 
/// std::error::Error trait and an optional message.
/// The message is of type Option<String>.
#[macro_export]
macro_rules! app_err_from_other {
    ( $kind: expr, $msg:expr, $source: expr ) => {

        // match $msg {
        //     Some(m) => AppErr::from_other($kind, apptools::this_code_loc!(), 
        //                         Some(m.to_owned()), $source.into()),
        //     None => AppErr::from_other($kind, apptools::this_code_loc!(), 
        //                         None, $source.into())
        // }
        AppErr::from_other($kind, apptools::this_code_loc!(), $msg, $source.into())
    };
}


// pub mod apperr;
// pub mod liberr;
// pub mod lwerr;
