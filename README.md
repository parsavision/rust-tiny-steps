
# 🦀 Rust Tiny Steps v2: 300 Exercises for Complete Beginners

A complete, gradual learning path designed for **absolute beginners**. Each exercise teaches exactly **ONE** concept with clear explanations.

## How to Use This Guide

* ✅ = **Complete**
* 📝 = **In Progress**
* ⭐ = **Important Concept (Must Practice)**
* 🎯 = **Practice This Multiple Times**

---

## 🌟 PHASE 1: Your First Steps (Exercises 1-20)
### Week 1: Hello World & Basic Output

| Status | Exercise | Goal | Concept |
| :---: | :---: | :--- | :--- |
| ⭐✅ | **Exercise 1** | Write your very first Rust program | `println!` macro, your first program |
| ✅| **Exercise 2** | Print your own name | Changing what you print |
| ✅| **Exercise 3** | Print multiple lines | Multiple `println!` statements |
| ⭐ ✅| **Exercise 4** | Print with a placeholder | String formatting with `{}` |
|✅ | **Exercise 5** | Print two things | Multiple placeholders `{}` and `{}` |

### Week 2: Variables & Numbers

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐✅ | **Exercise 6** | Create your first variable | **Immutable variables** with `let` |
| ✅| **Exercise 7** | Use two variables | Multiple variables |
| ⭐✅ | **Exercise 8** | Do simple math | Addition with `+` |
| ✅| **Exercise 9** | Save the result | Using math results in new variables |
| ✅| **Exercise 10** | Subtract numbers | Subtraction with `-` |
| ✅| **Exercise 11** | Multiply numbers | Multiplication with `*` |
| ✅| **Exercise 12** | Divide numbers | Division with `/` (Integer division) |
| ⭐✅ | **Exercise 13** | Understand you can't change variables | Variables are **immutable** by default (Must error) |
| ⭐✅ | **Exercise 14** | Learn to make changeable variables | **Mutable variables** with `mut` |
| 🎯 ✅| **Exercise 15** | Practice changing variables | Mutability practice |
| ✅| **Exercise 16** | Understand variable types | Type annotations (`: i32`) |
| ✅| **Exercise 17** | Try different number types | Unsigned integers (`u32`) |
| ⭐✅ | **Exercise 18** | Constants vs variables | **Constants** using `const` (UPPERCASE) |
| ✅| **Exercise 19** | Use multiple operations | Order of operations with parentheses |
| 📝 ✅| **Exercise 20** | Remainder operation | Modulo operator (`%`) |

---

## 🎯 PHASE 2: Interacting with Users (Exercises 21-35)
### Week 3: Getting Input

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 21** | Get user input for the first time | `stdin().read_line()`, using `std::io` |
| | **Exercise 22** | Create an empty String | Creating empty `String`s for input |
| | **Exercise 23** | Print what the user typed | Echo program |
| ⭐ | **Exercise 24** | Clean up user input | String trimming with `.trim()` |
| | **Exercise 25** | Get a number from the user | Input starts as text (`String`) |
| ⭐ | **Exercise 26** | Convert text to number | **Type conversion** with `.parse()` |
| | **Exercise 27** | Handle parsing errors safely | Basic error handling with `.expect()` |
| 📝 | **Exercise 28** | Simple calculator input | Combining input, parsing, and math |
| | **Exercise 29** | Input validation message | Handling `Result` with `match` |
| | **Exercise 30** | Build a greeting program | Combining input and formatting |

### Week 4: Making Decisions

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 31** | Your first `if` statement | Basic `if` statement |
| | **Exercise 32** | Add an `else` | `if-else` structure |
| | **Exercise 33** | Check if equal | Equality with `==` |
| | **Exercise 34** | Multiple conditions | `else if` structure |
| ⭐ | **Exercise 35** | `if` as an expression | `If` expressions return values |

---

