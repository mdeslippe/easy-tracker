# The Easy Tracker Server

This directory contains the source code for the public-facing Easy Tracker server; a high-performance web server written in the [Rust](https://www.rust-lang.org/) programming language with the [Actix Web](https://actix.rs/) framework.

## Installation Guide

Before installing, make sure you have the latest stable version of [Rust](https://www.rust-lang.org/) and [OpenSSL](https://www.openssl-library.org/) installed on your system. You also need to make sure the `OPENSSL_DIR` environment variable contains the path to your local OpenSSL installation; this will be used to locate the OpenSSL installation during the server's compilation.

1. If you have not already done so, clone the repository and navigate to the server directory.
2. Configure the `config.json` configuration file in the `server` directory.
3. Add your database connection string to the `.env` file in the `server` directory. This configuration value will only be used to perform compile-time query validation; when the server is running, it will use the connection string in the `config.json` configuration file.
4. To start a local development server, run the command `cargo run`.
5. To create a production release build, run the command `cargo build --release`.

When starting the server, you may optionally specify a path to the server's configuration file. If not path is specified, the server will attempt to locate the configuration file in the present working directory.

The repository contains a self-signed SSL/TLS certificate for **DEVELOPMENT PURPOSES ONLY**. It is advised that you **DO NOT** use this certificate in production.

## Password Hashing

The [Argon2id v19](https://en.wikipedia.org/wiki/Argon2) password hashing algorithm is used to hash user passwords.

## User Authentication

[JSON Web Tokens (JWT)](https://www.rfc-editor.org/rfc/rfc7519) are used for request authentication after users have logged in. As stated in the RFC, JSON Web Token (JWT) is a compact, URL-safe means of representing claims to be transferred between two parties. The claims in a JWT are encoded as a JSON object that is used as the payload of a JSON Web Signature (JWS) structure or as the plaintext of a JSON Web Encryption (JWE) structure, enabling the claims to be digitally signed or integrity protected with a Message Authentication Code (MAC) and/or encrypted.

The server is configured to use the RS256 algorithm to sign and verify tokens. RS256 is an asymmetric algorithm that uses a private key to sign tokens, and a public key to verify the signature.

## Testing

The standard cargo testing utility is used for testing.

All tests should be written in a new file called `test.rs`. The new file must be declared in the module of the code it is testing, if it is not, the `cargo test` command will not detect it. Make sure to give test modules the `#[cfg(test)]` attribute; this will allow the module to be picked up by the `cargo test` command, and ensures the test cases will not be compiled into the binary when building the project.

It is recommended that test cases are ran on a single thread, this can be achieved by using the following command: `cargo test -- --test-threads 1`. If multiple threads are used, deadlocks may occur in the database; deadlocks are a normal thing to occur and actual code should check for them, but tests do not need to.

## Dependencies

**actix-cors** - Cross-Origin Resource Sharing (CORS) controls for `actix-web`.
<br />
**actix-web** - A powerful, pragmatic, and extremely fast web framework for Rust.
<br />
**argon2** - A pure Rust implementation of the Argon2 password hashing function with support for the Argon2d, Argon2i, and Argon2id algorithmic variants.
<br />
**async-trait** - Type erasure for async trait methods.
<br />
**env_logger** - A logging implementation for the `log` crate.
<br />
**jsonwebtoken** - A library for JWT encoding an decoding.
<br />
**log** - A lightweight logging library for Rust.
<br />
**nameof** - A macro to determine the string name of a binding, type, const, or function.
<br />
**no-panic** - A macro that requires the compiler to prove a function cannot panic.
<br />
**openssl** - OpenSSL bindings for Rust.
<br />
**rand** - A utility to generate random numbers.
<br />
**regex** - An implementation of regular expressions for Rust.
<br />
**serde** - A framework for serializing and deserializing Rust data structures.
<br />
**serde_json** - A JSON extension for `serde`.
<br />
**shaku** - A compile time dependency injection framework for Rust.
<br />
**shaku_actix** - An integration between `shaku` and `actix-web`.
<br />
**sqlx** - A pure Rust SQL crate featuring compile-time checked queries without a DSL.
<br />
**time** - A date and time library.
<br />
**validator** - Macros to simplify struct validation.
