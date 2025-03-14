# Rust Quiz Questions

## 🔹 Basics

**Q1:** What is the primary difference between `String` and `&str`?  
**A:**
a) `String` is mutable, `&str` is immutable  
b) `String` is heap-allocated, `&str` is a slice  
c) `&str` has ownership, `String` does not  

**✅ Correct Answer:** (b)

## 🔹 Ownership & Borrowing

**Q2:** What will the following code output?

```rust
fn main() {
    let s = String::from("hello");
    let r = &s;
    println!("{}", s);
}

a) "hello"
b) Compilation error (cannot move after borrowing)
c) Runtime error

**✅ Correct Answer:** (b)

**Q3:** What is the purpose of the impl Trait syntax?
**A:**
a) It allows implementing a trait on a type
b) It defines a generic function with trait bounds
c) It dynamically dispatches trait objects
✅ Correct Answer: (b)
