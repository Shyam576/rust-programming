
# 🦀 8-Week Rust Mastery Plan (From Zero → Builder)

## 📘 Overview
This plan is designed to help you go from zero Rust knowledge to confidently building open-source, system-level projects in 8 weeks.  
You’ll study for **45 minutes twice a day (≈1.5 hours daily)**, following a practical, hands-on path.

---

## 🗓️ Phase 1: Core Foundations (Week 1 – 2)
Goal → Understand Rust’s syntax, ownership, and safety model.  
Main resource: [Rustlings](https://github.com/rust-lang/rustlings)

### Week 1
| Day | Focus | Practice |
|-----|--------|-----------|
| 1 | Setup Rust + Cargo + rust-analyzer in VS Code | Run `cargo new hello_rust`; print “Hello, world!” |
| 2 | Variables, mutability, shadowing | Rustlings: *variables1-3* |
| 3 | Data types (integers, floats, strings, bools) | Rustlings: *types1-3* |
| 4 | Functions & scope | Write a `sum(a, b)` and `factorial(n)` function |
| 5 | Ownership rules | Rustlings: *move_semantics1-5* |
| 6 | Borrowing & references | Rustlings: *references1-3* |
| 7 | Lifetimes intro + review | Explain ownership in your own words |

### Week 2
| Day | Focus | Practice |
|-----|--------|-----------|
| 8 | Structs & methods | Build a `Rectangle` struct with an `area()` method |
| 9 | Enums & pattern matching | Rustlings: *enums1-3* |
| 10 | `Result`, `Option`, `?` operator | Handle file open errors safely |
| 11 | Control flow & loops | Write a fizzbuzz program |
| 12 | Modules & visibility | Organize code into multiple files |
| 13 | Collections (`Vec`, `HashMap`) | Count word frequency from a file |
| 14 | Review + mini-project | CLI calculator |

---

## 🧠 Phase 2: Intermediate Concepts (Week 3 – 4)
Goal → Learn traits, generics, modules, and error handling.

### Week 3
| Day | Focus | Practice |
|-----|--------|-----------|
| 15 | Traits (interfaces) | Define a `Speak` trait for `Dog` and `Cat` |
| 16 | Generics & trait bounds | Generic `max()` function |
| 17 | String handling (`String` vs `&str`) | Build a string builder CLI |
| 18 | Error handling (advanced) | Custom error type with `thiserror` |
| 19 | Cargo commands & project layout | Explore `cargo build`, `test`, `fmt`, `clippy` |
| 20 | File I/O basics | Build a log reader CLI |
| 21 | Review + mini-project | To-do CLI app |

### Week 4
| Day | Focus | Practice |
|-----|--------|-----------|
| 22 | Command line arguments (`std::env`) | CLI that accepts args |
| 23 | Crates and libraries | Use `serde_json` for JSON parsing |
| 24 | Unit testing (`cargo test`) | Add tests for your to-do app |
| 25 | Iterators & closures | Implement `map` & `filter` examples |
| 26 | Smart pointers (`Box`, `Rc`, `Arc`) | Build a small linked list |
| 27 | Lifetimes (deep dive) | Practice references between structs |
| 28 | Review + mini-project | File indexer listing large files |

---

## ⚙️ Phase 3: Systems & Async (Week 5 – 6)
Goal → Learn concurrency, async I/O, and unsafe Rust.

### Week 5
| Day | Focus | Practice |
|-----|--------|-----------|
| 29 | Concurrency & threads | Spawn 3 threads to print messages |
| 30 | Channels & message passing | Producer-consumer queue |
| 31 | Mutex & Arc | Shared counter between threads |
| 32 | Async intro (`tokio`) | Async HTTP request demo |
| 33 | Async streams & tasks | Build async downloader |
| 34 | Unsafe Rust | Experiment with raw pointers safely |
| 35 | Review + mini-project | Mini task scheduler |

### Week 6
| Day | Focus | Practice |
|-----|--------|-----------|
| 36 | Error propagation across threads | Use `JoinHandle` & error handling |
| 37 | File system & environment | Walk directories asynchronously |
| 38 | Logging & config | Add `env_logger` + `dotenv` |
| 39 | Writing a crate | Package your own helper crate |
| 40 | Publish crate to crates.io | Try publishing or documenting it |
| 41 | Review | Document what you’ve built so far |
| 42 | Mini-project | System monitor CLI |

---

## 🚀 Phase 4: Real Project (Week 7 – 8)
Goal → Build an open-source-ready project.

### Week 7
| Day | Focus | Practice |
|-----|--------|-----------|
| 43 | Choose your project | Create repo, setup Cargo workspace |
| 44 | Design architecture | Draw module diagram |
| 45 | Implement core module | Start with data structures |
| 46 | Implement CLI interface | Basic command handling |
| 47 | Logging + error handling | Make it robust |
| 48 | Unit & integration tests | Write real test cases |
| 49 | Write README & contribution guide | Prepare for open source |

### Week 8
| Day | Focus | Practice |
|-----|--------|-----------|
| 50 | Refactor for performance | Use profiling tools |
| 51 | Documentation (`cargo doc`) | Generate docs |
| 52 | Set up CI/CD | Auto-test on push |
| 53 | Publish alpha release | Share link with community |
| 54 | Collect feedback | Ask for issues/PRs |
| 55 | Fix bugs + review | Apply PR feedback |
| 56 | Celebrate 🎉 | You’re a Rust dev! |

---

## 🧩 Tools & Resources
- **IDE:** VS Code + `rust-analyzer`
- **Core Tools:** `cargo check`, `cargo fmt`, `cargo test`, `cargo clippy`
- **Useful Crates:** `serde`, `tokio`, `thiserror`, `clap`, `reqwest`
- **Communities:** [r/rust](https://www.reddit.com/r/rust), [Rust Discord](https://discord.gg/rust-lang), [users.rust-lang.org](https://users.rust-lang.org/)

---

## ✅ Outcome
By the end of 8 weeks, you’ll:
- Understand Rust’s ownership, concurrency, and async patterns
- Be able to build real CLI and system-level projects
- Have your first open-source project ready for contribution
