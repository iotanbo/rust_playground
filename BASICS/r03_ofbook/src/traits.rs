// https://doc.rust-lang.org/book/ch10-02-traits.html

// trait is common functionality that can be shared between multiple types
// similar to C# interfaces



pub trait Summary {
    // We can declare just an interface
    //fn summarize(&self) -> String;

    // or it is possible to have a default implementation:
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn trait_basic_demo() {
    let a = NewsArticle {
        headline: "Markovian parallax".to_string(),
        author: "Landa Cowrysian".to_string(),
        content: "Markovian parallax is a super cool stuff.".to_string(),
        location: "Morocco".to_string(),
    };

    let t = Tweet {
        username: "Donny Lloyd".to_string(),
        content: "Tweet about kittens.".to_string(),
        reply: false,
        retweet: false,

    };

    // Normal Summary interface call
    println!(" {}", a.summarize());
    println!(" {}", t.summarize());

    // Define a function that excepts types with Summary implemented
    fn print_summary(item: &impl Summary) {
        println!(" {}", item.summarize());
    }

    print_summary(&a);
    print_summary(&t);

}

// TODO: demos for another syntax and ranges



pub fn demo() {

    println!("== traits demo begin ==");

    trait_basic_demo();

    println!("== traits demo end ==");
    println!();
    
}