## 🔨 PHASE 3: Functions & Reusable Code (Exercises 36-60)
### Week 5: Basic Functions

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 36** | Create your first function | Function definition with `fn` |
| | **Exercise 37** | Call your function | Function calls |
| | **Exercise 38** | Function with a parameter | Function parameters |
| | **Exercise 39** | Multiple parameters | Multiple parameters in a function |
| ⭐ | **Exercise 40** | Return a value | Return values with `->` (Last expression) |
| | **Exercise 41** | Use `return` keyword | Explicit `return` keyword |
| | **Exercise 42** | Return from middle of function | Early returns |
| | **Exercise 43** | Function returning `bool` | Boolean returns |
| | **Exercise 44** | Function returning bigger number | Comparison in functions |
| | **Exercise 45** | Function without return | Unit type `()` is implicit |
| 📝 | **Exercise 46** | Build a calculator | Multiple helper functions |
| | **Exercise 47** | Function calling function | Function composition |
| | **Exercise 48** | Absolute value function | Conditional returns |
| | **Exercise 49** | Square a number | Simple math functions |
| ⭐ | **Exercise 50** | Simple recursion | Function calling itself |

### Week 6: More Functions

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| | **Exercise 51** | Factorial with recursion | Practical recursion |
| | **Exercise 52** | Check if positive | Simple boolean logic |
| | **Exercise 53** | Check if negative | Comparison operators |
| | **Exercise 54** | Check range | Multiple conditions with `&&` |
| ⭐ | **Exercise 55** | String parameter | String slices as parameters (`&str`) |
| | **Exercise 56** | Convert temperature (C to F) | Working with floats (`f64`) |
| | **Exercise 57** | Convert temperature (F to C) | Inverse operations |
| | **Exercise 58** | Calculate circle area | Using PI (3.14159...) |
| | **Exercise 59** | Calculate rectangle area | Multiple parameters with floats |
| 📝 | **Exercise 60** | Grade calculator | Multiple `if` conditions |

---

## 🔄 PHASE 4: Loops & Repetition (Exercises 61-90)
### Week 7: Loop Basics

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 61** | Your first loop | Infinite loop with `loop`, using counter and `break` |
| | **Exercise 62** | Break out of loop | `break` keyword |
| | **Exercise 63** | Continue in loop | `continue` keyword |
| ⭐ | **Exercise 64** | `While` loop | `while` loop with condition |
| | **Exercise 65** | `While` with user input | Sentinel-controlled loop |
| ⭐ | **Exercise 66** | `For` loop with range | `for` loop with range (`0..10` excludes 10) |
| | **Exercise 67** | Inclusive range | Inclusive range with `..=` |
| | **Exercise 68** | Countdown loop | Reverse iteration with `.rev()` |
| | **Exercise 69** | Skip numbers | Step iteration with `.step_by(2)` |
| | **Exercise 70** | Sum with loop | Accumulator pattern |
| 📝 | **Exercise 71** | Multiplication table | Loop with multiplication |
| | **Exercise 72** | Factorial with loop | Loop-based algorithm |
| | **Exercise 73** | Count down to blast off | Loop with final action |
| | **Exercise 74** | Find first number divisible by 7 | Search with `break` |
| ⭐ | **Exercise 75** | Nested loops | Loop inside loop |

### Week 8: Advanced Loops

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| | **Exercise 76** | Multiplication table grid | Nested loop with math |
| | **Exercise 77** | Pattern printing | Inner loop depends on outer |
| ⭐ | **Exercise 78** | Loop labels | Named loops with labels |
| | **Exercise 79** | Sum of squares | Combining operations in loop |
| | **Exercise 80** | Find average | Mathematical operations with loops |
| | **Exercise 81** | `While` with multiple conditions | Compound conditions with `&&` |
| | **Exercise 82** | Or condition in `while` | Or conditions with `||` |
| | **Exercise 83** | Do-while pattern | Post-test loop pattern |
| 📝 | **Exercise 84** | FizzBuzz game | Multiple conditions in loop |
| | **Exercise 85** | Guess the number | Loop with input and comparison |
| | **Exercise 86** | Count vowels | Loop with string checking |
| | **Exercise 87** | Fibonacci sequence | Sequence generation |
| | **Exercise 88** | Reverse a number | Number manipulation |
| | **Exercise 89** | Check if prime | Mathematical algorithm |
| ⭐ | **Exercise 90** | Menu system | Practical loop pattern |

### Week 9: Arrays

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 91** | Create first array | **Fixed-size arrays** |
| | **Exercise 92** | Access array element | Zero-based indexing with `arr[2]` |
| | **Exercise 93** | Array length | `.len()` method |
| | **Exercise 94** | Loop through array | Iterating arrays (`for item in arr.iter()`) |
| | **Exercise 95** | Initialize array with same value | Array initialization syntax `[0; 10]` |
| | **Exercise 96** | Sum array elements | Accumulator with arrays |
| | **Exercise 97** | Find maximum | Comparison in loop |
| | **Exercise 98** | Find minimum | Finding minimum value |
| | **Exercise 99** | Array with explicit type | Type annotations for arrays |
| | **Exercise 100** | 2D array | Multidimensional arrays |

