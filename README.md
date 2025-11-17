# rust-tiny-steps
🦀 200 tiny Rust exercises from zero to advanced - one concept at a time. Perfect for beginners who want to learn gradually.
# 🦀 200 Tiny Rust Exercises: Zero to Advanced

A complete, gradual learning path where each exercise teaches **exactly one concept**.

---

## 📋 Complete Exercise List

| # | Exercise | Core Concept | Prerequisites | ✅ |
|---|----------|--------------|--------------|---|
| **1–10: Absolute Basics** ||||
| 1 | Print "Hello, world!" | First program | None | ☐ |
| 2 | Print with formatting: `println!("Hi {}", "Ali");` | String formatting | — | ☐ |
| 3 | Two variables `let a = 5; let b = 10;` and print sum | Immutable variables | — | ☐ |
| 4 | `let mut x = 5;` and change it twice | Mutability | — | ☐ |
| 5 | Explicit type: `let a: u32 = 42;` | Type annotation | — | ☐ |
| 6 | Define and use `const PI: f64 = 3.14;` | Constants | — | ☐ |
| 7 | Get user input with `stdin().read_line` | Input | String, mut | ☐ |
| 8 | Clean input with `.trim()` | String handling | — | ☐ |
| 9 | Parse string to `i32` | parse, Result | — | ☐ |
| 10| Handle parse error: print "Invalid number!" | Basic error handling | — | ☐ |
| **11–20: Basic Functions** ||||
| 11| Function `fn add(a: i32, b: i32) -> i32` | Simple function | — | ☐ |
| 12| Function `greet(name: &str)` that says hello | &str | — | ☐ |
| 13| Function returning larger of two numbers | if statement | — | ☐ |
| 14| Same but with expression: `let max = if a > b { a } else { b };` | if expression | — | ☐ |
| 15| Function `is_even(n: i32) -> bool` | bool return | — | ☐ |
| 16| Function that returns nothing (prints only) | unit type () | — | ☐ |
| 17| Function with `Option<i32>` parameter | Option intro | — | ☐ |
| 18| Recursive factorial (up to 5) | recursion | — | ☐ |
| 19| Function accepting slice `&[i32]` | slice parameter | — | ☐ |
| 20| Inline function with `#[inline]` | attributes | — | ☐ |
| **21–35: Loops** ||||
| 21| `loop` that counts to 10 then breaks | loop + break | — | ☐ |
| 22| `loop` with `continue` (print evens only) | continue | — | ☐ |
| 23| `while` until user types "quit" | while | — | ☐ |
| 24| `while` with compound condition (`&& ||`) | logical operators | — | ☐ |
| 25| `for i in 0..10` print numbers | for + Range | — | ☐ |
| 26| `for i in (1..=100).step_by(2)` odd numbers | step_by | — | ☐ |
| 27| `for i in (0..10).rev()` reverse | rev | — | ☐ |
| 28| Nested loop: 3×3 multiplication table | nested loops | — | ☐ |
| 29| Loop with label: `'outer: loop { … break 'outer; }` | loop labels | — | ☐ |
| 30| `for (index, value) in arr.iter().enumerate()` | enumerate | — | ☐ |
| 31| Infinite loop with internal exit condition | infinite loop pattern | — | ☐ |
| 32| `while let Some(x) = stack.pop()` | while let | Vec | ☐ |
| 33| Loop that repeats 5 times then skips rest | break with counter | — | ☐ |
| 34| `for c in "hello".chars()` | char iteration | — | ☐ |
| 35| `for byte in "Hi".bytes()` | byte iteration | — | ☐ |
| **36–50: Arrays & Vectors** ||||
| 36| Array `[1, 2, 3, 4]` and print all | array | — | ☐ |
| 37| Get array length with `.len()` | array methods | — | ☐ |
| 38| Access array elements by index | indexing | — | ☐ |
| 39| Array of 100 zeros: `[0; 100]` | array initialization | — | ☐ |
| 40| Empty `Vec<i32>` and push 3 numbers | Vec::new | — | ☐ |
| 41| `vec![1, 2, 3]` with macro | vec! macro | — | ☐ |
| 42| `.pop()` last element | Vec::pop | — | ☐ |
| 43| `for item in &vec` print all | borrowing in for | — | ☐ |
| 44| `vec.get(index)` with Option | safe indexing | Option | ☐ |
| 45| `vec.insert(index, value)` | Vec::insert | — | ☐ |
| 46| `vec.remove(index)` | Vec::remove | — | ☐ |
| 47| Function taking slice: `fn sum(nums: &[i32])` | slice parameter | — | ☐ |
| 48| Get slice `&vec[1..4]` | slicing | — | ☐ |
| 49| `vec.sort()` to sort Vec | sorting | — | ☐ |
| 50| `vec.iter().sum()` calculate sum | iterator basics | — | ☐ |
| **51–65: Strings** ||||
| 51| `String::new()` empty String | String::new | — | ☐ |
| 52| `String::from("hello")` | String::from | — | ☐ |
| 53| `.push_str("world")` | push_str | — | ☐ |
| 54| `.push('!')` add character | push | — | ☐ |
| 55| Concatenate with `+` operator | String concatenation | ownership intro | ☐ |
| 56| Build String with `format!` | format! macro | — | ☐ |
| 57| Slice `&s[0..5]` | string slicing | — | ☐ |
| 58| `.chars()` and count characters | char iterator | — | ☐ |
| 59| `.contains("word")` check | String methods | — | ☐ |
| 60| `.replace("old", "new")` | replace | — | ☐ |
| 61| `.split_whitespace()` print words | split | — | ☐ |
| 62| `.to_lowercase()` and `.to_uppercase()` | case conversion | — | ☐ |
| 63| `.trim()` remove whitespace | trim | — | ☐ |
| 64| Convert `&str` ↔ `String` | &str ↔ String | — | ☐ |
| 65| UTF-8: count bytes vs characters | UTF-8 encoding | — | ☐ |
| **66–80: match & enum** ||||
| 66| `match` on number: 1 → one, 2 → two, _ → other | basic match | — | ☐ |
| 67| `match` on string: "quit" → exit | match &str | — | ☐ |
| 68| `match` with range: `1..=5 =>` | match range | — | ☐ |
| 69| `match` with multiple patterns: `1 | 2 =>` | match OR | — | ☐ |
| 70| Simple enum: `Color { Red, Green, Blue }` | enum basics | — | ☐ |
| 71| Match on enum | match enum | — | ☐ |
| 72| Enum with data: `Message::Write(String)` | enum variants with data | — | ☐ |
| 73| Match with destructuring enum | destructuring | — | ☐ |
| 74| `Option<i32>`: Some and None | Option | — | ☐ |
| 75| Match on Option | Option matching | — | ☐ |
| 76| `.unwrap()` on Option (test only) | unwrap | — | ☐ |
| 77| `.expect("message")` | expect | — | ☐ |
| 78| `if let Some(x) = value` | if let | — | ☐ |
| 79| Simple `Result<T, E>` | Result basics | — | ☐ |
| 80| Match on Result: Ok and Err | Result matching | — | ☐ |
| **81–95: Structs** ||||
| 81| Simple struct: `Point { x: i32, y: i32 }` | struct definition | — | ☐ |
| 82| Create instance: `let p = Point { x: 1, y: 2 };` | struct instantiation | — | ☐ |
| 83| Access fields: `p.x` | field access | — | ☐ |
| 84| Struct with `mut` | mutable struct | — | ☐ |
| 85| Tuple struct: `Color(u8, u8, u8)` | tuple struct | — | ☐ |
| 86| Unit struct: `struct Marker;` | unit struct | — | ☐ |
| 87| `impl Point { fn new(x: i32, y: i32) -> Self }` | associated function | — | ☐ |
| 88| Method: `fn distance(&self)` | method with &self | — | ☐ |
| 89| Method with `&mut self` | mutable method | — | ☐ |
| 90| Method with `self` (consumes ownership) | ownership in methods | — | ☐ |
| 91| Struct with lifetime: `struct Wrapper<'a>(&'a str)` | lifetime intro | — | ☐ |
| 92| Generic struct: `struct Pair<T>` | generic struct | — | ☐ |
| 93| `#[derive(Debug)]` and `{:?}` | Debug trait | — | ☐ |
| 94| `#[derive(Clone)]` and `.clone()` | Clone trait | — | ☐ |
| 95| Nested structs (composition) | nested structs | — | ☐ |
| **96–110: Ownership Basics** ||||
| 96| Pass variable to function, see ownership move | move semantics | — | ☐ |
| 97| Same but function returns ownership | return ownership | — | ☐ |
| 98| Use reference instead: `&x` | borrowing | — | ☐ |
| 99| Mutable reference `&mut` | mutable borrow | — | ☐ |
| 100| Try having `&` and `&mut` simultaneously (error!) | borrow rules | — | ☐ |
| 101| Clone instead of move | .clone() | — | ☐ |
| 102| Copy trait: pass number without move | Copy trait | — | ☐ |
| 103| String vs &str in ownership | String ownership | — | ☐ |
| 104| Vec ownership: pass to function | Vec ownership | — | ☐ |
| 105| Dangling reference (compiler error) | lifetime error | — | ☐ |
| 106| Slice as reference | slice borrowing | — | ☐ |
| 107| Multiple immutable borrows | multiple &T | — | ☐ |
| 108| Function that doesn't take ownership | pure borrowing | — | ☐ |
| 109| Pattern with ref: `let ref x = value;` | ref keyword | — | ☐ |
| 110| `std::mem::drop()` for early deallocation | explicit drop | — | ☐ |
| **111–125: Traits Intro** ||||
| 111| Simple trait: `trait Greet { fn greet(&self); }` | trait definition | — | ☐ |
| 112| `impl Greet for MyStruct` | trait implementation | — | ☐ |
| 113| Default implementation in trait | default methods | — | ☐ |
| 114| Trait with associated type | associated types | — | ☐ |
| 115| `#[derive(PartialEq)]` and `==` | PartialEq | — | ☐ |
| 116| Manual impl PartialEq | custom equality | — | ☐ |
| 117| Display trait: `impl Display for Point` | Display trait | — | ☐ |
| 118| From trait: automatic conversion | From/Into | — | ☐ |
| 119| Simple Iterator trait | Iterator basics | — | ☐ |
| 120| Trait bound: `fn print<T: Display>(x: T)` | trait bounds | — | ☐ |
| 121| Multiple bounds: `T: Display + Clone` | multiple bounds | — | ☐ |
| 122| Where clause | where syntax | — | ☐ |
| 123| `impl Trait` in return type | impl Trait | — | ☐ |
| 124| Trait object: `Box<dyn Trait>` | dynamic dispatch | — | ☐ |
| 125| Vec of trait objects | heterogeneous collections | — | ☐ |
| **126–140: Advanced Lifetimes** ||||
| 126| Function with one lifetime: `fn first<'a>(x: &'a str)` | explicit lifetime | — | ☐ |
| 127| `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` | lifetime with multiple params | — | ☐ |
| 128| Lifetime in struct | struct lifetime | — | ☐ |
| 129| Lifetime elision (without writing lifetime) | lifetime elision | — | ☐ |
| 130| `'static` lifetime | static lifetime | — | ☐ |
| 131| Multiple lifetimes: `<'a, 'b>` | multiple lifetimes | — | ☐ |
| 132| Lifetime in impl | impl lifetime | — | ☐ |
| 133| Lifetime bound: `T: 'a` | lifetime bounds | — | ☐ |
| 134| Closure with lifetime | closure lifetime | — | ☐ |
| 135| HRTB: `for<'a>` (just know it exists) | higher-rank trait bounds | advanced | ☐ |
| 136| Lifetime in enum | enum lifetime | — | ☐ |
| 137| Mutable reference with lifetime | &'a mut | — | ☐ |
| 138| Function with independent lifetime | independent lifetime | — | ☐ |
| 139| Debug lifetime errors and fix | debugging lifetimes | — | ☐ |
| 140| `Cow<'a, str>` (Clone on Write) | Cow | advanced | ☐ |
| **141–155: Closures & Iterators** ||||
| 141| Simple closure: `\|x\| x + 1` | closure syntax | — | ☐ |
| 142| Closure with type annotation | closure types | — | ☐ |
| 143| Closure capturing environment | capture | — | ☐ |
| 144| `Fn`, `FnMut`, `FnOnce` traits | closure traits | — | ☐ |
| 145| Closure as function parameter | higher-order functions | — | ☐ |
| 146| Return closure from function | returning closures | — | ☐ |
| 147| `.map()` on iterator | map | — | ☐ |
| 148| `.filter()` | filter | — | ☐ |
| 149| `.collect()` | collect | — | ☐ |
| 150| Chain iterator methods | method chaining | — | ☐ |
| 151| `.fold()` | fold | — | ☐ |
| 152| `.for_each()` | for_each | — | ☐ |
| 153| `.enumerate()` | enumerate | — | ☐ |
| 154| `.zip()` | zip | — | ☐ |
| 155| Custom iterator with `impl Iterator` | custom iterator | — | ☐ |
| **156–170: Advanced Error Handling** ||||
| 156| `?` operator | ? operator | Result | ☐ |
| 157| Function returning `Result<T, E>` | Result return | — | ☐ |
| 158| Custom error type with enum | custom errors | — | ☐ |
| 159| `impl std::error::Error` | Error trait | — | ☐ |
| 160| `thiserror` crate (optional) | error libraries | external | ☐ |
| 161| `anyhow` for simple errors | anyhow | external | ☐ |
| 162| `panic!` vs Result | panic | — | ☐ |
| 163| `unwrap_or()` | unwrap alternatives | — | ☐ |
| 164| `unwrap_or_else()` | lazy unwrap | — | ☐ |
| 165| `and_then()` on Result | Result chaining | — | ☐ |
| 166| `map_err()` | error mapping | — | ☐ |
| 167| Multiple error types with `Box<dyn Error>` | type erasure | — | ☐ |
| 168| `catch_unwind` (just recognize) | panic recovery | advanced | ☐ |
| 169| Propagate error from multiple functions | error propagation | — | ☐ |
| 170| Validate input and return Err | input validation | — | ☐ |
| **171–185: Smart Pointers & Concurrency** ||||
| 171| `Box<T>` for heap allocation | Box | — | ☐ |
| 172| `Rc<T>` for multiple ownership | Rc | — | ☐ |
| 173| `RefCell<T>` for interior mutability | RefCell | — | ☐ |
| 174| `Rc<RefCell<T>>` pattern | Rc+RefCell | — | ☐ |
| 175| `Arc<T>` for thread-safe sharing | Arc | — | ☐ |
| 176| `Mutex<T>` | Mutex | — | ☐ |
| 177| `thread::spawn` first thread | threading | — | ☐ |
| 178| `.join()` wait for thread | join | — | ☐ |
| 179| Share data with Arc+Mutex | concurrent data | — | ☐ |
| 180| `mpsc::channel` for message passing | channels | — | ☐ |
| 181| Send and receive messages | channel communication | — | ☐ |
| 182| Multiple producers | mpsc usage | — | ☐ |
| 183| `RwLock` for read/write | RwLock | — | ☐ |
| 184| `thread::sleep` | sleep | — | ☐ |
| 185| Panic in thread and check it | thread panic | — | ☐ |
| **186–195: Async (Optional with tokio)** ||||
| 186| Install tokio and initial setup | tokio setup | external | ☐ |
| 187| `async fn` first async function | async syntax | — | ☐ |
| 188| `.await` | await | — | ☐ |
| 189| `tokio::spawn` | async tasks | — | ☐ |
| 190| `tokio::time::sleep` | async sleep | — | ☐ |
| 191| `tokio::join!` for concurrent execution | join macro | — | ☐ |
| 192| Async file I/O | async IO | — | ☐ |
| 193| HTTP request with `reqwest` | async HTTP | external | ☐ |
| 194| Error handling in async | async Result | — | ☐ |
| 195| `async` closure | async closures | — | ☐ |
| **196–200: Advanced Topics** ||||
| 196| `macro_rules!` simple macro | declarative macros | — | ☐ |
| 197| `unsafe` block and raw pointer | unsafe | — | ☐ |
| 198| FFI: calling C (just hello world) | FFI basics | advanced | ☐ |
| 199| Workspace with multiple crates | workspace | — | ☐ |
| 200| **Final Project**: CLI tool, game, web scraper, or anything you want! | Everything! | — | ☐ |

