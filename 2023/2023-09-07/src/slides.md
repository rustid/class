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
Part I

<div class="uppercase text-sm tracking-widest">
Azzam S.A
</div>

<div class="abs-bl mx-14 my-12 flex">
  <img src="/bgn-icon.png" class="w-10">
  <div class="ml-3 flex flex-col text-left">
    <div><b>Rust</b>Course</div>
    <div class="text-sm opacity-50">Sep. 8th, 2023</div>
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


<img src="https://avatars.githubusercontent.com/u/17734314?v=4" class="rounded-full w-40 abs-tr mt-16 mr-12"/>

---
layout: center
---

# Why Rust?

- ⚙️ Reliable: "if it compiles, it works"
- 🤸 Versatile: "you can do anything with Rust"

<!--
Type safety <br>
Consider all cases <br>

From low level to high level. <br>
Kernel, Server, Wasm. <br>
Full-Stack Rust <brt.

Clippy: very helpful! <br>

[Considering Rust by Jon Gjengset - YouTube](https://www.youtube.com/watch?v=DnT-LUQgc7s)
-->


---
layout: center
---

# Basic Syntax


---
layout: center
---

# Comment

```rust
fn main() {
    // Rust programs start with fn main()
    let some_number = 100; // We can write as much as we want here and the compiler won't look at it
}
```

```rust
fn main() {
    let some_number/*: i16*/ = 100;
    /* Block comment
    It's 100, which is my favourite number.
    It's called some_number but actually I think that... */
}
```

---
layout: center
---

# Types

---

# Scalar Types

|                        | Types                                      | Literals                       |
|------------------------|--------------------------------------------|--------------------------------|
| Signed integers        | `i8`, `i16`, `i32`, `i64`, `i128`, `isize` | `-10`, `0`, `1_000`, `123_i64` |
| Unsigned integers      | `u8`, `u16`, `u32`, `u64`, `u128`, `usize` | `0`, `123`, `10_u16`           |
| Floating point numbers | `f32`, `f64`                               | `3.14`, `-10.0e20`, `2_f32`    |
| Strings                | `&str`                                     | `"foo"`, `"two\nlines"`        |
| Unicode scalar values  | `char`                                     | `'a'`, `'α'`, `'∞'`            |
| Booleans               | `bool`                                     | `true`, `false`                |

<!--
Scalar types represent single values <br>

isize, usize (32-bit): i32 and u32 <br>
isize, usize (64-bit): i64 and u64

usize is used for indexing! <br>
must be positive, must be big, must be cross-platform.
-->

---

# Compound Types

|        | Types                         | Literals                          |
|--------|-------------------------------|-----------------------------------|
| Arrays | `[T; N]`                      | `[20, 30, 40]`, `[0; 3]`          |
| Tuples | `()`, `(T,)`, `(T1, T2)`, ... | `()`, `('x',)`, `('x', 1.2)`, ... |

<br>

```rust
fn main() {
    let mut array: [i8; 10] = [42; 10];
    array[5] = 0;
    println!("array contais: {:?}", array);

    let tuple: (i8, bool) = (7, true);
    println!("2nd index: {}", tuple.1);
}
```

<!--
Arrays and tuples have a fixed length. <br>
array is homogeneous, tuple is heterogeneous <br>
Compound types group multiple values into a single type. <br>
display and debug print <br>
debug contains more for programmer<br>
The empty tuple () is also known as the "unit type". default return value (void) <br>

explain about mut, Immutable by default <br>
-->

---

```rust
fn main() {
    let number: u8 = 10;
    let number = 10u8;
    let number = 10_u8;
    let number = 0________u8;

    let name = "Ponyo";
    println!("Hello, {}!", name);
}
```

<br>

<twemoji-crab class="text-sm text-orange-400 animate-bounce" /> [Try me](https://gist.github.com/rust-play/3645328be7a86889acfbfa3a7531d6d4)

<!--
_ : easy to read <br>
&str: reference <br>
type inference != dynamic type inference<br>
explain about shadowing <br>
-->

---

```rust
/// Adds two numbers and returns the result.
///
/// # Examples
///
/// ```
/// let result = add();
/// assert_eq!(result, 10);
/// ```
fn add() -> i32 {
    8 + 2
}

fn main() {
    println!("result: {}", add());
}
```


<!--
rustdoc <br>
{}: experssion <br>
function order doesn't matter.
-->


---

# The stack, the heap, and pointers


| **Aspect**         | **Stack** 📚                       | **Heap**  🗄️                        |
|--------------------|------------------------------------|------------------------------------|
| Memory Location    | Fast.                              | Relatively slower                  |
| Allocation         | Known size                         | Unknown size (String, Vectors)     |

<br>

The pointer in Rust is called a *reference* (`&`).

<!--
reference: borrow
-->

---

# Strings

## &str

- Fast
- *Does not have ownership.*
- Immutable.

## String

- *Has ownership.*
- `String` is a heap-allocated, growable string type.
- Mutable.
- Dynamic memory allocation on the heap.
- Supports operations like appending, resizing, and more.

<!--
&: refernece (location is in stack) <br>
-->

---

```rust
fn main() {
    let name = "Ashitaka";

    let name: String = "アシタカ".to_string();
    let name: String = String::from("アシタカ");
    let name: String = format!("{}", "アシタカ");

    let name = &name;

    println!("My name is {}", name);
}
```

---

```rust
fn return_str() -> &str {
    let country = String::from("Austria");
    let country_ref = &country;
    country_ref // ⚠️
}

fn main() {
    let country = return_str();
}
```

---
layout: center
---

# Control Flow

```rust
fn main() {
    // Using if-else to check a condition
    let number = 10;
    if number % 2 == 0 {
        println!("{} is even.", number);
    } else {
        println!("{} is odd.", number);
    }

    // Using a for loop to iterate over a range of numbers
    println!("Counting from 1 to 5:");
    for i in 1..=5 {
        println!("{}", i);
    }

    // Using a while loop to count down from 3 to 1
    let mut countdown = 3;
    println!("Countdown:");
    while countdown > 0 {
        println!("{}", countdown);
        countdown -= 1;
    }
}
```

---

```rust
fn main() {
    let mut counter = 0; // set a counter to 0
    loop {
        counter +=1; // increase the counter by 1
        println!("The counter is now: {}", counter);
        if counter == 5 { // stop when counter == 5
            break;
        }
    }
}
```

---
layout: center
---

# Variables

---

## Const and Static

```rust
const NUMBER_OF_MONTHS: u32 = 12;
static SEASONS: [&str; 4] = ["Spring", "Summer", "Fall", "Winter"];
```

<!--
static is similar to const, but has a fixed memory location and can act as a global variable
-->

---

## Scopes and Shadowing

```rust
fn main() {
    let a = 10;
    println!("before: {a}");

    {
        let a = "hello";
        println!("inner scope: {a}");

        let a = true;
        println!("shadowed in inner scope: {a}");
    }

    println!("after: {a}");
}
```