### Week 10: Vectors

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 101** | Create empty vector | **Dynamic vectors** vs fixed arrays |
| | **Exercise 102** | Push to vector | `.push()` method |
| ⭐ | **Exercise 103** | Vec macro | Quick vector creation (`vec![]`) |
| | **Exercise 104** | Pop from vector | Stack operations on vector with `.pop()` |
| | **Exercise 105** | Vector length | `.len()` on vectors |
| | **Exercise 106** | Check if empty | Boolean methods with `.is_empty()` |
| | **Exercise 107** | Access vector element | Indexing vectors (Warning: Can panic) |
| ⭐ | **Exercise 108** | Safe vector access | Safe indexing with `.get(index)` (Returns `Option`) |
| | **Exercise 109** | Insert at position | Inserting in middle with `.insert()` |
| | **Exercise 110** | Remove from position | Removing specific element with `.remove()` |
| | **Exercise 111** | Clear vector | Emptying collections with `.clear()` |
| | **Exercise 112** | Loop through vector | Iterating vectors |
| | **Exercise 113** | Iterate with index | Enumeration with `.iter().enumerate()` |
| | **Exercise 114** | Modify while iterating | Mutable iteration with `.iter_mut()` |
| 📝 | **Exercise 115** | Filter vector | Conditional collection building |
| | **Exercise 116** | Vector of strings | Vectors hold any type |
| | **Exercise 117** | Sort vector | In-place sorting with `.sort()` |
| | **Exercise 118** | Reverse vector | Reversing collections with `.reverse()` |
| | **Exercise 119** | Check if contains | Searching vectors with `.contains()` |
| ⭐ | **Exercise 120** | Vector capacity | Memory allocation (`.capacity()` vs `.len()`) |

### Week 11: Strings

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 121** | Create empty String | `String` vs **&str** |
| | **Exercise 122** | String from literal | Converting `&str` to `String` |
| | **Exercise 123** | Push to string | String concatenation method 1: `.push_str()` |
| | **Exercise 124** | Push single char | Adding single characters with `.push('!')` |
| ⭐ | **Exercise 125** | Concatenate with `+` | `+` operator (takes **ownership**!) |
| | **Exercise 126** | Format macro | Non-consuming concatenation with `format!` |
| | **Exercise 127** | String length | `.len()` gives **bytes**, not characters! |
| | **Exercise 128** | Count characters | Difference between bytes and chars (`.chars().count()`) |
| | **Exercise 129** | Iterate chars | `.chars()` iterator |
| | **Exercise 130** | String slicing | String slices (`&s[0..5]`) |

---

## 🔀 PHASE 6: Control Flow Advanced (Exercises 131-165)
### Week 12: Match & Patterns

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 131** | Basic `match` | `match` expression |
| | **Exercise 132** | Match returns value | Match as expression |
| | **Exercise 133** | Match on multiple values | OR patterns with `|` |
| | **Exercise 134** | Match ranges | Range patterns |
| | **Exercise 135** | Match guard | Conditional patterns |
| ⭐ | **Exercise 136** | Match on `bool` | Boolean matching |
| | **Exercise 137** | Match on tuple | Tuple patterns |
| | **Exercise 138** | Destructure in match | Pattern binding |
| | **Exercise 139** | Match on char | Character matching |
| | **Exercise 140** | Match on string slice | String matching |

