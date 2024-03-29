# Easy Tracker

![GitHub Repo Size](https://img.shields.io/github/repo-size/mdeslippe/easy-tracker?style=for-the-badge)
![GitHub Server Workflow Status](https://img.shields.io/github/actions/workflow/status/mdeslippe/easy-tracker/server-build.yml?label=Server%20Build&style=for-the-badge)
![GitHub Website Workflow Status](https://img.shields.io/github/actions/workflow/status/mdeslippe/easy-tracker/website-build.yml?label=Website%20Build&style=for-the-badge)

Easy Tracker is a web-based utility that enables users to easily monitor the current and historical status of their digital services. In an ever-increasingly digital world, detecting and mitigating service disruptions has become more important than ever before; with Easy Tracker, you can be confident that your digital services are fully operational.

## Project Overview

### Migrations

The [migrations](https://github.com/mdeslippe/easy-tracker/tree/main/migrations) directory contains the migrations for the Easy Tracker MySQL database. All migrations are managed using the [sqlx-cli](https://crates.io/crates/sqlx-cli), a command-line utility for managing migrations with [sqlx](https://github.com/launchbadge/sqlx).

### Server

The [server](https://github.com/mdeslippe/easy-tracker/tree/main/server) directory contains the source code for the public-facing Easy Tracker server; a high-performance web server written in the [Rust](https://www.rust-lang.org/) programming language with the [Actix Web](https://actix.rs/) framework.

### Website

The [website](https://github.com/mdeslippe/easy-tracker/tree/main/website) directory contains the source code for the public-facing Easy Tracker website; a website written in the [TypeScript](https://www.typescriptlang.org/) programming language with the [React](https://react.dev/) library.

## Contributions

Pull requests are welcome, please open an issue discussing what you would like to change beforehand.
