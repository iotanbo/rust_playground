// https://doc.rust-lang.org/std/primitive.slice.html
// + https://doc.rust-lang.org/rust-by-example/primitives/array.html


// Slice is a dynamically-sized view into a contiguous sequence, [T]. 
// In other words: it is a view into a block of memory represented as a pointer and a length.
// * similar to array, but its length is not known at compile time. 
// * implemented as a two-word object, the first word is a pointer to the data, 
//   and the second word is the length of the slice.
//
// * can be shared (immutable) or mutable
// * shared  slices signature: &[T]
// * mutable slices signature: &mut [T]


fn type_of<T>(_: T) -> &'static str { std::any::type_name::<T>() }

fn slice_basics() {

    // 1. Array and its slices
    let mut arr = [1, 2, 3];  // Array of 3 ints
    println!("  * Type of arr: {}", type_of(arr));  //[i32; 3]

    // a. shared slice
    let arr_shared_full_slice: &[i32] = &arr[..];  // shared slice, type is &[i32]
    println!("  * Type of arr_full_slice: {}", type_of(arr_shared_full_slice));

    // b. mutable slice (this invalidates 'arr_shared_full_slice')
    let arr_mutable_part_slice: &mut[i32] = &mut arr[..=1];
    // modify second element of the array
    arr_mutable_part_slice[1] = 20;
    println!("  * arr[1]: {}", arr[1]);

    // 2. Slicing a Vec
    let vec = vec![1, 2, 3];
    let vec_slice = &vec[1..];

    // 3. Function that takes slice as a parameter
    fn print_slices(s: &[i32]) {
        println!("  Printing slices: ");
        for i in 0..s.len() {
            println!("    * slice[{}]: {}", i, s[i]);
        }
    }

    print_slices(vec_slice);

    // coercing an array to a slice
    let str_slice: &[&str] = &["one", "two", "three"];
    println!("  String slice: {:?}", str_slice);
}

pub fn demo() {

    println!("== slice demo begin ==");

    slice_basics();

    println!("== slice demo end ==");
    println!();
}
