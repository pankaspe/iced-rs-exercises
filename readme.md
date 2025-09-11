# Iced-RS Exercises

A collection of Rust exercises built with [Iced](https://github.com/iced-rs/iced), demonstrating modular, clean, and idiomatic Rust + GUI development.  
This repository is designed to serve as a learning playground for Rust GUI programming and the Elm Architecture pattern.

---

## 📝 Purpose

The goal of this repository is to:

- Practice Rust programming with a GUI framework.
- Learn the **Elm Architecture** (Model-Message-Update-View) in Iced.
- Demonstrate **modular code structure** in Rust.
- Build small, reusable exercises that can be extended over time.

Each exercise is self-contained inside a separate module/folder.

---

## 📂 Repository Structure

```

iced-rs-exercises/
├─ src/
│  ├─ main.rs         # Application entry point
│  ├─ counter/        # Counter exercise
│  │   ├─ mod.rs
│  │   ├─ state.rs
│  │   └─ message.rs
│  └─ todo/           # To-Do List exercise 
│      ├─ mod.rs
│      ├─ state.rs
│      └─ message.rs
├─ Cargo.toml
└─ README.md

````

---

## 🏗 Exercises

### 1. Counter
- Simple counter with `Increment`, `Decrement`, and `Reset`.
- Demonstrates:
  - Basic Elm Architecture in Iced.
  - Using `row!` and `column!` for layout.
  - Buttons, text, and container alignment.
  - Modular Rust structure with `mod.rs`, `state.rs`, and `message.rs`.

### 2. To-Do List
- Add, remove, and toggle tasks.
- Demonstrates:
  - Dynamic collections with `Vec<T>`.
  - Messages carrying data (`Add(String)`).
  - More complex UI layout using `row!` + `column!`.
  - Advanced Rust patterns (`match`, `enumerate`, `structs`, `enums`).

---

## ⚡ Features / Best Practices

- **Modular design:** Each exercise in its own folder with separate `state.rs` and `message.rs`.
- **Elm Architecture:** Clear separation between Model, Message, Update, and View.
- **Rust idioms:** Use of `struct`, `enum`, `derive(Default, Clone, Copy, Debug)`, `match` expressions.
- **Cross-platform GUI:** Powered by Iced (works on Windows, Linux, macOS, and WASM).
- **Commented code:** Each exercise contains detailed inline comments explaining the logic, Rust features, and Iced usage.

---

## 🚀 Getting Started

1. Clone the repository:

```bash
git clone https://github.com/your-username/iced-rs-exercises.git
cd iced-rs-exercises
````

2. Build and run an exercise (example: Counter):

```bash
cargo run
```

3. Explore other exercises as they are added.

---

## 📚 References

* [Iced Documentation](https://docs.rs/iced)
* [The Elm Architecture](https://guide.elm-lang.org/architecture/)
* [Rust Book](https://doc.rust-lang.org/book/)

---

## 💡 Notes

This repository is a **learning playground**. The code emphasizes **clarity, modularity, and idiomatic Rust** rather than production-level optimizations.