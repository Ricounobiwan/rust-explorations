# Memory Tricks for Rust

## Borrowing & Ownership

- **Think of Rust ownership like real estate:**  
  - One owner per house (one owner per value).  
  - Renting out = borrowing (immutable reference).  
  - Renovation needs permission = mutable reference.

## Lifetimes

- **Mnemonic: "A borrowed book must be returned before the library closes."**  
  - This helps remember that a borrowed value must live at least as long as the reference.

## Error Handling

- **"Result is for errors you expect. Panic is for errors you don’t."**  
  - `Result<T, E>` is recoverable.  
  - `panic!()` is a program-ending failure.
