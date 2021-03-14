
# Rust library cross-compilation demo

## Readings
Similar project:
+++ https://github.com/brickpop/flutter-rust-ffi


## iOS

Official guide:
https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
https://robertohuertas.com/2019/10/27/rust-for-android-ios-flutter/


Supported platforms:

https://doc.rust-lang.org/beta/rustc/platform-support.html

aarch64-apple-ios (ARM64 iOS)
x86_64-apple-ios  (64-bit x86 iOS)
aarch64-apple-ios-macabi (Apple Catalyst on ARM64)
armv7-apple-ios (ARMv7 iOS, Cortex-a8)
armv7s-apple-ios 
i386-apple-ios (32-bit x86 iOS)
x86_64-apple-ios-macabi (Apple Catalyst on x86_64)

! Starting from Rust 1.41.0, 32-bit targets are no longer supported:
armv7-apple-ios
armv7s-apple-ios
i386-apple-ios


Official guide:
https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
https://robertohuertas.com/2019/10/27/rust-for-android-ios-flutter/



1) Install toolchains:

```
rustup target add x86_64-apple-ios aarch64-apple-ios 
```

2) Install instruments:

```
cargo install cargo-lipo
```

3) Create some code with C ABI interface, e.g. 
   "hello_from_rust() -> *const c_char  function.

4) Create bridging header `myrustlib.h` in the `include` dir:

``` C++
#ifndef MYRUSTLIB_H
#define MYRUSTLIB_H
#ifdef __cplusplus
extern "C" {
#endif

#include <stdint.h>

const char* rust_greeting(const char* to);
void rust_greeting_free(char *);
const char* hello_from_rust(void);


#ifdef __cplusplus
}
#endif
#endif  // MYRUSTLIB_H
```

5) Create (optional) Swift `module.modulemap`:

```
module myrustlib [system][extern_c] {
 header "myrustlib.h"
 export *
}
```

6) Add to the Cargo.toml
! Note: "cdylib" crate type is no longer supported for iOS!

```
[lib]
name = "myrustlib"
crate-type = ["staticlib"]
```

7) Build the library:
```
cargo lipo --release
```

As a result, we have three libraries (two for each arch and one universal):
  * `myrustlib/target/aarch64-apple-ios/release/libmyrustlib.a`
  * `myrustlib/target/x86_64-apple-ios/release/libmyrustlib.a`
  and
  * `myrustlib/target/universal/release/libmyrustlib.a`


### Using Rust library from a Flutter project

8) Create directory `external_c_libs` inside Flutter Runner project

9) Create symlink to the myrustlib:

```
ln -s /Users/sugurd/LEARNING_RUST21/CROSS_COMPILE/myrustlib/target/universal/release/libmyrustlib.a libmyrustlib.a
```

10) Add it to the XCode project:
   
   a) Runner (target) -> General -> Frameworks, Libraries and Embedded Code
      (add that symlink)

   b) Build Settings -> Search Paths -> Library Search Paths
      `$(PROJECT_DIR)/Runner/external_c_libs`

   c) Build Settings -> Swift Compiler — Search Paths: 
      `$(PROJECT_DIR)/Runner/external_c_libs`


   d) Build settings -> Linking -> Other linker flags: 
        `-lmyrustlib`


11) Add to Flutter project pubspec.yaml:

```
  dependencies:
    ffi: ^0.1.3
```

12) Create Dart bridging file `c_demo.dart`
! There is a tool `ffigen` that automates this task!
https://pub.dev/packages/ffigen

```dart
import 'package:ffi/ffi.dart';
import 'dart:ffi';

// Open our C++ library, iOS only
final DynamicLibrary projectLibHandler = DynamicLibrary.process();

final Pointer<Utf8> Function() hello_from_rust =
    projectLibHandler
    .lookup<NativeFunction<Pointer<Utf8> Function()>>("hello_from_rust")
    .asFunction();

// Auxiliary function that converts Pointer<Utf8> to Dart String
String helloFromRust() {
    return Utf8.fromUtf8(hello_from_rust());
}

```

13) Create a public function in Swift that uses
    all imported functions so that linker bundles your static library into the app


14) Use your functions from any part of Flutter code