### Week 13: Enums

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 141** | Define first `enum` | Simple enums |
| | **Exercise 142** | Use `enum` | Enum instantiation |
| | **Exercise 143** | Match on `enum` | Enum matching |
| | **Exercise 144** | Enum with data | Enum variants with **associated data** |
| | **Exercise 145** | Match and extract data | Destructuring enum variants |
| ⭐ | **Exercise 146** | **Option** type | `Option` enum for nullable values (`Some`/`None`) |
| | **Exercise 147** | Match on `Option` | `Option` handling |
| | **Exercise 148** | Unwrap option | Unsafe unwrapping with `.unwrap()` |
| | **Exercise 149** | Expect with message | Better error messages with `.expect()` |
| ⭐ | **Exercise 150** | `if let` with `Option` | Concise `Option` handling |
| | **Exercise 151** | `unwrap_or` | Safe unwrapping with default value |
| | **Exercise 152** | **Result** type | `Result` for operations that can fail (`Ok`/`Err`) |
| | **Exercise 153** | Match on `Result` | `Result` handling |
| | **Exercise 154** | Return `Result` from function | Error-returning functions |
| ⭐ | **Exercise 155** | `?` operator | **Error propagation** |
| | **Exercise 156** | Multiple enums | Enum design |
| | **Exercise 157** | Enum methods | Methods on enums |
| | **Exercise 158** | Nested enums | Complex types |
| | **Exercise 159** | Match with variables | Pattern binding |
| | **Exercise 160** | Ignore with `_` | Wildcard patterns |
| | **Exercise 161** | `while let` pattern | Loop with pattern matching |
| | **Exercise 162** | Combining `Option` and `Result` | Nested types |
| | **Exercise 163** | `map` on `Option` | Functional `Option` handling |
| | **Exercise 164** | `and_then` on `Option` | `Option` combinators |
| 📝 | **Exercise 165** | Build menu with enums | Practical enum usage |

### Week 14: Struct Basics

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 166** | Define first struct | **Struct definition** |
| | **Exercise 167** | Create struct instance | Struct instantiation |
| | **Exercise 168** | Access struct fields | Field access with `.` |
| | **Exercise 169** | Mutable struct | Changing fields with `mut` |
| | **Exercise 170** | Struct with multiple fields | Multi-field structs |
| | **Exercise 171** | Tuple struct | Tuple structs (unnamed fields) |
| | **Exercise 172** | Unit struct | Unit-like structs (no data) |
| ⭐ | **Exercise 173** | Struct method - `new` | Associated function (**constructor**) |
| | **Exercise 174** | Method with `&self` | **Borrowing** self in methods |
| | **Exercise 175** | Method with `&mut self` | Mutable self |
| | **Exercise 176** | Method that consumes self | Moving self (takes ownership) |
| | **Exercise 177** | Method returning value | Methods with return values |
| | **Exercise 178** | Multiple methods | `impl` blocks |
| | **Exercise 179** | Struct with reference field | Lifetime intro (Skip for now) |
| ⭐ | **Exercise 180** | **Debug** trait | Automatic trait derivation (`#[derive(Debug)]`) |
| | **Exercise 181** | Pretty print | Pretty Debug formatting (`{:#?}`) |
| | **Exercise 182** | **Clone** trait | Duplicating structs |
| | **Exercise 183** | Nested structs | Composition |
| | **Exercise 184** | Struct with vector | Structs with collections |
| 📝 | **Exercise 185** | Rectangle struct | Practical struct example |

### Week 15: Advanced Structs

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| | **Exercise 186** | `PartialEq` derivation | Equality comparison |
| | **Exercise 187** | Manual `PartialEq` | Custom equality logic |
| | **Exercise 188** | Default values | Struct update syntax (`..Default::default()`) |
| | **Exercise 189** | Struct with `Option` field | Optional fields |
| | **Exercise 190** | Builder pattern start | Method chaining |
| | **Exercise 191** | Struct validation | Validation in constructors |
| | **Exercise 192** | Private fields | **Encapsulation** |
| | **Exercise 193** | Getter methods | Accessors |
| | **Exercise 194** | Setter methods | Mutators with validation |
| | **Exercise 195** | Associated constants | Constants in `impl` blocks |
| | **Exercise 196** | Multiple `impl` blocks | Multiple implementations |
| | **Exercise 197** | Struct with generic - preview | Generic structs intro |
| 📝 | **Exercise 198** | Bank account struct | Real-world struct modeling |
| | **Exercise 199** | Student grade tracker | Structs with calculations |
| ⭐ | **Exercise 200** | Mini project - Contact book | Combining concepts |

---

## 🔗 PHASE 8: Ownership & Borrowing (Exercises 201-230)
### Week 16: Ownership Basics

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 201** | Observe ownership **move** | Move semantics |
| | **Exercise 202** | Return ownership | Giving ownership back |
| ⭐ | **Exercise 203** | **Borrow** with reference | Immutable borrowing (`&string`) |
| | **Exercise 204** | Multiple borrows | Multiple immutable borrows OK |
| ⭐ | **Exercise 205** | **Mutable borrow** | Mutable borrowing (`&mut x`) |
| | **Exercise 206** | Cannot have `&` and `&mut` | The Borrowing Rule (One `&mut` OR many `&`) |
| | **Exercise 207** | **Copy** trait | Types that implement `Copy` don't move |
| | **Exercise 209** | Ownership with `Vec` | Collections and ownership |
| | **Exercise 210** | Borrow `Vec` | Borrowing collections (`&Vec<i32>`) |
| | **Exercise 211** | Modify through `&mut` | Mutable borrow allows changes |
| | **Exercise 212** | Slice as borrow | Slices are borrowed views |
| | **Exercise 213** | `String` vs `&str` ownership | Owned vs borrowed strings |
| | **Exercise 214** | Scope and dropping | Automatic cleanup |
| 📝 | **Exercise 215** | Return reference - error | Dangling references not allowed |

