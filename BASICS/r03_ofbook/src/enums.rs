// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

// https://doc.rust-lang.org/reference/items/enumerations.html


// * Rust enums are ? multi-paradigm and may be closer to Python Unions or C enums
//   (this is how I understand, maybe wrong)
// + enums can contain different types
// + by implementation it is something like union of structs


// -----------------------------------------------------------------
// Demo 1. Simple numeric enums of type u32 close to C enums

#[allow(dead_code)]  // Some enums values are unused but reserved for future
#[repr(u32)]  // If this line ommited, the default value type would be 'isize'
enum Digits {
    Zero, One, Two, Three,
    Four, Five, Six, Seven, Eight, Nine,
    Million = 1000_000,
}

fn basic_enum_demo() {
    // Simple enums can be cast to  integers with 'as' operator
    let d1 = Digits::Zero;
    assert_eq!(d1 as i32, 0);  // Note that we cast here from 'u32' to 'i32'
    assert_eq!(Digits::Million as i32, 1_000_000);

    // ! Danger! Compiler won't warn that truncation took place!
    let v2: u8 = Digits::Million as u8;
    assert_eq!(v2, 64);

}

// -----------------------------------------------------------------
// Demo 2. Complex enums close to Python Unions

#[allow(dead_code)]
enum ActionMsg {  // Enumerate all messages that our API supports along with the signatures
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl ActionMsg {  // We can implement methods for enums as for structs
    fn send(&self) {
        print!("  * ActionMsg 'send' called: ");
        use ActionMsg::*;

        // Check msg type and generate actual data to be sent
        // All arms must return same value type
        let cmd = match self {
            Quit => "Quit".to_string(),
            Move {x, y} => format!("Move({}, {})", x, y),
            Write(msg) => format!("Write({})", msg),
            ChangeColor(r, g, b) => format!("ChangeColor(r={}, g={}, b={})", r, g, b),
        };

        // Simulate data sending
        println!("  * msg sent: {}", cmd);
    }
}

fn complex_enum_demo() {

    let mut m = ActionMsg::Write(String::from("hello"));
    m.send();
    m = ActionMsg::ChangeColor(1, 2, 255);
    m.send();
    m = ActionMsg::Move {x:10, y:20};
    m.send();
    m = ActionMsg::Quit;
    m.send();
}

// -----------------------------------------------------------------

pub fn demo() {

    println!("== enums demo begin ==");

    basic_enum_demo();
    complex_enum_demo();

    println!("== enums demo end ==");
    println!();
}

