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
Part III

<div class="uppercase text-sm tracking-widest">
Azzam S.A
</div>

<div class="abs-bl mx-14 my-12 flex">
  <img src="/bgn-icon.png" class="w-10">
  <div class="ml-3 flex flex-col text-left">
    <div><b>Rust</b>Course</div>
    <div class="text-sm opacity-50">Sep. 29th, 2023</div>
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
- [Exercises](https://github.com/rustid/class/tree/master/2023/2023-09-29/src/exercise/)

<br>

<details>
  <summary>Show hints</summary>
  <a href="https://github.com/rust-lang/rustlings/blob/main/info.toml">Hints</a>
</details>


---
layout: center
---

# Memory Management

---

# The Stack vs The Heap

- **Stack**: Continuous area of memory for local variables.
  - Values have **fixed sizes** known at compile time.
  - **Extremely fast**: just move a stack pointer.
  - Easy to manage: follows function calls.

- Heap: Storage of values outside of function calls.
  - Values have **dynamic sizes** determined at runtime.
  - **Slower** than the stack: some book-keeping needed.

---

```rust
fn main() {
    let s1 = String::from("Hello");
}
```

```text
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - -.
:                           :     :                               :
:    s1                     :     :                               :
:   +-----------+-------+   :     :   +----+----+----+----+----+  :
:   | ptr       |   o---+---+-----+-->| H  | e  | l  | l  | o  |  :
:   | len       |     5 |   :     :   +----+----+----+----+----+  :
:   | capacity  |     5 |   :     :                               :
:   +-----------+-------+   :     :                               :
:                           :     `- - - - - - - - - - - - - - - -'
`- - - - - - - - - - - - - -'
```

<!--
`String` is backed by a `Vec`, so it has a capacity and length and can grow if mutable via reallocation on the heap.
-->

---

<div align="center">
  <img src='/stack-vs-heap.png' />
  <figcaption>Head First Java by Kathy Sierra</figcaption>
</div>


<!--
- stack
- heap: pool of data
-->

---
layout: center
---

# Ownership

---

```rust
fn main() {
    {
        let x = 42;
        println!("x: {x}");
    } // variable `x` is dropped, data is freed.

    println!("x: {x}");
}
```


<!--
A destructor can run to free up resources. <br>
We say that the variable owns the value. <br>
-->

---
layout: center
---

# Move Semantics

```rust
fn main() {
    let s1: String = String::from("Hello!");
    let s2: String = s1;

    println!("s2: {s2}");
    //println!("s1: {s1}");
}
```

- The assignment of `s1` to `s2` transfers ownership.
- There is always _exactly_ one variable binding which owns a value.

<!--
- When `s1` goes out of scope, nothing happens: it does not own anything.
- When `s2` goes out of scope, the string data is freed.
-->

---

```rust
error[E0382]: borrow of moved value: `s1`
 --> src/main.rs:6:19
  |
2 |     let s1: String = String::from("Hello!");
  |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
3 |     let s2: String = s1;
  |                      -- value moved here
...
6 |     println!("s1: {s1}");
  |                   ^^^^ value borrowed here after move
  |
...
  |
3 |     let s2: String = s1.clone();
  |                        ++++++++

```

---
layout: center
---

# Moved Strings in Rust

```rust
fn main() {
    let s1: String = String::from("Rust");
    let s2: String = s1;
}
```

- The heap data from `s1` is reused for `s2`.
- When `s1` goes out of scope, nothing happens (it has been moved from).

---

Before move to `s2`:

```text
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - -.
:                           :     :                           :
:    s1                     :     :                           :
:   +-----------+-------+   :     :   +----+----+----+----+   :
:   | ptr       |   o---+---+-----+-->| R  | u  | s  | t  |   :
:   | len       |     4 |   :     :   +----+----+----+----+   :
:   | capacity  |     4 |   :     :                           :
:   +-----------+-------+   :     :                           :
:                           :     `- - - - - - - - - - - - - -'
:                           :
`- - - - - - - - - - - - - -'
```

---

After move to `s2`:

```text
 Stack                             Heap
.- - - - - - - - - - - - - -.     .- - - - - - - - - - - - - -.
:                           :     :                           :
:    s1 "(inaccessible)"    :     :                           :
:   +-----------+-------+   :     :   +----+----+----+----+   :
:   | ptr       |   o---+---+--+--+-->| R  | u  | s  | t  |   :
:   | len       |     4 |   :  |  :   +----+----+----+----+   :
:   | capacity  |     4 |   :  |  :                           :
:   +-----------+-------+   :  |  :                           :
:                           :  |  `- - - - - - - - - - - - - -'
:    s2                     :  |
:   +-----------+-------+   :  |
:   | ptr       |   o---+---+--'
:   | len       |     4 |   :
:   | capacity  |     4 |   :
:   +-----------+-------+   :
:                           :
`- - - - - - - - - - - - - -'
```

---
layout: center
---

# Moves in Function Calls

```rust
fn say_hello(name: String) {
    println!("Hello {name}")
} // The heap memory allocated for `name` will be freed here


fn main() {
    let name = String::from("Alice");
    say_hello(name); // Ownership transferred to say_hello
    // say_hello(name); // You can't use `name` here anymore because it was moved
}
```

---

```rust
fn say_hello(name: &String) {
    println!("Hello, {}", name);
}

fn main() {
    let name = String::from("Alice");
    say_hello(&name); // Borrowing `name`, ownership remains with `main`
    // You can still use `name` here
}
```

---

```rust
fn say_hello(name: String) {
    println!("Hello {}", name);
}

fn main() {
    let name = String::from("Alice");
    let cloned_name = name.clone();
    say_hello(cloned_name);
    // You can still use `name` here because you cloned it.
```

<!--
These avoid many memory issues in other programming languages that lack similar ownership and borrowing mechanisms.
-->

---
layout: center
---

# Copying and Cloning

Move semantics are the default, but certain types are copied by default.

```rust
fn main() {
    let x = 42;
    let y = x;

    println!("x: {x}");
    println!("y: {y}");
}
```

These types implement the `Copy` trait.

- Primitive numeric types
- Tuples (if all their elements implement Copy)
- Fixed-size arrays (if their elements implement Copy)
- Some built-in types
- User-Defined types

---

```rust
#[derive(Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;

    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
```

---

```rust
error[E0382]: borrow of moved value: `p1`
 --> src/main.rs:8:19
  |
5 |     let p1 = Point(3, 4);
  |         -- move occurs because `p1` has type `Point`, which does not implement the `Copy` trait
6 |     let p2 = p1;
  |              -- value moved here
7 |
8 |     println!("p1: {p1:?}");
  |                   ^^^^^^ value borrowed here after move
  |
```

---

```rust {1}
#[derive(Copy, Clone, Debug)]
struct Point(i32, i32);

fn main() {
    let p1 = Point(3, 4);
    let p2 = p1;

    println!("p1: {p1:?}");
    println!("p2: {p2:?}");
}
```

---
layout: center
---

# Shared and Unique Borrows

Rust puts constraints on the ways you can borrow values:

- You can have one or more `&T` values at any given time, _or_
- You can have exactly one `&mut T` value.


---
layout: center
---

# Lifetimes

A borrowed value has a _lifetime_:

- The lifetime can be implicit: `add(p1: &Point, p2: &Point) -> Point`.
- Lifetimes can also be explicit: `&'a Point`, `&'document str`.

---

# Lifetimes in Function Calls

```rust
#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p_ref: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p_ref);
}
```

---

```rust{4}
#[derive(Debug)]
struct Point(i32, i32);

fn left_most<'a>(p1: &'a Point, p2: &'a Point) -> &'a Point {
    if p1.0 < p2.0 { p1 } else { p2 }
}

fn main() {
    let p1: Point = Point(10, 10);
    let p2: Point = Point(20, 20);
    let p_ref: &Point = left_most(&p1, &p2);
    println!("left-most point: {:?}", p_ref);
}
```

---

# Lifetimes in Data Structures

```rust
#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{fox:?}");
    println!("{dog:?}");
}
```

---

```rust{2}
#[derive(Debug)]
struct Highlight<'doc>(&'doc str);

fn main() {
    let text = String::from("The quick brown fox jumps over the lazy dog.");
    let fox = Highlight(&text[4..19]);
    let dog = Highlight(&text[35..43]);
    println!("{fox:?}");
    println!("{dog:?}");
}
```

---
layout: center
---

# Exercises

- `move_semantics1`
- `move_semantics2`
- `move_semantics3`
- `lifetimes1`
- `lifetimes2`
- `lifetimes3`

---
layout: center
---

# Credits 🌟

- [Mo's (mo8it) Comprehensive Rust 🦀](https://comprehensive-rust.mo8it.com/)
- [rustlings 🦀](https://github.com/rust-lang/rustlings)
