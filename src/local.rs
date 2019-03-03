use crate::migration::LocalMigration;
use crate::sql::{INIT_DOWN_SQL, INIT_UP_SQL};
use chrono::prelude::*;
use std::fs;
use std::fs::File;
use std::io::{ErrorKind, Read, Result, Write};
use std::path::Path;

pub struct LocalMigrations {}

impl LocalMigrations {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load_migrations(&self) -> Result<Vec<LocalMigration>> {
        let directory = fs::read_dir("./migrations/")?;
        let mut migrations = Vec::new();

        for entry in directory {
            let entry = entry?;
            let name: String = entry.file_name().into_string().unwrap();
            let mut up_path = entry.path();
            let mut down_path = entry.path();
            up_path.push("up.sql");
            down_path.push("down.sql");

            let mut file = File::open(up_path)?;
            let mut up_contents = String::new();
            file.read_to_string(&mut up_contents)?;

            let mut file = File::open(down_path)?;
            let mut down_contents = String::new();
            file.read_to_string(&mut down_contents)?;

            let mut migration = LocalMigration::new(&name);
            migration.set_up_sql(&up_contents);
            migration.set_down_sql(&down_contents);

            migrations.push(migration);
        }

        Ok(migrations)
    }

    pub fn init(&self) -> Result<()> {
        self.create_base_directory()?;
        let name = self.create_initial_migration_folder()?;
        Ok(())
    }

    fn create_base_directory(&self) -> Result<()> {
        let exists = Path::new("./migrations/").exists();
        if !exists {
            fs::create_dir("./migrations/")?;
        }
        Ok(())
    }

    fn create_initial_migration_folder(&self) -> Result<String> {
        let time = Utc.timestamp(0, 0);
        let time = time.format("%Y-%m-%d-%H%M%S").to_string();
        let name = format!("{}_movine_init", time);
        let folder = format!("./migrations/{}", &name);
        let exists = Path::new(&folder).exists();
        if !exists {
            fs::create_dir(&folder)?;
            let mut up = File::create(format!("{}/up.sql", &folder))?;
            up.write_all(INIT_UP_SQL.as_bytes()).unwrap();
            let mut down = File::create(format!("{}/down.sql", &folder))?;
            down.write_all(INIT_DOWN_SQL.as_bytes()).unwrap();
        }
        Ok(name)
    }

    pub fn create_new_migration(&self, name: &str) -> Result<()> {
        let time = Utc::now();
        let time = time.format("%Y-%m-%d-%H%M%S").to_string();
        let name = format!("{}_{}", time, name);
        let folder = format!("./migrations/{}", name);
        fs::create_dir(&folder)?;
        let _ = File::create(format!("{}/up.sql", &folder))?;
        let _ = File::create(format!("{}/down.sql", &folder))?;

        Ok(())
    }
}
