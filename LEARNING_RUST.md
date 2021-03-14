
# LEARNING RUST


## Install Rust on MacOS, Ubuntu

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

* Selected 'default installation' or modify options if required.


```
rustc --version
    rustc 1.49.0
```

How to update: `rustup update`



## VS Code

Rust Extension Pack



## Readings

* A very nice book about async Rust:
  https://rust-lang.github.io/async-book/01_getting_started/01_chapter.html
  - Unfortunately, not yet complete

* Rust cheatsheet (compared to Go):
  https://programming-idioms.org/cheatsheet/Go/Rust

* Getting started (official):
  https://www.rust-lang.org/learn/get-started

* Rust by example (easy demo projects)
  https://doc.rust-lang.org/rust-by-example

* Official learning courses:
  https://www.rust-lang.org/learn

* Rustomonicon: The Dark Arts of Unsafe Rust (official)
  https://doc.rust-lang.org/nomicon/
  
* Rust project structure
  https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee

* Error handling
  https://nick.groenen.me/posts/rust-error-handling/

* Actor model in rust
  https://www.reddit.com/r/rust/comments/7yw358/is_the_actor_model_unfit_for_rust/
  https://youtube.com/watch?v=qr9GTTST_Dk
  Actors or not (2020)
  https://www.youtube.com/watch?v=FM_wuZj83-8
  The Actor Model
  https://www.youtube.com/watch?v=7erJ1DV_Tlo



## Books available

----
Abhishek Chanda - Network Programming with Rust - 2018  
* score: (8 overall)

+ async part present but outdated
+ many real-world examples in detail
+ protocols covered

- almost half of the book is dedicated to Rust basics

----

## Rust best libraries

PRODUCTION
clippy
A tool that analyzes your code and makes kind of peer review reporting non-ideomatic constructs.

CLI

clap 
https://clap.rs/
https://github.com/clap-rs/clap


Serialization

serde
https://serde.rs/

+ Meta-programming (can simplify code)
build_script
syn
quote


HTTP clients

reqwest
https://docs.rs/reqwest/



LOGGING

slog
https://github.com/slog-rs/slog

log - a simpler, standard Rust logger
https://docs.rs/log


DATA PROCESSING

itertools
https://github.com/bluss/rust-itertools

rayon
* give it data and it will know how to split it into independent chunks and work all your CPU cores.


NETWORKING

hyper: a fast HTTP implementation written in and for Rust
https://hyper.rs/
https://github.com/hyperium/hyper

warp - build on top of hyper
https://github.com/seanmonstar/warp

https://github.com/rousan/rust-web-frameworks-benchmark


DATABASES
sqlx

Building fast concurrent DB
https://www.youtube.com/watch?v=s19G6n0UjsM


TEMPLATE RENDERING

Jinja syntax in Rust:
https://github.com/djc/askama
+ Has nice documentation:
  https://djc.github.io/askama/getting_started.html

* Tera: slower but does not need to re-compile for template changes


MEMORY POOLS
https://github.com/sebastiencs/shared-arena



USING C AND GO
https://docs.rs/libloading/0.5.0/libloading/


FLUTTER-RUST FFI
https://github.com/brickpop/flutter-rust-ffi


BENCHMARKING
https://github.com/sebastiencs/shared-arena/blob/master/benches/mempool.rs




## CURATED LIST OF RUST PROJECTS BY CATEGORIES

https://crates.io/crates/awesome-rust



## Cool projects to take into account

A safe, extensible ORM and Query Builder for Rust
https://github.com/diesel-rs/diesel


Actix - super fast WEB and WebSocket server
https://github.com/actix/actix-web








## Basics

### Cargo

Cargo is the Rust build tool and package manager.

`cargo --version`


### Create a new project

`cargo new r01_hello_world`

### Compile and run

`cargo run`


## Networking

### Cool frameworks

https://github.com/tokio-rs/tokio
https://tokio.rs/tokio/tutorial



### Async/await - for beginners

+ Very nice article (?british)
https://blog.logrocket.com/a-practical-guide-to-async-in-rust/



### Memory allocation
Guide to porting C and C++ to Rust
https://locka99.gitbooks.io/a-guide-to-porting-c-to-rust/content/features_of_rust/memory_allocation.html



### Testing

https://doc.rust-lang.org/book/ch11-02-running-tests.html

UNIT TESTS

* run unit tests for the library: `cargo test --lib`
* run unit tests for specified binary crate: `cargo test --bin single_file_exe`
* run all tests: `cargo run`
* run only tests that contain certain name: `cargo test certain_name`
* run only tests annotated with #ignore: `cargo test -- --ignored`

INTEGRATION TESTS

* should be located in the project's `tests` directory
* no need to create module 'test' or annotate with `#[cfg(test)]`
* run with `cargo test`
* run specified file with `cargo test --test integration_test_file`
* each .rs file inside `tests` directory is compiled as separate binary target

* in order to create multi-file integration tests, same principle 
  as for creating multi-file modules applies, e.g.: create `test_mylib_common\mod.rs`
  and then place other .rs files into `test_mylib_common`.

TESTING BINARY CRATES
* With this layout, it is impossible to create integration tests for any binary crate,
  so the option is to put as much code as possible into the library.
  Also, unit tests for binary creates are possible.


## Compiling RUST for iOS
See CROSS_COMPILE projects



## Internationalization

List of available crates:
https://lib.rs/internationalization



+ Compile-time macros (useful as an example):
  + simple
  + effective
  + concise
https://docs.rs/internationalization/0.0.3/internationalization/
https://github.com/terry90/internationalization-rs


+ Fluent (484 stars)
  + Good for translating complex situations FROM TEMPLATES when e.g. gender, age or number matter
  + Something like jinja2
  + There is a UI tool for translation in browser, with playground
    https://www.projectfluent.org/play/

  + Bindings for Javascript, Python, Rust

  - Looks overcomplicated
  - As of 2021 Feb, VERY verbose, like TOO OVER-VERBOSE.
    + Higher level API: fluent-fallback


https://www.projectfluent.org
https://docs.rs/fluent
https://lib.rs/crates/fluent
https://github.com/projectfluent/fluent-rs
https://habr.com/ru/post/448944/

  +++ Higher-level template renderer:
      https://github.com/xampprocky/fluent-templates
      




