
// https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

// * implement Deref and Drop traits
// * Deref: allows an instance of the smart pointer struct to behave like a reference
// * Drop: analog of C++ destructor, called when object goes out of scope

// Common standard library smart pointers:
// * Box<T>: allocates values on the heap
// * Rc<T>: reference counting type that enables multiple ownership
// * Ref<T> and RefMut<T>, accessed through RefCell<T>: 
//   a type that enforces the borrowing rules at runtime instead of compile time
// * Vec<T> and String are also smart pointers


// * Heap memory allocation used when:
//   * object size is unknown at compile time
//   * data size is too large to be allocated on stack
//   * to implement duck typing (dynamic traits)

// * std::cell::RefCell is used when "Internal Mutability" pattern is required.
//   that allows to mutate data of immutable reference.
//   + It is still safe because borrowing rules are enforced at run time.
//   - Like other smart pointers, it introduces small runtime penalty.


// std::mem::drop() is an analog of C++ 'delete' method


// ! It is possible to have memory leaks because of smart pointers,
// e.g. reference cycles, object A has a reference to B and B -> A.

// * Use Weak<T> reference to avoid reference cycles.
//   It references object but does not own it (like in C++).
// https://doc.rust-lang.org/book/ch15-06-reference-cycles.html





#[derive(Debug)]
struct Point {
    x: u32,
    y: u32
}

impl Drop for Point {

    fn drop(&mut self) {
        println!(" * Point ({}, {}) custom destructor called", self.x, self.y);
    }
}


fn basic_box_demo() {
    let mut p = Box::new(Point{x: 10, y: 20});
    // Note that we can use . operator due to ?'automatic referencing and dereferencing'
    // or ?type coercion?
    // https://doc.rust-lang.org/book/ch05-03-method-syntax.html#wheres-the---operator

    p.x += 5;
    p.y += 10;

    println!(" * p = {:?}", p);
}


pub fn demo() {

    println!("== smart_pointer demo begin ==");


    basic_box_demo();

    println!("== smart_pointer demo end ==");
    println!();
}
