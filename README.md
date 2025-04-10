# Brainfuck CLI

A simple and educational Brainfuck interpreter written in Rust.  
Supports both **file execution** and an **interactive REPL** mode.

---

## ✨ Features

- Fully functional Brainfuck interpreter
- REPL mode with built-in commands
- File execution via CLI
- Clean and minimal interface
- Written 100% in Rust 🦀

---

## 🚀 Installation

### With [Cargo](https://www.rust-lang.org/tools/install):

```bash
cargo install brainfuck-cli
```

This will install the `brainfuck` command globally.

### 🧰 Manual Installation (No Cargo)

If you don't have Rust installed or prefer not to use `cargo`, you can download a precompiled binary from the [Releases](https://github.com/pedroafabri/brainfuck-cli/releases) page.

#### Linux/macOS:

```bash
wget https://github.com/pedroafabri/brainfuck-cli/releases/download/<version>/brainfuck
chmod +x brainfuck
sudo mv brainfuck /usr/local/bin/
```

> Remember to change the version accordingly.

#### Windows:

1. Download `brainfuck.exe` from the [Releases](https://github.com/pedroafabri/brainfuck-cli/releases) page.
2. Move it to a folder in your `PATH` (e.g. `C:\Windows\System32` or `C:\Program Files\brainfuck`).
3. Optionally, add the folder to your `PATH` environment variable if not already included.

Once installed, you can run `brainfuck` from your terminal or command prompt.


```bash
wget https://github.com/pedroafabri/brainfuck-cli/releases/download/vX.Y.Z/brainfuck
chmod +x brainfuck
sudo mv brainfuck /usr/local/bin/


---

## 🧠 Usage

### Execute a Brainfuck file:

```bash
brainfuck path/to/program.bf
```

### Start REPL mode:

```bash
brainfuck
```

---

## 💬 REPL Commands

Once inside the REPL, the following commands are available:

```
.help   -> Display this message
.reset  -> Reset the current Brainfuck interpreter to its initial state
.exit   -> Exit this REPL session
```

You can also type and run Brainfuck code interactively:

```bf
+++++++[>++++++++<-]>.
```

---

## 📄 Example

```bash
$ brainfuck hello.bf
Hello, World!
```

```bash
$ brainfuck
Brainfuck Interpreter - v0.1.0
Type '.help' for available commands.
> +++++++[>++++++++<-]>.
A
```

---

## 🛠️ Project Status

This project is a personal learning project and a tribute to minimalism.  
Contributions and suggestions are welcome!

---

## 📜 License

MIT