### Week 17: Borrowing Rules & Lifetimes

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| | **Exercise 216** | Borrow in different scopes | Scope-based borrowing |
| | **Exercise 217** | Reborrowing | References of references |
| | **Exercise 218** | Struct field borrowing | Partial borrowing |
| | **Exercise 219** | Pass by reference pattern | Idiomatic Rust |
| | **Exercise 220** | Return borrowed value | Lifetime elision (simple case) |
| | **Exercise 221** | `Vec` iteration borrowing | Borrowed vs consuming iteration |
| | **Exercise 222** | Mutable iteration | Mutable iteration for modifying elements |
| | **Exercise 223** | Iterator `collect` | Ownership in iterators |
| | **Exercise 224** | Reference in struct - error | Need **lifetimes** for references in structs |
| ⭐ | **Exercise 225** | Simple **lifetime annotation** | Explicit lifetimes (`<'a>`) |
| | **Exercise 226** | Function lifetime | Lifetime parameters in functions |
| | **Exercise 227** | Why lifetimes matter | Lifetime purpose |
| | **Exercise 228** | Independent lifetimes | Multiple lifetime parameters |
| | **Exercise 229** | **'static** lifetime | Static lifetime |
| 📝 | **Exercise 230** | Ownership quiz | Reinforcement |

---

## 🎨 PHASE 9: Traits & Generics (Exercises 231-265)
### Week 18: Traits

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 231** | Define first **Trait** | Trait definition |
| | **Exercise 232** | Implement trait | Trait implementation (`impl Speak for Dog`) |
| | **Exercise 233** | Multiple types, same trait | **Polymorphism** |
| | **Exercise 234** | Default implementation | Default methods in a trait |
| | **Exercise 235** | Override default | Overriding defaults |
| ⭐ | **Exercise 236** | **Trait bound** in function | Generic with trait bound (`T: Display`) |
| | **Exercise 237** | Multiple trait bounds | Compound trait bounds |
| | **Exercise 238** | `where` clause | `Where` clause syntax |
| | **Exercise 239** | `impl Trait` syntax | Abstract return types |
| | **Exercise 240** | **Display** trait | Custom formatting |
| | **Exercise 241** | **From** trait | Type conversion trait |
| | **Exercise 242** | **Into** trait | Automatic `Into` from `From` |
| | **Exercise 243** | **TryFrom** trait | Fallible conversion |
| | **Exercise 244** | **AsRef** trait | Flexible references |
| 📝 | **Exercise 245** | Custom trait for shapes | Practical traits |

### Week 19: Generics

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 246** | **Generic function** | Generic functions |
| ⭐ | **Exercise 247** | **Generic struct** | Generic structs |
| | **Exercise 248** | Multiple type parameters | Multiple generics |
| | **Exercise 249** | Generic enum | Generic enums (like `Option`/`Result`) |
| | **Exercise 250** | Generic method | Methods on generic types |
| | **Exercise 251** | Mixing concrete and generic | Specialized implementations |
| | **Exercise 252** | **Const generics** | Compile-time constants in generics |
| | **Exercise 253** | Turbofish operator | Explicit type specification (`::<>`) |
| | **Exercise 254** | Generic with lifetime | Combining lifetimes and generics |
| | **Exercise 255** | Default type parameter | Default generic types |
| | **Exercise 256** | `PhantomData` (advanced preview) | Zero-size type markers |
| | **Exercise 257** | Associated type in trait | Associated types |
| | **Exercise 258** | Generic trait implementation | Blanket implementations |
| | **Exercise 259** | Monomorphization concept | How generics work at compile time |
| 📝 | **Exercise 260** | Generic container | Building generic containers |

Understood. I will continue the `README.md` in English.

Here is the continuation of **PHASE 9** and the start of **PHASE 10: Collections & Iterators** for your Rust exercises.

