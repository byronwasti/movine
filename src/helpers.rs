use crate::errors::{Result};
use crate::migration::{Migration, MigrationBuilder};
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;

pub fn create_migration_directory() -> Result<()> {
    let exists = Path::new("./migrations/").exists();
    if !exists {
        fs::create_dir("./migrations/")?;
    }
    Ok(())
}

pub fn write_migration(migration: &Migration) -> Result<()> {
    let name = &migration.name;
    let folder = format!("./migrations/{}", name);
    fs::create_dir(&folder)?;
    let mut up = File::create(format!("{}/up.sql", &folder))?;
    let mut down = File::create(format!("{}/down.sql", &folder))?;

    if let Some(up_sql) = &migration.up_sql {
        up.write_all(up_sql.as_bytes())?;
    }
    if let Some(down_sql) = &migration.down_sql {
        down.write_all(down_sql.as_bytes())?;
    }
    Ok(())
}

pub fn load_local_migrations() -> Result<Vec<Migration>> {
    let directory = fs::read_dir("./migrations/")?;
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
