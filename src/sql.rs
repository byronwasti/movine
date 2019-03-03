pub const LOG_UP_MIGRATION: &'static str = "\
INSERT INTO movine_migrations (name, up_hash, down_hash, down_sql)
VALUES ($1, $2, $3, $4);
";

pub const LOG_DOWN_MIGRATION: &'static str = "\
DELETE FROM movine_migrations 
WHERE name = $1;
";

pub const INIT_UP_SQL: &'static str = "\
CREATE TABLE movine_migrations (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP DEFAULT now(),
    updated_at TIMESTAMP DEFAULT now(),
    name TEXT NOT NULL,
    up_hash TEXT NOT NULL,
    down_hash TEXT,
    down_sql TEXT
);
";

pub const INIT_DOWN_SQL: &'static str = "\
DROP TABLE movine_migrations;
";
