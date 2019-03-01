mod config;
mod cli;
mod db;
mod local;
mod state_machine;
mod migration;

use state_machine::{StateMachine};
use db::{DBExecutor};
use local::{LocalMigrations};

pub fn run() {
    let args = cli::load_params();
    let config = config::load();

    let db_exec = DBExecutor::new(config.connection);
    let local = LocalMigrations::new();
    let mut state_machine = StateMachine::new(db_exec, local);
    state_machine.init();
}
