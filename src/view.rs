use crate::migration::{MigrationOp, MigrationPlan, MigrationStatus};
use std::io::{self, Write};
use termion::color;

pub fn display_status(migration_statuses: &[MigrationStatus]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for migration_status in migration_statuses.iter().rev() {
        writeln!(
            handle,
            "{date} - {color}{status}{reset} {name}",
            date = migration_status.get_date(),
            status = &migration_status,
            name = migration_status.get_name(),
            color = match &migration_status {
                MigrationStatus::Applied(_) => color::Fg(color::Green).to_string(),
                MigrationStatus::Pending(_) => color::Fg(color::Yellow).to_string(),
                MigrationStatus::Divergent(_) => color::Fg(color::Red).to_string(),
                MigrationStatus::Variant(_, _) => color::Fg(color::LightRed).to_string(),
            },
            reset = color::Fg(color::Reset),
        )
        .unwrap();
    }
}

pub fn display_plan(migration_plan: MigrationPlan) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    for (idx, (op, migration)) in migration_plan.iter().enumerate() {
        writeln!(
            handle,
            "{num}. {op} - {name}",
            num = idx + 1,
            op = match op {
                MigrationOp::Up => "Up  ",
                MigrationOp::Down => "Down",
            },
            name = &migration.get_name(),
        )
        .unwrap();
    }
}
