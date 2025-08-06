# calc_parser

A tiny Rust calculator that parses simple math expressions like `"10 + 2"` and evaluates them.

---

## Features

- Handles basic arithmetic: `+`, `-`, `*`, `/`
- Parses expressions like `10 + 2` (must be space-separated)
- Gracefully handles invalid input

---

## Example

```rust
let e = "10 + 2";
```

Output:

```
The answer is 12
```

---

## Project Structure

```
calc_parser/
├── src/
│   └── main.rs
├── Cargo.toml
```

---

## Running

```bash
cargo run
```

Modify the expression in `main()` to try out different calculations.

---

## Input Format

**Valid:**  
- `"10 + 2"`  
- `"25 * 4"`  

**Invalid:**  
- `"5+3"` — no spaces  
- `"5 ** 2"` — unsupported operator  
- `"five + 2"` — not a number  

---
