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
Part VI

<div class="uppercase text-sm tracking-widest">
Azzam S.A
</div>

<div class="abs-bl mx-14 my-12 flex">
  <img src="/bgn-icon.png" class="w-10">
  <div class="ml-3 flex flex-col text-left">
    <div><b>Rust</b>Course</div>
    <div class="text-sm opacity-50">Nov. 3th, 2023</div>
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

# Standard Library

The common vocabulary types include:

- **Box\<T\>**: for allocating values on the heap.
- **Rc\<T\>**: a reference counting type that enables multiple ownership.

---
layout: center
---

# Box

- `Boxes` allow you to store data on the heap rather than the stack.

```rust
fn main() {
    let five = Box::new(5);
    println!("five: {}", five);
}
```

```text
 Stack                     Heap
.- - - - - - -.     .- - - - - - -.
:             :     :             :
:    five     :     :             :
:   +-----+   :     :   +-----+   :
:   | o---|---+-----+-->|  5  |   :
:   +-----+   :     :   +-----+   :
:             :     :             :
:             :     :             :
`- - - - - - -'     `- - - - - - -'
```

---

## When to use `Box`?

- When the size of a type is unknown at compile time, but it's needed in a context that requires a fixed size.
- When transferring ownership of a large data chunk without copying it.
- When you want to own a value based on a specific trait, not a particular type.

---

```rust{3}
fn main() {
    let five = Box::new(5);
    println!("five: {}", *five);
}
```


```rust{3}
fn main() {
    let five = Box::new(5);
    println!("five: {}", five);
}
```

<!--
- For `*y` behind the scenes Rust actually ran `*(y.deref())`.
- You can even leave out the `*`, thanks to `Deref`.
- A `Box` can be useful when you:
  - Have a type whose size that can't be known at compile time.
  - Want to transfer ownership of a large amount of data. To avoid copying large amounts of data on the stack, instead store the data on the heap in a `Box` so only the pointer is moved.
-->

---

## Box with Recursive Data Structures

Recursive data types or data types with dynamic sizes need to use a `Box`:

```rust
#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

fn main() {
    let list: List<i32> = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("{list:?}");
}
```

- Recursive types pose an issue because, at compile time, Rust needs to know how much space a type takes up.
- The cons list isn’t a commonly used data structure in Rust. Most of the time, `Vec\<T\>` is a better choice to us.
---

```text
 Stack                           Heap
.- - - - - - - - - - - - -.     .- - - - - - - - - - - - - - - - - - - - - - - -.
:                         :     :                                               :
:    list                 :     :                                               :
:   +------+----+----+    :     :    +------+----+----+    +------+----+----+   :
:   | Cons | 1  | o--+----+-----+--->| Cons | 2  | o--+--->| Nil  | // | // |   :
:   +------+----+----+    :     :    +------+----+----+    +------+----+----+   :
:                         :     :                                               :
:                         :     :                                               :
'- - - - - - - - - - - - -'     '- - - - - - - - - - - - - - - - - - - - - - - -'
```

<!--

- If `Box` was not used and we attempted to embed a `List` directly into the `List`,
  the compiler would not compute a fixed size of the struct in memory (**`List` would be of infinite size**).

- `Box` solves this problem as it has the same size as a regular pointer and just points at the next
  element of the `List` in the heap.

- Remove the `Box` in the List definition and show the compiler error. "Recursive with indirection" is a hint you might want to use a Box or reference of some kind, instead of storing a value directly.

-->

---

## The compiler is smart enough

Even if you missed the `Box`, the compiler is here for you!

```rust
error[E0072]: recursive type `List` has infinite size
 --> src/main.rs:2:1
  |
2 | enum List<T> {
  | ^^^^^^^^^^^^
3 |     Cons(T, List<T>),
  |             ------- recursive without indirection
  |
help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
  |
3 |     Cons(T, Box<List<T>>),
  |             ++++       +
```

- "Indirection" means that instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.

---
layout: center
---

# Rc (Reference Counted)

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(10);
    let b = Rc::clone(&a);

    println!("a: {a}");
    println!("b: {b}");
}
```

---
layout: center
---

# Exersices

- `box1.rs`
- `rc1.rs`

---

## `box1.rs`


```diff
#[derive(PartialEq, Debug)]
pub enum List {
-   Cons(i32, List),
+   Cons(i32, Box<List>),
    Nil,
}

pub fn create_empty_list() -> List {
+    List::Nil
}

pub fn create_non_empty_list() -> List {
+    List::Cons(10, Box::new(List::Nil))
}
```

---


## `rc1.rs`


`Rc::clone()` https://doc.rust-lang.org/std/rc/struct.Rc.html#method.clone

> This creates another pointer to the same allocation, increasing the strong reference count.

```diff
+    let saturn = Planet::Saturn(Rc::clone(&sun));
     println!("reference count = {}", Rc::strong_count(&sun)); // 7 references
     saturn.details();

+    let uranus = Planet::Uranus(Rc::clone(&sun));
     println!("reference count = {}", Rc::strong_count(&sun)); // 8 references
     uranus.details();

+    let neptune = Planet::Neptune(Rc::clone(&sun));

+    drop(earth);
     println!("reference count = {}", Rc::strong_count(&sun)); // 3 references

+    drop(venus);
     println!("reference count = {}", Rc::strong_count(&sun)); // 2 references

+    drop(mercury);
     println!("reference count = {}", Rc::strong_count(&sun)); // 1 reference
```


---
layout: center
---

# Generics

---

## Generic Data Types

You can use generics to abstract over the concrete field type:

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    println!("{integer:?} and {float:?}");
}
```

<!--
- Try declaring a new variable `let p = Point { x: 5, y: 10.0 };`.
  - Fix the code to allow points that have elements of different types.
-->

---

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let p = Point { x: 5, y: 10.0 };
}
```

Gives a compiler error: <br>

```rust
error[E0308]: mismatched types
 --> src/main.rs:8:30
  |
8 |     let p = Point { x: 5, y: 10.0 };
  |                              ^^^^ expected integer, found floating-point number

For more information about this error, try `rustc --explain E0308`.
error: could not compile `playground` (bin "playground") due to previous error
```

Why? 🤔

---

## Generic Methods

You can declare a generic type on your `impl` block:

```rust
#[derive(Debug)]
struct Point<T>(T, T);

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.0  // + 10
    }
}

fn main() {
    let p = Point(5, 10);
    println!("p.x = {}", p.x());
}
```

---

## Monomorphization

Generic code is turned into non-generic code based on the call sites:

```rust
fn main() {
    let integer = Some(5);
    let float = Some(5.0);
}
```

Behaves as if you wrote

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```

This is a **zero-cost abstraction**: you get exactly the same result as if you had hand-coded the data structures without the abstraction.


---
layout: center
---

# Exercises

---

## `generics1.rs`

```diff
fn main() {
-   let mut shopping_list: Vec<?> = Vec::new();
+   let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
```

---

## `generics2.rs`

```diff
- struct Wrapper {
+ struct Wrapper<T> {
-    value: u32,
+    value: T,
}

- impl Wrapper {
+ impl<T> Wrapper<T> {
-    pub fn new(value: u32) -> Self {
+    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}
```

---
layout: center
---

# Credits 🌟

- [Mo's (mo8it) Comprehensive Rust 🦀](https://comprehensive-rust.mo8it.com/)
- [rustlings 🦀](https://github.com/rust-lang/rustlings)
