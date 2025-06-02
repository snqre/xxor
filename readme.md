# XXOR

> `XOR<This, That>` — for when both outcomes are valid, and either is okay.

---

Where `Option<T>` means *maybe*, and `Result<T, E>` means *success or error*,  
**`XOR<A, B>` means two possible **success** outcomes** — without the noise of errors or missing values.

---

## ✨ Features

- ✅ `XOR<A, B>`: holds either `This(A)` or `That(B)`.
- 🧼 Clean, focused API (`map_this`, `map_that`, `unwrap_*`, etc.).
- 🔒 `#![no_std]` compatible.
- 🧪 Derives `Debug`, `Clone`, `Eq`, and `PartialEq`.
- 🎯 Perfect for avoiding new `enum`s in small, focused cases.

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
xxor = "0.1.0"
```