# Miniproject 7

Ids 706, miniproject 7 in Rust by Aryan Poonacha for Fall '23. Includes a package system that can be installed and used. Below is a user guide on how to install and use the tool:

## User Guide

This is a basic environment that comes with the following software choices preinstalled:

### System Tools

- [curl/curl](https://github.com/curl/curl): the command line tool for transferring data over a metric boatload of protocols.
- git: the Git SCM tool.
- [gnupg2](https://gnupg.org/): a complete and free implementatiuon of the OpenPGP standard.
- [stedolan/jq](https://github.com/stedolan/jq) - a command line JSON parser.
- [sudo](https://www.sudo.ws/) - the superuser authority delegation tool.
- [zsh](https://www.zsh.org/) - interactive terminal (alternative to `bash`).
- [ohmyzsh/ohmyzsh](https://github.com/ohmyzsh/ohmyzsh) - a delightful community driven framework for managing zsh config.
- [vim](https://www.vim.org/) - a text editor
- build essentials - tools for compiling and linking code
- [openssl](https://www.openssl.org/) - tls and ssl toolkit

### Rust Tools

Besides Rust and Cargo, the image comes with the following Rust related tooling:

- [rustup](https://rustup.rs/): installer and toolchain manager
- [rustfmt](https://github.com/rust-lang/rustfmt): a tool for formatting Rust code according to style guidelines
- [clippy](https://github.com/rust-lang/rust-clippy): lints to catch common mistakes and improve your Rust code

### VS Code Extensions

- [Rust Analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer): an alternative rust language server to the RLS.
- [CodeLLDB](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb): native debugger based on LLDB.
- [Crates](https://marketplace.visualstudio.com/items?itemName=serayuzgur.crates): helps Rust developers managing dependencies with Cargo.toml.
- [Live Share](https://marketplace.visualstudio.com/items?itemName=ms-vsliveshare.vsliveshare): collaborative, multi-user remote editing from directly within the editor.

### Operating System

- [Ubuntu 18.04](https://releases.ubuntu.com/18.04.4/): The 18.04 LTS version of Ubuntu.

## Tool Usage And Documentation

### Pattern Generator

This is a command-line tool written in Rust that generates cool and interesting patterns. It currently supports two types of patterns: squares and triangles.

#### Installation
Before you can use this tool, you need to install Rust. You can do this by following the instructions on the official Rust website.

Once you have Rust installed, you can build and install the pattern generator tool by cloning this repository and running the following command in the root directory of the project:

```
cargo build --release
```

This will create an optimized executable in the target/release directory.

Usage
You can run the pattern generator tool using the cargo run command followed by the pattern type and size. For example, to generate a square pattern of size 5, you would use:

```
cargo run square 5
```

And to generate a triangle pattern of size 7, you would use:

```
cargo run triangle 7
```

The pattern type can be either square or triangle, and the size must be a positive integer.

Please replace cargo run with the actual command to run your Rust application if it’s different.

The CI/CD included in the deploy.yml file should automatically build and run an example of this.