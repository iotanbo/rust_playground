

RUST PROJECT STRUCTURE
=====================

## Readings

https://dev.to/ghost/rust-project-structure-example-step-by-step-3ee

The Cargo Book (official)
https://doc.rust-lang.org/cargo/index.html

Official package layout guidelines
https://doc.rust-lang.org/cargo/guide/project-layout.html

Nice answer about layouts
https://stackoverflow.com/a/57767413/3824328


This is a demo project that contains a library and 2 executables.
Unit and integration tests are demonstrated as well.

## Terminology

https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/cargo-workspaces-for-multi-package-projects.html

* workspace: the highest level of project organization, contains one or more packages
* package: contains one or more crates
* crate: contains one or more modules


### Workspaces

* allows sharing a single set of dependencies


### General rules for rust package

* May contain up to 1 library crate
* May contain any number of executables (binary crates)
* src/main.rs is the crate root of a binary crate with the same name as the package (by default)
* src/lib.rs means that the package contains a library crate with the same name as the package
* for multiple binary crates, create src/bin directory: each file in it will be a separate binary crate
* module names for files are case insensitive: if file is A_mod.rs, module can be referenced as 'mod a_mod;'. !IT IS STILL RECOMMENDED TO USE THE PRECISE CASE.
* module names for directories ARE CASE SENSITIVE.



### Writing complex module with submodules (old structure organization)

In order to create a complex multi-file module:

1) create a directory (e.g. `b_mod`, snake-case preferable) with the file named `mod.rs` that serves as module's root.

2) `mod.rs` is a standard file name should not be changed.

3) create as many `.rs` files as needed. They all can be imported into `mod.rs` with `mod my_mod;` statement;

4) to make any of inner submodules visible to outer world, `mod.rs` declare them public in `mod.rs`:

    ```rust
    pub mod b_one;
    pub mod b_two;
    ```
* You can nest submodules as deeply as you want, just repeat steps 1-4 


### Writing complex module with submodules (new structure organization)

Starting from 2018, rust supports following module organization:
./src/my_mod.rs <- this file has to make public the submodules if required
./src/my_mod
  |
  |-sm1.rs
   - sm2.rs

Note that there is no mod.rs file any longer.


### Creating a single-file executable (binary crate) within same package

1) Create file `./src/bin/single_file_exe.rs`
2) In order to use modules from the library, 
   MAKE THEM VISIBLE from the `lib.rs`: `pub mod A_mod;`

3) Because the library and executable share same package (and Cargo.toml), 
   there is no need to specify it as dependency there.

4) Now, in order to use any of public elements from the library, simply import them:
   `use mylib::A_mod;`

5) This executable can be launched with `cargo run --bin single_file_exe`;



### Creating multi-src executable within same package
1) Specify it's root in Cargo.toml:
https://stackoverflow.com/a/36604610/3824328

```
[[bin]]
name = "multi_file_exe"
path = "src/bin/multi_file_exe/main.rs"
```

2) Add as many files you want, create module structure of any depth.
3) In order to use the library, follow steps from the previous part.

4) Run exe: `cargo run --bin multi_file_exe`.


### Running executables

First, we have to add it explicitly into Cargo.toml:

```
[[bin]]
name = "main_app"
path = "bin/main.rs"
```

Now it's possible to build and run, e.g.: `cargo run --bin main_app`.



### Other Cargo-related topics

#### Import a library from another local project

https://stackoverflow.com/a/45520092/3824328


#### Overriding external crate dependencies with local (patched) versions:
https://doc.rust-lang.org/edition-guide/rust-2018/cargo-and-crates-io/replacing-dependencies-with-patch.html

```
[dependencies]
foo = "1.2.3"

[patch.crates-io]
# if foo depends on bar and bar is buggy, we can use local fixed version:
bar = { path = '/path/to/bar' }
```

## Package layout

This package layout follows recommendations but with one exception:
`bin` directory is moved out from `src` to the root:

```
.
├── Cargo.lock
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── ... other library modules
│
├── bin/
│   ├── named-executable.rs
│   ├── another-executable.rs
│   └── multi-file-executable/
│        ├── main.rs
│        └── some_module.rs
├── benches/
│   ├── large-input.rs
│   └── multi-file-bench/
│       ├── main.rs
│       └── bench_module.rs
├── examples/
│   ├── simple.rs
│   └── multi-file-example/
│       ├── main.rs
│       └── ex_module.rs
└── tests/
    ├── some-integration-tests.rs
    └── multi-file-test/
        ├── main.rs
        └── test_module.rs
```

Because we changed `bin` directory location in order to have cleaner structure, 
it must be reflected in the Cargo.toml file (see its `[[bin]]` sections.)


## Building the library

TODO

## Publishing the package

TODO