---

## 🎨 PHASE 9: Traits & Generics (Exercises 231-265)
### Week 19: Generics (Continued)

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| | **Exercise 261** | Generic function with constraints | Applying multiple trait bounds on T |
| | **Exercise 262** | Generic trait methods | Defining and using methods specific to the generic type |
| | **Exercise 263** | Custom Result type | Creating a generic `MyResult<T, E>` enum |
| | **Exercise 264** | Implement `Debug` for Generic Struct | Ensuring all type parameters `T` also implement `Debug` |
| 📝 | **Exercise 265** | Generic stack implementation | Implementing a basic Stack (`push`/`pop`) using `Vec<T>` |

---

## 📚 PHASE 10: Collections & Iterators (Exercises 266-300)
### Week 20: HashMaps and Sets

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 266** | Create a basic `HashMap` | Using `use std::collections::HashMap;` |
| | **Exercise 267** | Insert key-value pairs | Inserting data with `.insert(key, value)` |
| | **Exercise 268** | Retrieve a value | Accessing values with `.get(&key)` (Returns `Option`) |
| ⭐ | **Exercise 269** | Handle non-existent key | Matching on the `Option` returned by `.get()` |
| | **Exercise 270** | Iterate over a `HashMap` | Looping through key-value pairs (`for (k, v) in &map`) |
| | **Exercise 271** | Update a value | Overwriting an existing key's value |
| ⭐ | **Exercise 272** | Check for presence | Checking if a key exists with `.contains_key(&key)` |
| | **Exercise 273** | Insert only if empty | Using `.entry(key).or_insert(value)` |
| | **Exercise 274** | Count word frequency | Practical use: counting occurrences of words in a string |
| | **Exercise 275** | Remove a key-value pair | Using `.remove(&key)` |
| | **Exercise 276** | Create a `HashSet` | Using `use std::collections::HashSet;` |
| | **Exercise 277** | Insert into `HashSet` | Adding unique elements with `.insert(value)` |
| | **Exercise 278** | Check for element existence | Checking if a value is present with `.contains(&value)` |
| | **Exercise 279** | Set union | Combining two sets with `.union()` |
| | **Exercise 280** | Set intersection | Finding common elements with `.intersection()` |

### Week 21: Iterators and Functional Programming

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| ⭐ | **Exercise 281** | Basic iterator creation | Creating an iterator with `.iter()` |
| | **Exercise 282** | Using `.next()` manually | Understanding how iterators step through elements |
| | **Exercise 283** | **Map** iterator adapter | Transforming each element with `.map()` |
| ⭐ | **Exercise 284** | **Filter** iterator adapter | Selecting elements based on a condition with `.filter()` |
| | **Exercise 285** | Combine `map` and `filter` | Chaining iterator methods |
| | **Exercise 286** | **Fold** (Reduce) operation | Accumulating a single result with `.fold()` |
| | **Exercise 287** | **Sum** method | Quickly summing numeric iterator items |
| | **Exercise 288** | **Take** and **Skip** | Limiting iteration with `.take()` and `.skip()` |
| | **Exercise 289** | **Find** method | Returning the first element that satisfies a condition |
| | **Exercise 290** | **Any** and **All** | Checking if *any* or *all* elements meet a condition |
| | **Exercise 291** | Infinite iterator | Using `std::iter::repeat()` or `std::iter::successors()` |
| | **Exercise 292** | Zipping iterators | Combining two iterators into one of pairs with `.zip()` |
| | **Exercise 293** | Flatten iterator | Handling nested iterators with `.flatten()` |
| | **Exercise 294** | Using closures with iterators | Defining inline functions for iteration logic |
| 📝 | **Exercise 295** | Custom struct for iteration | Implementing the `Iterator` trait manually (advanced) |

### Week 22: Final Challenges

| Status | Exercise | Goal | Concept |
| :---: | :--- | :--- | :--- |
| | **Exercise 296** | **Chaining** iterators | Concatenating two iterators with `.chain()` |
| | **Exercise 297** | Convert `HashMap` keys to `Vec` | Collecting only keys into a new vector |
| | **Exercise 298** | Filter `HashMap` by value | Creating a new map based on value constraints |
| | **Exercise 299** | Calculate median | Combining sorting and indexing/iterators |
| ⭐ | **Exercise 300** | **Final Project:** Simple CLI Calculator | Combine input, enums (operations), `match`, and functions |

---

