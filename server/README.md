# The Easy Tracker Server

This directory contains the source code for the public-facing Easy Tracker server; a high-performance web server written in the [Rust](https://www.rust-lang.org/) programming language.

## Installation Guide

Before installing, make sure you have the latest stable version of [Rust](https://www.rust-lang.org/) and [OpenSSL version 1.1.1](https://www.openssl.org/news/openssl-1.1.1-notes.html) installed on your system. You also need to make sure the `OPENSSL_DIR` environment variable contains the path to your local OpenSSL installation; this will be used to locate the OpenSSL installation during the server's compilation.

1. If you have not already done so, clone the repository and navigate to the server directory.
2. Configure the config.json configuration file in the `server` directory.
3. To start a local development server, run the command `cargo run`.
4. To create a production release build, run the command `cargo build --release`.

When starting the server, you may optionally specify a path to the server's configuration file. If not path is specified, the server will attempt to locate the configuration file in the present working directory.

The repository contains a self-signed SSL/TLS certificate for **DEVELOPMENT PURPOSES ONLY**. It is advised that you **DO NOT** use this certificate in production.

## Dependencies

**actix-web** - A powerful, pragmatic, and extremely fast web framework for Rust.
<br />
**env_logger** - A logging implementation for the `log` crate.
<br />
**log** - A lightweight logging library for Rust.
<br />
**no-panic** - A macro that requires the compiler to prove a function cannot panic.
<br />
**openssl** - OpenSSL bindings for Rust.
<br />
**serde** - A framework for serializing and deserializing Rust data structures.
<br />
**serde_json** - A JSON extension for `serde`.
