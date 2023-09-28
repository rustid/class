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
Part II

<div class="uppercase text-sm tracking-widest">
Azzam S.A
</div>

<div class="abs-bl mx-14 my-12 flex">
  <img src="/bgn-icon.png" class="w-10">
  <div class="ml-3 flex flex-col text-left">
    <div><b>Rust</b>Course</div>
    <div class="text-sm opacity-50">Sep. 22th, 2023</div>
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

# Course. Not talk!

<!--
- Follow along
- Slow paced
- Ask, don't wait
- You are expected to able to write Rust code!
-->


---
layout: center
---

# Follow along!

- [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)
- [Exercises](https://github.com/rustid/class/tree/master/2023/2023-09-22/src/exercise/)

<br>

<details>
  <summary>Show hints</summary>
  <a href="https://github.com/rust-lang/rustlings/blob/main/info.toml">Hints</a>
</details>

---
layout: center
---

# Simple Function

---

<Transform :scale="2">

```rust
fn main() {
    // Hi, 🦀
    let name = "ferris";
    println!("My name is {}!", name);
}
```

</Transform>

<!--
- Simple void function. Retuns `()` <br>

- Functions are introduced with `fn`.
- Blocks are delimited by curly braces like in C and C++.
- The `main` function is the entry point of the program.

- Rust uses macros for situations where you want to have a variable number of
  arguments (no function [overloading](basic-syntax/functions-interlude.md)).

- Rust is multi-paradigm. It has powerful [object-oriented programming features](https://doc.rust-lang.org/book/ch17-00-oop.html),
  and, while it is not a functional language, it includes a range of [functional concepts](https://doc.rust-lang.org/book/ch13-00-functional-features.html).
-->

---
layout: center
---

# More Examples

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
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn greet(name: &str) -> String {
    return format!("Hi, {}. 👋🏻", name); // 🛠️
}

fn main() {
    let result = add(5, 4);
    println!("5 + 4 = {}", result);

    println!("{}", greet("ferris"));
}
```

<!--
Function with return value <br>
Use `{result}` <br>
`return` keyword <br>
rustdoc
-->


---
layout: center
---

# Exercises 1

- [ ] `intro2`
- [ ] `variables1`
- [ ] `variables2`
- [ ] `variables3`
- [ ] `primitive_types1`
- [ ] `primitive_types2`
- [ ] `if1`
- [ ] `if2`

---
layout: center
---

# More about Functions

---

# Early Return

```rust
fn divide(a: i32, b: i32) -> Option<i32> {
    // Check if the divisor is zero and return early with None
    if b == 0 {
        println!("Error: Division by zero is not allowed.");
        return None;
    }

    Some(a / b)
}

fn main() {
    let result = divide(8, 0);
    println!("Result: {:?}", result);
}
```

<!--
- Early returning is possible with the `return` keyword.
-->

---

# Methods

```rust
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: i32) {
        self.width += delta;
    }
}

fn main() {
    let mut rect = Rectangle { width: 10, height: 5 };
    println!("old area: {}", rect.area());

    rect.inc_width(5);
    println!("new area: {}", rect.area());
}
```

---

# Function Overloading

```rust
fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 { a } else { b }
}

fn main() {
    println!("random number: {}", pick_one(500, 1000));
    println!("random figure: {}", pick_one("aragorn", "legolas"));
}
```

<!--
Overloading is not supported: <br>

- Each function has a single implementation:
  - Always takes a fixed number of parameters.
- Default values are not supported
-->


---
layout: center
---

# Variables


- Static and Constant Variables
- Type Inference

<br>

```rust
fn takes_i32(x: i32) {
    println!("i32: {x}");
}

fn main() {
    let x = 10;
    takes_i32(x);
}
```

<!--
Different than Python Dynamic type.
-->

---

# Scopes and Shadowing

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

---
layout: center
---

# Exercises 2

- [ ] `functions1`
- [ ] `functions2`
- [ ] `functions3`
- [ ] `variables5`
- [ ] `variables6`


---
layout: center
---

# Credits 🌟

- [Mo's (mo8it) Comprehensive Rust 🦀](https://comprehensive-rust.mo8it.com/)
- [rustlings 🦀](https://github.com/rust-lang/rustlings)
