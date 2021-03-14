


fn add_one(num: &mut i32) {
    *num += 1;
}


fn do_n_times(f: fn(&mut i32), val: &mut i32, n: u32) {
    for _ in 0..n {
        f(val);
    }
}

// Returns lambda
fn select_method(method_mum: i32) -> Box<dyn Fn(i32) -> i32> {

    match method_mum {
        1 => Box::new(|x| x + 1),
        2 => Box::new(|x| x + 2),
        _ => Box::new(|x| x),
    }
}


fn basic_demo() {

    let mut val = 10;

    // Call with function pointer
    do_n_times(add_one, &mut val, 5);
    assert_eq!(val, 15);

    // Call with lambda
    do_n_times(|i: &mut i32| {*i += 2}, &mut val, 5);
    assert_eq!(val, 25);

    let selected_method = select_method(2);
    let val = selected_method(val);
    assert_eq!(val, 27);

    let selected_method = select_method(-1);
    let val = selected_method(val);
    assert_eq!(val, 27);

}


pub fn demo() {

    println!("== function pointers demo begin ==");

    basic_demo();


    println!("== function pointers demo end ==");
    println!();
}