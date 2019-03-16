use crate::migration::{
    DbMigration, LocalMigration, Migration, MigrationOp, MigrationPlan, MigrationStatus,
};

#[derive(Debug, Copy, Clone)]
pub enum PlanType {
    Up(Option<usize>),
    Down(Option<usize>, bool),
    Redo(Option<usize>),
    Fix,
    Custom,
}

pub struct PlanBuilder {
    local_migrations: Option<Vec<LocalMigration>>,
    db_migrations: Option<Vec<DbMigration>>,
}

impl PlanBuilder {
    pub fn new() -> Self {
        Self {
            local_migrations: None,
            db_migrations: None,
        }
    }

    pub fn set_local_migrations<'a>(
        &'a mut self,
        mut migrations: Vec<LocalMigration>,
    ) -> &'a mut Self {
        migrations.sort_unstable_by(|a, b| a.get_date().cmp(&b.get_date()));
        self.local_migrations = Some(migrations);
        self
    }

    pub fn set_db_migrations<'a>(&'a mut self, mut migrations: Vec<DbMigration>) -> &'a mut Self {
        migrations.sort_unstable_by(|a, b| a.get_date().cmp(&b.get_date()));
        self.db_migrations = Some(migrations);
        self
    }

    pub fn with_no_db_migrations<'a>(&'a mut self) -> &'a mut Self {
        self.db_migrations = Some(Vec::new());
        self
    }

    pub fn get_status<'a>(&'a mut self) -> Vec<MigrationStatus> {
        self.combine_migrations()
    }

    pub fn build<'a>(&'a mut self, plan_type: PlanType) -> MigrationPlan {
        let combined_migrations = self.combine_migrations();
        debug!("Combined migrations: {:?}", combined_migrations);

        match plan_type {
            PlanType::Up(n) => {
                let mut count = 0;
                let mut migration_plan = MigrationPlan::new();
                for migration in combined_migrations.into_iter() {
                    if let Some(max) = n {
                        if count == max {
                            break;
                        }
                    }
                    if let MigrationStatus::Pending(migration) = migration {
                        migration_plan.push((MigrationOp::Up, migration.into()));
                        count += 1;
                    }
                }
                migration_plan
            }

            PlanType::Down(n, ignore_divergent) => {
                let mut count = 0;
                let mut migration_plan = MigrationPlan::new();
                for migration in combined_migrations.into_iter().rev() {
                    if let Some(max) = n {
                        if count == max {
                            break;
                        }
                    } else {
                        if count == 1 {
                            break;
                        }
                    }

                    match migration {
                        MigrationStatus::Applied(migration) => {
                            migration_plan.push((MigrationOp::Down, migration.into()));
                            count += 1;
                        }
                        MigrationStatus::Variant(m1, m2) => {
                            migration_plan.push((MigrationOp::Down, m1.into()));
                            count += 1;
                        }
                        MigrationStatus::Divergent(migration) => {
                            if !ignore_divergent {
                                migration_plan.push((MigrationOp::Down, migration.into()));
                                count += 1;
                            }
                        }
                        _ => (),
                    }
                }
                migration_plan
            }

            PlanType::Redo(n) => {
                let mut count = 0;
                let mut migration_plan = MigrationPlan::new();
                let mut up_migrations = Vec::new();
                for migration in combined_migrations.into_iter().rev() {
                    if let Some(max) = n {
                        if count == max {
                            break;
                        }
                    } else {
                        if count == 1 {
                            break;
                        }
                    }

                    match migration {
                        MigrationStatus::Applied(migration) => {
                            let migration: Migration = migration.into();
                            migration_plan.push((MigrationOp::Down, migration.clone()));
                            up_migrations.push(migration);
                            count += 1;
                        }
                        MigrationStatus::Variant(m1, m2) => {
                            migration_plan.push((MigrationOp::Down, m1.into()));
                            up_migrations.push(m2.into());
                            count += 1;
                        }
                        _ => (),
                    }
                }
                for migration in up_migrations.into_iter().rev() {
                    migration_plan.push((MigrationOp::Up, migration));
                }
                migration_plan
            }

            PlanType::Fix => {
                // 1. Find where we have to rollback to
                let mut count = 0;
                for (idx, migration) in combined_migrations.iter().rev().enumerate() {
                    match migration {
                        MigrationStatus::Variant(_, _) => {
                            count = idx;
                        }
                        MigrationStatus::Divergent(_) => {
                            count = idx;
                        }
                        _ => ()
                    }
                }

                // 2. Rollback to that point
                let mut migration_plan = MigrationPlan::new();
                let mut up_migrations = Vec::new();
                for (idx, migration) in combined_migrations.into_iter().rev().enumerate() {
                    debug!("Rollbacks: {}, {}", idx, &migration);
                    match migration {
                        MigrationStatus::Pending(_) => (),
                        MigrationStatus::Applied(m) => {
                            let m: Migration = m.into();
                            migration_plan.push((MigrationOp::Down, m.clone()));
                            up_migrations.push(m);
                        }
                        MigrationStatus::Variant(m1, m2) => {
                            migration_plan.push((MigrationOp::Down, m1.into()));
                            up_migrations.push(m2.into());
                        }
                        MigrationStatus::Divergent(m) => {
                            migration_plan.push((MigrationOp::Down, m.into()));
                        }
                        _ => ()
                    }

                    if idx == count {
                        break;
                    }
                }

                // 3. Run all pending migrations
                for migration in up_migrations.into_iter().rev() {
                    migration_plan.push((MigrationOp::Up, migration));
                }

                migration_plan
            }

            PlanType::Custom => MigrationPlan::new(),

            _ => MigrationPlan::new(),
        }
    }

    fn combine_migrations(&mut self) -> Vec<MigrationStatus> {
        if let (Some(local), Some(db)) = (&mut self.local_migrations, &mut self.db_migrations) {
            let mut local_iter = local.drain(..);
            let mut db_iter = db.drain(..);

            let mut local_e = local_iter.next();
            let mut db_e = db_iter.next();

            let mut statuses = Vec::new();
            loop {
                let local = local_e.take();
                let db = db_e.take();
                match (db, local) {
                    (Some(d), Some(l)) => {
                        if d.get_name() == l.get_name() {
                            let diff_up_hashes = d.get_up_hash() == l.get_up_hash();
                            let diff_down_hashes = d.get_down_hash() == l.get_down_hash();
                            if diff_up_hashes && diff_down_hashes {
                                statuses.push(MigrationStatus::Applied(l));
                            } else {
                                statuses.push(MigrationStatus::Variant(d, l));
                            }

                            db_e = db_iter.next();
                            local_e = local_iter.next();
                        } else {
                            if d.get_date() < l.get_date() {
                                statuses.push(MigrationStatus::Divergent(d));
                                db_e = db_iter.next();
                            } else {
                                statuses.push(MigrationStatus::Pending(l));
                                local_e = local_iter.next();
                            }
                        }
                    }
                    (Some(d), None) => {
                        statuses.push(MigrationStatus::Divergent(d));
                        db_e = db_iter.next();
                    }
                    (None, Some(l)) => {
                        statuses.push(MigrationStatus::Pending(l));
                        local_e = local_iter.next();
                    }
                    (None, None) => break,
                }
            }

            statuses
        } else {
            Vec::new()
        }
    }
}
