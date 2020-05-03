use crate::errors::{Error, Result};
use crate::migration::{Migration, MigrationBuilder};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;

pub struct FileHandler {
    migration_dir: PathBuf,
}

impl FileHandler {
    pub fn new(migration_dir: &str) -> Self {
        Self {
            migration_dir: migration_dir.into(),
        }
    }

    pub fn create_migration_directory(&self) -> Result<()> {
        let exists = self.migration_dir.exists();
        if !exists {
            fs::create_dir(&self.migration_dir)?;
        }
        Ok(())
    }

    pub fn write_migration(&self, migration: &Migration) -> Result<()> {
        let name = migration.name.clone().into();
        let folder: PathBuf = [&self.migration_dir, &name].iter().collect();
        fs::create_dir(&folder)?;

        let up_file: PathBuf = [&folder, &"up.sql".into()].iter().collect();
        let mut up = File::create(up_file)?;

        let down_file: PathBuf = [&folder, &"down.sql".into()].iter().collect();
        let mut down = File::create(down_file)?;

        if let Some(up_sql) = &migration.up_sql {
            up.write_all(up_sql.as_bytes())?;
        }
        if let Some(down_sql) = &migration.down_sql {
            down.write_all(down_sql.as_bytes())?;
        }
        Ok(())
    }

    pub fn load_local_migrations(&self) -> Result<Vec<Migration>> {
        let directory = match fs::read_dir(&self.migration_dir) {
            Ok(dir) => dir,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Err(Error::MigrationDirNotFound);
            }
            Err(e) => {
                return Err(e)?;
            }
        };
        let mut migrations = Vec::new();

        for entry in directory {
            let entry = entry?;
            let compound_name: String = entry.file_name().into_string().unwrap();

            let mut up_path = entry.path();
            let mut down_path = entry.path();
            up_path.push("up.sql");
            down_path.push("down.sql");

            let mut file = File::open(up_path)?;
            let mut up_sql = String::new();
            file.read_to_string(&mut up_sql)?;

            let mut file = File::open(down_path)?;
            let mut down_sql = String::new();
            file.read_to_string(&mut down_sql)?;

            let migration = MigrationBuilder::new()
                .compound_name(&compound_name)
                .up_sql(&up_sql)
                .down_sql(&down_sql)
                .build()?;
            migrations.push(migration);
        }

        Ok(migrations)
    }
}
