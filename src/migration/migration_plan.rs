use crate::migration::Migration;

#[derive(Debug)]
pub enum MigrationOp {
    Up,
    Down,
}

pub type MigrationPlan = Vec<(MigrationOp, Migration)>;
