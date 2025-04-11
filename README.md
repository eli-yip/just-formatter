# Justfile Formatter

`just-formatter` is a simple wrapper for `just`. Since `just --dump` does not support reading files from standard input, unexpected issues may arise when using it as a formatter for editors like Helix Editor.

## Installation

Run the following command:

```bash
cargo install just-formatter
```

## Usage

Ensure that `just` is installed.

### Command Line

```
cat <justfile> | just-formatter
```

### Using in Helix

Example `languages.toml`:

```toml
[[language]]
name = "just"
auto-format = true
formatter = { command = "just-formatter" }
```

### Skip `just` check to accelarate speed

Use `--skip-check`/`-s` flag. An example helix config:

```toml
[[language]]
name = "just"
auto-format = true
formatter = { command = "just-formatter", args = ["--skip-check"] }
```
