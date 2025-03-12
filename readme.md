# 🐱 Cat - A Recreation of the Linux `cat` Command but in Rust

Recreating the `cat` command in **Linux** but with I was too lazy so I just copy pasted this read me from the previous one.

---

## 🚀 Features
- 📖 **Read files** and display their content in the terminal.
- 🔢 **Line numbering** with `-n` flag.
- 📝 **Write output to a file** with the `-w` flag.
- 🔄 **Reverse output to a file** with the `-r` flag.
- 💲 **Display a $ at the end of each line** with the `-e` flag.
- ❌ **Ignore blank lines** with the `-s` flag.
- 🙏 **Display help text** with `--help` flag.
- ௷ Use `>` to redirect the following files into one file
- 🛠️ Built with **C++**, **CMake**, and **GoogleTest**.

---

## 🛠️ How to Build
Get started by cloning and using cargo run

```sh
# 1️⃣ Clone the repository
git clone 
cd Cat-but-in-rust

```
---

## How to use
```sh
cargo run -- [-any flags] files
```
---

#### Redirect files
```sh
cargo run -- [-any flags] files > file_you_want_to_redirect_into
```
---
