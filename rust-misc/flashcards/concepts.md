# Rust Concepts Flashcards

## 🔹 Ownership & Borrowing

**Q:** What are the three rules of ownership in Rust?  
**A:**  

1. Each value in Rust has a single owner.  
2. When the owner goes out of scope, the value is dropped.  
3. Ownership can be transferred (move) or borrowed (mutable/immutable references).

**Q:** What is the difference between `&T` and `&mut T`?  
**A:**  

- `&T` is an immutable reference (read-only).  
- `&mut T` is a mutable reference (allows modification).

---

## 🔹 Lifetimes

**Q:** What do lifetimes ensure in Rust?  
**A:** Lifetimes prevent dangling references and ensure references are valid for a specific duration.

**Q:** What does `'a` mean in Rust?  
**A:** `'a` is a lifetime annotation, indicating that a reference must live as long as `'a`.

---

## 🔹 Traits & Generics

**Q:** What is the purpose of Rust traits?  
**A:** Traits define shared behavior (similar to interfaces in other languages).

**Q:** How do you define a generic function in Rust?  

```rust
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}
