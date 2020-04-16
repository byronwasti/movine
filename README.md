# Movine
![Linux build status](https://github.com/byronwasti/movine/workflows/Rust/badge.svg)
[![Crates.io](https://img.shields.io/crates/v/movine.svg)](https://crates.io/crates/movine)

Movine is a simple database migration manager that aims to be compatible with real-world migration work. Many migration managers get confused with complicated development strategies for the migrations. Movine attempts to solve this issue by keeping track of the unique hashes for the `up.sql` and `down.sql` for each migration. This allows users to easily keep track of whether their local migration history matches the one on the database.

This project is currently in *very* early stages, and should be considered a proof-of-concept. The base functionality is implemented, but things should be expected to change. 

Movine does *not* aim to be an ORM. Consider [diesel](http://diesel.rs/) instead if you want an ORM.

## Migration Concepts

Movine keeps track of four different states of migrations on the database. There are the basic ones:

- Applied: Found locally and applied to the database
- Pending: Found locally and not applied to the database

Then there are the more complicated ones, which Movine was specifically designed to handle:

- Variant: Found locally but a different version is applied to the database
- Divergent: Not found locally but applied to the database

## Getting Started 
The first step to get started with Movine is to set up the `movine.toml`. This file stores the connection parameters that Movine needs in order to connect to the database. In the future this file will also hold various parameters to customize the way Movine operates.

```
# movine.toml
[postgres]
host = {host} 
database = {db} 
user = {username}
password = {pass}
port = {port}

## Or use the Sqlite adaptor
[sqlite]
file={file}
```

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

## Why you should use Movine

- You are tolerant to occasional breakage
- You want to write raw sql for your migrations
- You have a shared database that has migrations developed by multiple developers
- You are bad at rolling back a migration before editing it

## Why you should not use Movine

- You want a robust and proven database migration manager
- You want ORM integration (consider the [diesel](http://diesel.rs/) instead)
- You don't see value in keeping track of variant or divergent migrations.

