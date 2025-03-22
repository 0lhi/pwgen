`pwgen` is a Password Generator for the Command Line.

It was developed as a learning exercise **to explore the capabilities and limitations of AI-assisted coding.**

See `GeminiExperimental1206.md`.

## Features

* Generates random passwords with customizable length and optional symbols.
* Overwrites the password in memory after use to prevent data remanence.
* Copies the password to the clipboard for convenience.
* Automatically clears the clipboard after 15 seconds.
* Defaults to a 50-character password with symbols if no options are provided.

## Usage

The program accepts a single optional CLI argument: `--ask`

It prompts the user for password length and symbol inclusion. If this flag is not provided, the program uses the default settings (length 50, with symbols).

## Building

To build the project, you'll need Rust and Cargo installed. Then, run:

```bash
cargo build --release
```

The executable will be located in `target/release/pwgen`.

## License

Apache License 2.0.
