use std::fs;
use std::fs::{File};
use std::path::{Path};
use std::io::{Result, ErrorKind, Write};
use chrono::prelude::*;

pub struct LocalMigrations {
    migrations: Vec<String>,
}

impl LocalMigrations {
    pub fn new() -> Self {
        Self {
            migrations: Vec::new(),
        }
    }

    pub fn load_path_migrations(&self) {
        match self.load_path_migrations_inner() {
            Ok(_) => return,
            Err(ref e) if e.kind() == ErrorKind::NotFound => {
                panic!("Create a migration path {}", e)
            }
            Err(e) => panic!("{}", e),
        }
    }

    fn load_path_migrations_inner(&self) -> Result<()> {
        let paths = fs::read_dir("./migrations/")?;

        for path in paths {
            println!("Name: {}", path.unwrap().path().display())
        }

        Ok(())
    }

    pub fn init(&self) -> Result<()> {
        self.create_base_directory()?;
        self.create_initial_migration_folder()?;
        Ok(())
    }

    fn create_base_directory(&self) -> Result<()> {
        let exists = Path::new("./migrations/").exists();
        if !exists {
            fs::create_dir("./migrations/")?;
        }
        Ok(())
    }

    fn create_initial_migration_folder(&self) -> Result<()> {
        let time = Utc.timestamp(0, 0);
        let time = time.format("%Y-%m-%d-%H%M%S").to_string();
        let folder = format!("./migrations/{time}_movine_init", time=time);
        let exists = Path::new(&folder).exists();
        if !exists {
            fs::create_dir(&folder)?;
            let mut up = File::create(format!("{}/up.sql", &folder))?;
            up.write_all(INIT_UP_SQL.as_bytes());
            let mut down = File::create(format!("{}/down.sql", &folder))?;
            down.write_all(INIT_DOWN_SQL.as_bytes());
        }
        Ok(())
    }
}

pub const INIT_UP_SQL: &'static str = "\
CREATE TABLE movine_meta (
    created_at TIMESTAMP DEFAULT now()
);

CREATE TABLE movine_migrations (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    name TEXT NOT NULL,
    up_hash TEXT NOT NULL,
    down_hash TEXT NOT NULL,
    down_sql TEXT
);
";

const INIT_DOWN_SQL: &'static str = "\
DROP TABLE movine_meta;
DROP TABLE movine_migrations;
";
