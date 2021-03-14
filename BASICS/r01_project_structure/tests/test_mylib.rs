// Integration tests for mylib


use mylib;

#[test]
#[allow(non_snake_case)]
fn test_mylib_mod_A_demo() {
    assert_eq!(mylib::A_mod::demo(), "  * This is A_module demo");
}

