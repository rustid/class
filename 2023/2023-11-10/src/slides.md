---
layout: cover
download: 'https://azzamsa.com/talks/'
highlighter: shiki
info: |
  ## Rust Course

  A Rust course developed by the engineering team at Biznet Gio.

  [Azzam S.A](https://azzamsa.com/) at Rust Course

---

# Rust Fundamentals

Basics of Rust<br>
Part 8

<div class="uppercase text-sm tracking-widest">
Azzam S.A
</div>

<div class="abs-bl mx-14 my-12 flex">
  <img src="/bgn-icon.png" class="w-10">
  <div class="ml-3 flex flex-col text-left">
    <div><b>Rust</b>Course</div>
    <div class="text-sm opacity-50">Nov. 10th, 2023</div>
  </div>
</div>


---
layout: 'intro'
---

# Azzam S.A

<div class="leading-8 opacity-80">
OSS devotee, speaker, and teacher. <br>
Open sourceror. Namely Rust, Python, and Emacs. <br>
</div>

<div class="mt-30"></div>

<div class="my-10 flex items-center space-x-4">
  <ri-github-line class="opacity-50"/><div><a href="https://github.com/azzamsa" target="_blank">azzamsa</a></div>
  <ri-twitter-line class="opacity-50"/><div><a href="https://twitter.com/azzamsyawqi" target="_blank">azzamsyawqi</a></div>
  <ri-user-3-line class="opacity-50"/><div><a href="https://azzamsa.com" target="_blank">azzamsa.com</a></div>
</div>


<img src="https://raw.githubusercontent.com/azzamsa/azzamsa/master/static/pfp/2023/face.png" class="rounded-full w-40 abs-tr mt-16 mr-12"/>


---
layout: center
---

# Follow along!

- [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
- [Exercises](https://github.com/rustid/class/tree/master/)

<br>

<details>
  <summary>Show hints</summary>
  <a href="https://github.com/rust-lang/rustlings/blob/main/info.toml">Hints</a>
</details>


---
layout: center
---

# Error Handling

---

## Error Handling

- Rust groups errors into two major categories: *recoverable* and *unrecoverable errors*.
- Rust doesn’t have exceptions. Instead, it has the type `Result\<T, E\>` and `panic!`.

<!--
For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program. <br>

Most languages don’t distinguish between these two kinds of errors and handle both in the same way, using mechanisms such as exceptions. <br>

Rust doesn’t have exceptions. Instead, it has the type `Result\<T, E\>` and `panic!` for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.
-->

## Unrecoverable Errors with `panic!`

- *Unwinding the stack* vs *aborting*.
- No *buffer overread* in Rust. It stop and refuse to continue.
- Use `RUST_BACKTRACE `environment variable to get a backtrace.
- Debug symbols are enabled by default, unless built with the `--release` flag.


```rust
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is
99', src/main.rs:4:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

<!--
You can switch from unwinding to aborting upon a panic by adding panic = 'abort' <br>

In C, attempting to read beyond the end of a data structure is undefined behavior. You might get whatever is at the location in memory that would correspond to that element in the data structure, even though the memory doesn’t belong to that structure. This is called a buffer overread and can lead to security vulnerabilities if an attacker is able to manipulate the index in such a way as to read data they shouldn’t be allowed to that is stored after the data structure.
To protect your program from this sort of vulnerability, if you try to read an element at an index that doesn’t exist, Rust will stop execution and refuse to continue
-->

---

## Recoverable Errors with Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- In the case where `File::open` succeeds ➜  an instance of `Ok` that contains a file handle.
- In the case where it fails ➜ an instance of `Err` that contains more information about the kind of error that occurred.

```rust
fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error);
        }
    };
}
```

---

## Alternatives

- Using `if`-`else`
- `unwrap` and `expect`

```rust
fn main() {
    let greeting_file = File::open("hello.txt").unwrap();
}
```

Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.

```rust
fn main() {
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

In production enviroment, use `expect` to get give more context about why the operation is expected to always succeed.

---

## Propagating Errors

- Returns the error to the caller.

```rust
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
```

---

## A Shortcut for Propagating Errors

- The `?` Operator
- Unlike `match`, `?` goes though the `from` funcion in `From` trait.

```rust {2,4-5}
fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
```

<!--
This pattern of propagating errors is so common in Rust that Rust provides the question mark operator ? to make this easier. <br>

There is a difference between what the match expression from Listing 9-6 does and what the ? operator does: error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert values from one type into another. When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function. This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons. <br>
-->

---

## Where we can use the `?` operator

- As with the `Result`, we can use `?` with `Option<T>` as long the function returns `Options`.

```rust
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
```

- You can't mix and match `?` in `Result` and `Option`.
- The `?` operator won’t automatically convert a `Result` to an `Option` or vice versa; use `ok` method on Result or the `ok_or` method on Option to do the conversion explicitly.

<!--
Important for one liner; coding competition
-->

---

## Executable return values

```rust{1}
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;
    Ok(())
}
```

- If a main function returns a `Result<(), E>`, the executable will exit with `0` or nonzero value.
- Rust follows C convention in this case.

<!--
When a main function returns a Result<(), E>, the executable will exit with a value of 0 if main returns Ok(()) and will exit with a nonzero value if main returns an Err value. Executables written in C return integers when they exit: programs that exit successfully return the integer 0, and programs that error return some integer other than 0. Rust also returns integers from executables to be compatible with this convention.
-->

---

## To `panic!` vs Not to `panic!`

- `panic!`
  - Examples, prototype code, and tests
  - You Have More Information Than the Compiler

- Using robust error-handling code can make the example and the target concept less clear.
- `unwrap` and `expect` act as clear markers in prototype, before you’re ready to decide how to handle errors.

```rust
let home: IpAddr = "127.0.0.1"
    .parse()
    .expect("Hardcoded IP address should be valid");
```

---

## Guidelines

- `panic!`
  - The program end up in bad state.
  - Something unexpected, not something that will likely happen occasionally.
  - Harmful or insecure such out-of-bounds memory access.
- Not `panic!`
  - Occasional error.
  - Bad HTTP request, malformed input for parser.

Relies as much as possible to the Rust type system, such as missing arguments, and negative values.

<!--
Similarly, the unwrap and expect methods are very handy when prototyping, before you’re ready to decide how to handle errors. They leave clear markers in your code for when you’re ready to make your program more robust. <br>

This is the main reason the standard library will call panic! if you attempt an out-of-bounds memory access: trying to access memory that doesn’t belong to the current data structure is a common security problem <br>

-->

---

## Creating Custom Types for Validation

Avoid performance penalty by not doing too much checking.

```rust{5-11}
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

---

Use public method to access `value` to prevent setting a value directly.

```rust{12-15}
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

---
layout: center
---

# Testing

- Strong type is not enough

```rust
fn add_ten(x: i32) -> i32 {
    x + 10
}
```

---

The test function anatomy:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
       let result = 2 + 2;
       assert_eq!(result, 4);
    }
}
```

- Test module can contain non-test function.
- It’s possible to mark a test as ignored so it doesn’t run in a particular instance
- Cargo able run specific test. Test filtering.
- Doc-tests helps keep your docs and your code in sync.

<!--
Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed. <br>
-->

---

```rust
/// Shortens a string to the given length.
///
/// ```
/// use playground::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}
```

---

```rust
$ cargo test

