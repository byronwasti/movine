# Movine
![Linux build status](https://github.com/byronwasti/movine/workflows/CI/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/movine.svg)](https://crates.io/crates/movine)

Movine is a simple database migration manager that aims to be compatible with real-world migration work. Many migration managers get confused with complicated development strategies for migrations. Oftentimes migration managers do not warn you if the SQL saved in git differs from what was actually run on the database. Movine solves this issue by keeping track of the unique hashes for the `up.sql` and `down.sql` for each migration, and provides tools for fixing issues. This allows users to easily keep track of whether their local migration history matches the one on the database.

This project is currently in early stages.

Movine does *not* aim to be an ORM. Consider [diesel](http://diesel.rs/) instead if you want an ORM.

## Migration Concepts

Movine keeps track of four different states of migrations on the database. There are the basic ones:

- Applied: Found locally and applied to the database
- Pending: Found locally and not applied to the database

Then there are the more complicated ones, which Movine was specifically designed to handle:

- Variant: Found locally but a different version is applied to the database
- Divergent: Not found locally but applied to the database

## Short Asciinema Demo

A 3.5 minute video showcasing the various tools Movine provides.

[![asciicast](https://asciinema.org/a/337321.svg)](https://asciinema.org/a/337321)

## Configuration

The first step to get started with Movine is to set the configuration. Configuration can be supplied either through a `movile.toml` file or environment variables:

### Using a Config File
If Movine finds a config file named `movine.toml` it will use the parameters specified.

```toml
[postgres]
host = {host}
database = {db}
user = {username}
password = {pass}
port = {port}
sslrootcert = {cert filename}

## Or use the Sqlite adaptor
[sqlite]
file={file}

## Or supply a database URL
database_url={url_string}
```

*Note: SSLRootCert currently does not work when supplying a database_url.*
*Note: You should only specify connection details for one database type, or Movine will implicitly choose one*

### Environment variables

You can configure the PostgreSQL adaptor using the environment variables described in the [PostgreSQL documentation](https://www.postgresql.org/docs/current/libpq-envars.html). Specifically `PGHOST`, `PGPORT`, `PGDATABASE`, `PGUSER`, and `PGPASSWORD` and `PGSSLROOTCERT` are supported.

You can configure the SQLite adaptor using an `SQLITE_FILE` environment variable.

Finally, you can also supply a `DATABASE_URL` environment variable.

*Note: SSLRootCert does not work when using a database URL.*

Movine supports [`.env`](https://github.com/dotenv-rs/dotenv#usage) files as a source of configuration.

## Initializing

Next, you can run the `init` command to set everything up, the `generate` command to create your first migration, and once those are written you can run `up` to apply them.
```
$ movine init
$ tree migrations/
migrations/
└── 1970-01-01-000000_movine_init
    ├── down.sql
    └── up.sql

1 directory, 2 files
$ movine generate create_new_table
$ tree migrations/
migrations/
├── 1970-01-01-000000_movine_init
│   ├── down.sql
│   └── up.sql
└── 2019-03-17-163451_create_new_table
    ├── down.sql
    └── up.sql

2 directories, 4 files
$ movine up
$ movine status
2019-03-17 16:34:51 UTC - Applied   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
```

## Commands
There are a few commands that Movine uses, and all of them can be listed by using `--help` on the command line.

### Init

The `init` command will run the initialization routine for Movine, which will create a table on the database to keep track of migrations and create a local migrations folder.
```
$ movine init
$ ls
migrations/   movine.toml
$ tree migrations/
migrations/
└── 1970-01-01-000000_movine_init
    ├── down.sql
    └── up.sql

1 directory, 2 files
$ psql $PARAMS -c "\d"
                   List of relations
 Schema |           Name           |   Type   | Owner
--------+--------------------------+----------+--------
 public | movine_migrations        | table    | movine
 public | movine_migrations_id_seq | sequence | movine
```

### Generate

The `generate` command will generate a folder with the current date and the given name in the `migrations/` directory with blank `up.sql` and `down.sql` files.
```
$ movine generate create_new_table
$ tree migrations/
migrations/
├── 1970-01-01-000000_movine_init
│   ├── down.sql
│   └── up.sql
└── 2019-03-17-163451_create_new_table
    ├── down.sql
    └── up.sql

2 directories, 4 files
```

### Status

The `status` command will tell you the current state of all migrations, both local and on the database.

```
$ movine status
2019-03-17 16:34:51 UTC - Pending   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
```
### Up

The `up` command will run all pending migrations. You can also run with the `-p` flag to show the migration plan without running it. This is true for all commands that modify the database and is useful for seeing if Movine will do what you expect.

```
$ movine up -p
1. Up   - 2019-03-17-163451_create_new_table
$ movine status
2019-03-17 16:34:51 UTC - Pending   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
$ movine up
$ movine status
2019-03-17 16:34:51 UTC - Applied   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
```

### Down

The `down` command will rollback the most recent migration.
```
$ movine down
$ movine status
2019-03-17 16:34:51 UTC - Pending   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
```

### Redo

The `redo` command will rollback and then re-apply the most recent applied migration or variant migration.
_Note: If the latest migration is `divergent` then redo will simply skip it. Be careful, and run `fix` if you want to fix `divergent` migrations._
```
$ movine status
2019-03-17 16:34:51 UTC - Variant   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
$ movine redo
$ movine status
2019-03-17 16:34:51 UTC - Applied   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
```

### Fix

The `fix` command will rollback everything until there are no divergent or variant migrations, and then apply all migrations _except_ the migrations that were pending at the start.
```
$ movine status
2019-03-17 16:41:07 UTC - Pending   2019-03-17-164107_create_another_table
2019-03-17 16:40:59 UTC - Divergent 2019-03-17-164059_modify_table
2019-03-17 16:34:51 UTC - Variant   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
$ movine fix
$ movine status
2019-03-17 16:41:07 UTC - Pending   2019-03-17-164107_create_another_table
2019-03-17 16:34:51 UTC - Applied   2019-03-17-163451_create_new_table
1970-01-01 00:00:00 UTC - Applied   1970-01-01-000000_movine_init
```

### Custom

The `custom` command will allow you to specify your own migration strategy (in case Movine is not smart enough). *Note: this is currently not implemented*

## Library Usage
*Note: While the `Movine` implementation is stable at this point, the `config` API may be in flux (specifically the helper functions). Please let me know any feedback!*

Movine can be used as a library like so (using helper functions to load the database connection):
```rust
use movine::{Movine, Config};
use movine::errors::Error;

fn main() -> Result<(), Error> {
    let config = Config::load(&"movine.toml")?;
    let mut conn = config.into_sqlite_conn();
    let mut movine = Movine::new(&mut conn);
    movine.up()?;
    Ok(())
}
```

Or if you already have a connection:
```rust
use movine::{Movine, Config};
use movine::errors::Error;

fn main() -> Result<(), Error> {
    // Same concept with a postgres connection!
    let mut conn = rusqlite::Connection::open("file.db")?;
    let mut movine = Movine::new(&mut conn);
    movine.up()?;
    Ok(())
}
```

## Why you should use Movine

- You accept the risks of pre-1.0 software
- You want to write raw sql for your migrations
- You have a shared database that has migrations developed by multiple developers
- You want a migration management solution that works for the developers 

## Why you should not use Movine

- You want long battle-tested database migration manager
- You want ORM integration (consider [diesel](http://diesel.rs/) instead)
- You don't see value in keeping track of variant or divergent migrations

