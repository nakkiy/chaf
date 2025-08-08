# **`chaf`** – Not a finder. A *filter*.

---

## ✨ What is **chaf**?

**chaf** is a text filtering tool with the following features:

1. Logical expressions for condition specification  
1. Instead of "searching" like `grep`, it "excludes" specified lines  
1. Supports standard input/output, making it pipe-friendly  
1. Uses stdin when no input file is provided  
1. Invert exclusion with `--invert` to act like `grep`  
1. Display summary report with `--report`  
1. Not as fast as `grep`, but can process ~10 million lines in a few seconds  

---

## 💡 Why was it created?

While `grep -v` is convenient, expressing complex exclusion rules becomes unreadable.  
For example: "Exclude `DEBUG`, but keep `[DEBUG] connect DB`"  
Such conditions are difficult to express clearly with `grep`.

**chaf** lets you write exclusion logic with logical expressions — a filter *dedicated to exclusion*.  

---

## 🖥️ Example Usage

```bash
## 🖥️ Example Usage

```bash
$ cat logs.txt
error: failed to connect
warn: deprecated API used
debug: retrying connection
info: connection established
warn: low disk space
error: timeout while waiting for response

# Lines containing "debug" or "warn" are excluded
$ chaf 'debug | warn' logs.txt
error: failed to connect
info: connection established
error: timeout while waiting for response

# Excludes:
# - Lines containing "debug"
# - Lines containing "warn" unless they also contain "API"
$ chaf 'debug | (warn & !API)' logs.txt
error: failed to connect
warn: deprecated API used
info: connection established
error: timeout while waiting for response
# Explanation:
# → "warn: low disk space" is excluded (no "API")
# → "warn: deprecated API used" is kept (contains "API")

# Invert mode: show only lines that match the condition
# Includes:
# - Lines containing "debug"
# - Lines containing "warn" unless they also contain "API"
$ chaf --invert 'debug | (warn & !API)' logs.txt
debug: retrying connection
warn: low disk space
```

---

## 🚀 Installation

```bash
cargo install --git https://github.com/nakkiy/chaf
```

The `chaf` binary will be placed in:

```bash
$HOME/.cargo/bin/chaf
```

If your `PATH` doesn't include that directory, add it like so:

```bash
export PATH="$HOME/.cargo/bin:$PATH"
```

---

## ✔ Command-Line Interface

```
chaf [OPTIONS] <QUERY> [FILE]
```

---

## 📝 Query DSL (Logical Filter Language)

| Syntax                           | Meaning                                                       |
|----------------------------------|----------------------------------------------------------------|
| `aaa & bbb`                      | AND — match lines containing both `aaa` and `bbb` (exclude)   |
| `aaa \| bbb`                     | OR — match lines containing either `aaa` or `bbb` (exclude)   |
| `!aaa`                           | NOT — match lines not containing `aaa` (exclude)              |
| `(aaa \| bbb)`, `(aaa & bbb)`    | Use parentheses to group and control precedence               |

---

## Operator Precedence

```
1. Parentheses "()"
2. NOT "!"
3. AND "&"
4. OR "|"
```

---

## 🎛️ Available Options

| Option              | Description                                                        |
|---------------------|--------------------------------------------------------------------|
| `--report`, `-r`     | Show summary: total lines, excluded lines, output lines            |
| `--invert`, `-i`     | Invert filter to show *matching* lines only (like `grep`)         |
| `--help`, `-h`       | Show help message                                                  |
| `--version`, `-v`    | Show version information                                           |

Tip: You can redirect output to a file with `>` if needed.

---

## 🙌 Contributing

Contributions for improvements or feature expansions are very welcome!  
Feel free to open an issue or PR if you have any feedback or suggestions —  
no matter how small, we'd love to hear from you.

---

## 🔮 Planned Features

1. Wildcard support (`*`, `?`) for more flexible pattern matching

---

## ✅ License

This project is dual-licensed under:

- [MIT License](LICENSE-MIT)  
  https://opensource.org/licenses/MIT  
- [Apache License 2.0](LICENSE-APACHE)  
  https://www.apache.org/licenses/LICENSE-2.0  

Use whichever license suits your needs.