running 1 test
test tests::it_works ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0
filtered out; finished in 0.00s

  Doc-tests adder

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0
filtered out; finished in 0.00s
```

---

## Checking Test Results

- Using `assert!` macro for boolean condition.
- `assert_eq!`, and `assert_ne!` for testing equality.
- `assert_eq!(<left>, <right>)`. expected <=> actual.
- These macros print using debug formatting, means the values being compared must implement `PartialEq` and `Debug` traits.
- Can have custom failure messages.

```rust
#[test]
fn test_empty() {
    assert_eq!(first_word(""), "");
}
```

```rust
assert!(
        result.contains("Carol"),
        "Greeting did not contain name, value was `{result}`"
    );
```


<!--
Note that in some languages and test frameworks, the parameters to equality assertion functions are called expected and actual, and the order in which we specify the arguments matters. However, in Rust, they’re called left and right, and the order in which we specify the value we expect and the value the code produces doesn’t matter. <br>
-->

---

## Checking Test Results

- Use `should_panic` to check panics.

```rust{2}
#[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
```

- Make it more precise with `expected`.

```rust{2}
#[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
```

---

## Using `Result<T, E>` in Tests

- Gives the ability to use `?` inside tests.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}
```

---

## More...

- `cargo test -- --test-threads=1`
- `cargo test -- --show-output`
- `cargo test <test name>`
- Ignoring some tests unless specifically requested.

```rust{2}
#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}
```

---

## Integration tests

Create a `.rs` file under `tests/`:

```rust
use my_library::init;

#[test]
fn test_init() {
    assert!(init().is_ok());
}
```


---
layout: center
---

# Credits 🌟

- [Mo's (mo8it) Comprehensive Rust 🦀](https://comprehensive-rust.mo8it.com/)
- [rustlings 🦀](https://github.com/rust-lang/rustlings)
