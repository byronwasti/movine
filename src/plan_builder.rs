use crate::errors::{Error, Result};
use crate::match_maker::{self, Matching};
use crate::migration::Migration;

pub type Plan<'a> = Vec<(Step, &'a Migration)>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Step {
    Up,
    Down,
}

pub struct PlanBuilder<'a> {
    local_migrations: Option<&'a [Migration]>,
    db_migrations: Option<&'a [Migration]>,
    count: Option<usize>,
    strict: bool,
    ignore_divergent: bool,
}

impl<'a> PlanBuilder<'a> {
    pub fn new() -> Self {
        Self {
            local_migrations: None,
            db_migrations: None,
            count: None,
            strict: false,
            ignore_divergent: false,
        }
    }

    pub fn local_migrations(mut self, m: &'a [Migration]) -> Self {
        self.local_migrations = Some(m);
        self
    }

    pub fn db_migrations(mut self, m: &'a [Migration]) -> Self {
        self.db_migrations = Some(m);
        self
    }

    pub fn count(mut self, count: Option<usize>) -> Self {
        self.count = count;
        self
    }

    pub fn set_strict(mut self, strict: bool) -> Self {
        self.strict = strict;
        self
    }

    pub fn set_ignore_divergent(mut self, ignore: bool) -> Self {
        self.ignore_divergent = ignore;
        self
    }

    pub fn up(self) -> Result<Plan<'a>> {
        let mut dirty = false;
        let mut pending_found = false;
        let mut plan = Vec::new();

        let matches = self.get_matches()?;
        for m in matches {
            match m {
                Matching::Pending(x) => {
                    pending_found = true;
                    if let Some(count) = self.count {
                        if count == plan.len() {
                            continue;
                        }
                    }

                    let step = (Step::Up, x);
                    plan.push(step);
                }
                _ => {
                    if pending_found {
                        dirty = true;
                    }
                    continue;
                }
            }
        }

        if self.strict && dirty {
            return Err(Error::DirtyMigrations);
        }

