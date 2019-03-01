use crate::db::{DBExecutor};
use crate::local;
use crate::local::{LocalMigrations};

pub struct StateMachine {
    db_exec: DBExecutor,
    local: LocalMigrations,
}

impl StateMachine {
    pub fn new(db_exec: DBExecutor, local: LocalMigrations) -> Self {
        Self {
            db_exec,
            local,
        }
    }

    pub fn init(&mut self) {
        self.local.init().unwrap();
        self.db_exec.run_sql(local::INIT_UP_SQL).unwrap();
    }
}
