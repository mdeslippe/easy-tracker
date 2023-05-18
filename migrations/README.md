# The Easy Tracker Migrations

This directory contains the migrations for the Easy Tracker MySQL database. All migrations are managed using the [sqlx-cli](https://crates.io/crates/sqlx-cli), a command-line utility for managing migrations with [sqlx](https://github.com/launchbadge/sqlx).

## Installation Guide

Before you will be able to run any migrations, you must globally install the [sqlx-cli](https://crates.io/crates/sqlx-cli)
rust crate.

1. If you have not already done so, clone the repository and navigate to the to **base directory**, if you are not in this directory, the cli tool will not be able to find the migrations.
2. Run the command `sqlx migrate run --database-url <DATABASE_URL>`.
3. Check the database to verify it has been setup.

## Creating Migrations

All migrations have an `up` and `down` file. The `up` file performs the migration, and the `down` file reverts the migration. To create a new migration, you do the following:

1. Run the command `sqlx migrate new -r <NAME>` from the base repository directory.
2. Navigate to the `migrations` directory and look for two new files that should have been generated: `<yyyyMMddhhmmss>_<NAME>.up.sql` and `<yyyyMMddhhmmss>_<NAME>.down.sql`.
3. Implement your migration in the `up` file that was generated.
4. Implement a way to revert the migration in the `down` file that was generated.
5. Run the command `sqlx migrate run --database-url <DATABASE_URL>` to apply your migration.

## Dependencies

**sqlx-cli** - A command line utility to manage migrations.
