// https://doc.rust-lang.org/reference/macros-by-example.html



// Nested macros
// https://www.reddit.com/r/rust/comments/dtrmmg/help_macro_rules_inside_of_macro_rules/

#[macro_export]
macro_rules! create_another_macro {
    ( ($plaintext:tt) $another_macro_name:ident ) => {

        #[macro_export]
        /// Create app-specific app_err enum and description
        macro_rules! $another_macro_name {
            ( $EnumName:ident, $EnumNameDesc:ident, $plaintext( $EnumElem:ident, $EnumDesc:expr ),+ ) => {

                #[derive(Debug, Copy, Clone)]
                #[allow(unused)]
                #[repr(u32)]
                pub enum $EnumName {
                    $plaintext(
                        $EnumElem,
                    )*
                }

                #[allow(unused)]
                #[allow(non_upper_case_globals)]
                const $EnumNameDesc: &[&str] = &[
                    $plaintext(
                        $EnumDesc,
                    )*
                ];
            };
        }

    };
}

create_another_macro!(($) my_secondary_macro);
my_secondary_macro!(MyTestEnum, MyTestErrListLookupTable,
    StdIoError, "I/O error",
    UserHasNoProfile, "user has no profile",
    RequestLimitExceeded, "user exceeded the number of requests per minute"
);


fn macros_basic_demo() {


}



pub fn demo() {

    println!("== macros demo begin ==");

    macros_basic_demo();

    println!("== macros demo end ==");
    println!();
}