        Ok(plan)
    }

    pub fn down(self) -> Result<Plan<'a>> {
        let mut plan: Plan<'a> = Vec::new();
        let matches = self.get_matches()?;

        // Note: get_matches() returns the migrations in date-order.
        // We want the most recently run, so we have to reverse the order.
        for m in matches.iter().rev() {
            match m {
                Matching::Divergent(x) => {
                    if self.ignore_divergent {
                        continue;
                    }

                    plan.push((Step::Down, x));
                }
                Matching::Applied(_) | Matching::Variant(_, _) => {
                    if m.is_reversable() {
                        plan.push((Step::Down, m.get_best_down_migration()));
                    } else {
                        return Err(Error::UnrollbackableMigration);
                    }
                }
                _ => {}
            }

            if let Some(count) = self.count {
                if count == plan.len() {
                    break;
                }
            } else if plan.len() == 1 {
                break;
            }
        }

        Ok(plan)
    }

    pub fn fix(self) -> Result<Plan<'a>> {
        let matches = self.get_matches()?;

        let mut bad_migration_found = false;
        let mut rollback_plan_rev = Vec::new();
        let mut rollup_plan = Vec::new();
        for m in matches {
            match m {
                Matching::Divergent(x) => {
                    bad_migration_found = true;
                    if m.is_reversable() {
                        rollback_plan_rev.push((Step::Down, x));
                    } else {
                        return Err(Error::UnrollbackableMigration);
                    }
                }
                Matching::Variant(_, _) => {
                    bad_migration_found = true;
                    let down = m.get_best_down_migration();
                    let up = m.get_local_migration().unwrap();
                    if m.is_reversable() {
                        rollback_plan_rev.push((Step::Down, down));
                        rollup_plan.push((Step::Up, up));
                    } else {
                        return Err(Error::UnrollbackableMigration);
                    }
                }
                Matching::Applied(x) => {
                    if bad_migration_found {
                        if m.is_reversable() {
                            rollback_plan_rev.push((Step::Down, x));
                            rollup_plan.push((Step::Up, x));
                        } else {
                            return Err(Error::UnrollbackableMigration);
                        }
                    }
                }
                Matching::Pending(x) => {
                    bad_migration_found = true;
                    rollup_plan.push((Step::Up, x));
                }
            }
        }

        let mut plan: Plan<'a> = rollback_plan_rev.drain(..).rev().collect();
        plan.append(&mut rollup_plan);
        Ok(plan)
    }

    pub fn redo(self) -> Result<Plan<'a>> {
        let matches = self.get_matches()?;
        let mut rollback_plan: Plan<'a> = Vec::new();
        let mut rollup_plan_rev: Plan<'a> = Vec::new();

        // Note: get_matches() returns the migrations in date-order.
        // We want the most recently run, so we have to reverse the order.
        for m in matches.iter().rev() {
            match m {
                Matching::Divergent(_) => {
                    if self.ignore_divergent {
                        continue;
                    }

                    return Err(Error::DivergentMigration);
                }
                Matching::Applied(_) | Matching::Variant(_, _) => {
                    if m.is_reversable() {
                        rollback_plan.push((Step::Down, m.get_best_down_migration()));
                        rollup_plan_rev.push((Step::Up, m.get_local_migration().unwrap()));
                    } else {
                        return Err(Error::UnrollbackableMigration);
                    }
                }
                _ => {}
            }

            if let Some(count) = self.count {
                if count == rollback_plan.len() {
                    break;
                }
            } else if rollback_plan.len() == 1 {
                break;
            }
        }

        let mut rollup_plan: Plan<'a> = rollup_plan_rev.drain(..).rev().collect();
        let mut plan = rollback_plan;
        plan.append(&mut rollup_plan);
        Ok(plan)
    }

    pub fn status(self) -> Result<Vec<Matching<'a>>> {
        self.get_matches()
    }

    fn get_matches(&self) -> Result<Vec<Matching<'a>>> {
        if let (Some(local_migrations), Some(db_migrations)) =
            (self.local_migrations, self.db_migrations)
        {
            let mut matches = match_maker::find_matches(local_migrations, db_migrations);
            matches.sort();
            Ok(matches)
        } else {
            Err(Error::Unknown)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::migration::Migration;

    // QoL impl
    impl Migration {
        fn new(name: &str) -> Self {
            Self {
                name: name.to_string(),
                up_sql: None,
                down_sql: Some("test".to_owned()),
                hash: None,
            }
        }

        fn new_with_hash(name: &str, hash: &str) -> Self {
            Self {
                name: name.to_string(),
                up_sql: None,
                down_sql: None,
                hash: Some(hash.to_string()),
            }
        }
    }

    #[test]
    /// Up should run pending migrations in-order.
    fn test_up_1() {
        let local = [Migration::new(&"test_1"), Migration::new(&"test_2")];
        let db = [];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .up()
            .unwrap();
        assert_eq!(plan, [(Step::Up, &local[0]), (Step::Up, &local[1])])
    }

    #[test]
    /// Up should run pending migrations even if divergent migrations exist.
    fn test_up_2() {
        let local = [Migration::new(&"test"), Migration::new(&"test_2")];
        let db = [Migration::new(&"test"), Migration::new(&"test_3")];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .up()
            .unwrap();
        assert_eq!(plan, [(Step::Up, &local[1])])
    }

    #[test]
    /// Up should error with --strict if migrations are out-of-order.
    fn test_up_3() {
        let local = [Migration::new(&"test"), Migration::new(&"test_2")];
        let db = [Migration::new(&"test"), Migration::new(&"test_3")];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .set_strict(true)
            .up();
        assert!(plan.is_err());
        let is_correct_error = match plan.err().unwrap() {
            Error::DirtyMigrations => true,
            _ => false,
        };
        assert!(is_correct_error);
    }

    #[test]
    /// Down should rollback the most recent migration (divergent included by default)
    fn test_down_1() {
        let local = [Migration::new(&"test"), Migration::new(&"test_2")];
        let db = [Migration::new(&"test"), Migration::new(&"test_3")];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .down()
            .unwrap();
        assert_eq!(plan, [(Step::Down, &db[1])])
    }

    #[test]
    /// Down should rollback the most recent migration (ignoring divergent)
    fn test_down_2() {
        let local = [Migration::new(&"test"), Migration::new(&"test_2")];
        let db = [Migration::new(&"test"), Migration::new(&"test_3")];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .set_ignore_divergent(true)
            .down()
            .unwrap();
        assert_eq!(plan, [(Step::Down, &local[0])])
    }

    #[test]
    /// Fix should rollback all variant and divergent migrations, and then run pending migrations.
    fn test_fix_1() {
        let local = [
            Migration::new(&"test_0"),
            Migration::new(&"test_1"),
            Migration::new(&"test_2"),
        ];
        let db = [
            Migration::new(&"test_0"),
            Migration::new_with_hash(&"test_1", &"hash"),
            Migration::new_with_hash(&"test_2", &"hash"),
            Migration::new(&"test_3"),
        ];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .fix()
            .unwrap();
        assert_eq!(
            plan,
            [
                (Step::Down, &db[3]),
                (Step::Down, &local[2]),
                (Step::Down, &local[1]),
                (Step::Up, &local[1]),
                (Step::Up, &local[2]),
            ]
        )
    }

    #[test]
    /// Fix should rollback applied migrations if they are ahead of variant migrations.
    fn test_fix_2() {
        let local = [
            Migration::new(&"test"),
            Migration::new(&"test_1"),
            Migration::new(&"test_2"),
        ];
        let db = [
            Migration::new(&"test"),
            Migration::new_with_hash(&"test_1", &"hash"),
            Migration::new(&"test_2"),
        ];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .fix()
            .unwrap();
        assert_eq!(
            plan,
            [
                (Step::Down, &local[2]),
                (Step::Down, &local[1]),
                (Step::Up, &local[1]),
                (Step::Up, &local[2]),
            ]
        )
    }

    #[test]
    /// Fix should rollback everything to a fully applied state and then roll back up, regardless
    /// of applied/variant/diverget migration orders.
    fn test_fix_3() {
        let local = [
            Migration::new(&"test_0"),
            Migration::new(&"test_1"),
            Migration::new(&"test_2"),
            Migration::new(&"test_3"),
            Migration::new(&"test_4"),
        ];
        let db = [
            Migration::new(&"test_0"),
            Migration::new_with_hash(&"test_1", &"hash"),
            Migration::new(&"test_2"),
            Migration::new(&"test_3b"),
            Migration::new(&"test_4"),
        ];
        let actual = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .fix()
            .unwrap();
        let expected = [
            (Step::Down, &local[4]),
            (Step::Down, &db[3]),
            (Step::Down, &local[2]),
            (Step::Down, &local[1]),
            (Step::Up, &local[1]),
            (Step::Up, &local[2]),
            (Step::Up, &local[3]),
            (Step::Up, &local[4]),
        ];
        assert_eq!(actual, expected)
    }

    #[test]
    /// Fix should run pending migrations without problems.
    fn test_fix_4() {
        let local = [Migration::new(&"test_0"), Migration::new(&"test_1")];
        let db = [Migration::new(&"test_0")];
        let actual = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .fix()
            .unwrap();
        let expected = [(Step::Up, &local[1])];
        assert_eq!(actual, expected)
    }

    #[test]
    /// Redo should fail if there is a divergent migration (and we are not ignoring them)
    fn test_redo_1() {
        let local = [Migration::new(&"test"), Migration::new(&"test_2")];
        let db = [
            Migration::new(&"test"),
            Migration::new_with_hash(&"test_2", &"hash_1"),
            Migration::new(&"test_3"),
        ];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .count(Some(2))
            .redo();
        assert!(plan.is_err());
        let is_correct_err = match plan.err().unwrap() {
            Error::DivergentMigration => true,
            _ => false,
        };
        assert!(is_correct_err);
    }

    #[test]
    /// Redo should properly ignore divergent migrations
    fn test_redo_2() {
        let local = [
            Migration::new(&"test_0"),
            Migration::new(&"test_1"),
            Migration::new(&"test_2"),
        ];
        let db = [
            Migration::new(&"test_0"),
            Migration::new(&"test_1"),
            Migration::new(&"test_2"),
            Migration::new(&"test_3"),
        ];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .count(Some(2))
            .set_ignore_divergent(true)
            .redo()
            .unwrap();
        assert_eq!(
            plan,
            [
                (Step::Down, &local[2]),
                (Step::Down, &local[1]),
                (Step::Up, &local[1]),
                (Step::Up, &local[2]),
            ]
        )
    }

    #[test]
    /// Redo should not care about variant migrations further than what we are redo'ing
    fn test_redo_3() {
        let local = [
            Migration::new(&"test_0"),
            Migration::new(&"test_1"),
            Migration::new(&"test_2"),
        ];
        let db = [
            Migration::new(&"test_0"),
            Migration::new_with_hash(&"test_1", &"hash_1"),
            Migration::new(&"test_2"),
        ];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .count(Some(1))
            .redo()
            .unwrap();
        assert_eq!(plan, [(Step::Down, &local[2]), (Step::Up, &local[2]),])
    }

    #[test]
    /// Redo should properly rollback variant migrations
    fn test_redo_4() {
        let local = [
            Migration::new(&"test_0"),
            Migration::new(&"test_1"),
            Migration::new(&"test_2"),
        ];
        let db = [
            Migration::new(&"test_0"),
            Migration::new_with_hash(&"test_1", &"hash_1"),
            Migration::new(&"test_2"),
        ];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .count(Some(2))
            .redo()
            .unwrap();
        assert_eq!(
            plan,
            [
                (Step::Down, &local[2]),
                (Step::Down, &local[1]),
                (Step::Up, &local[1]),
                (Step::Up, &local[2]),
            ]
        )
    }
}
