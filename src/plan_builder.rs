use crate::errors::{Error, Result};
use crate::match_maker::{self, Matching};
use crate::migration::Migration;

pub struct PlanBuilder<'a> {
    local_migrations: Option<&'a [Migration]>,
    db_migrations: Option<&'a [Migration]>,
    count: Option<usize>,
}

impl<'a> PlanBuilder<'a> {
    pub fn new() -> Self {
        Self {
            local_migrations: None,
            db_migrations: None,
            count: None,
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

    pub fn up(self) -> Result<Plan<'a>> {
        let matches = self.get_matches()?;
        let plan = matches
            .iter()
            .filter(|x| match x {
                Matching::Pending(_) => true,
                _ => false,
            })
            .map(|x| (Step::Up, x.get_local_migration().unwrap()));
        let plan = if let Some(count) = self.count {
            plan.take(count).collect()
        } else {
            plan.collect()
        };
        Ok(plan)
    }

    pub fn down(self) -> Result<Plan<'a>> {
        let matches = self.get_matches()?;
        let plan = matches
            .iter()
            .filter(|x| match x {
                Matching::Pending(_) => false,
                _ => true,
            })
            .map(|x| {
                let migration = match x {
                    Matching::Applied(x) => x,
                    Matching::Divergent(x) => x,
                    Matching::Variant(x, y) => {
                        if x.down_sql.is_some() {
                            // TODO: Have some output explaining why we default to local
                            x
                        } else {
                            y
                        }
                    }
                    _ => unreachable!(),
                };
                (Step::Down, *migration)
            })
            .rev()
            // We don't want to rollback everything by default
            .take(self.count.unwrap_or(1))
            .collect();
        Ok(plan)
    }

    pub fn fix(self) -> Result<Plan<'a>> {
        let matches = self.get_matches()?;

        let matches_to_fix: Vec<_> = matches
            .iter()
            .skip_while(|x| match x {
                Matching::Divergent(_) | Matching::Variant(_, _) => false,
                _ => true,
            })
            .collect();

        let plan_down: Vec<_> = matches_to_fix
            .iter()
            .filter(|x| match x {
                Matching::Pending(_) => false,
                _ => true,
            })
            .map(|x| (Step::Down, x.get_best_down_migration()))
            .rev()
            .collect();

        let mut plan_up: Vec<_> = matches_to_fix
            .iter()
            .filter_map(|x| x.get_local_migration())
            .map(|x| (Step::Up, x))
            .collect();

        let mut plan = plan_down;
        plan.append(&mut plan_up);
        Ok(plan)
    }

    pub fn redo(self) -> Result<Plan<'a>> {
        let matches = self.get_matches()?;
        let filtered_matches: Vec<_> = matches
            .iter()
            .filter(|x| match x {
                Matching::Applied(_) | Matching::Variant(_, _) => true,
                _ => false,
            })
            .collect();

        let plan_down: Vec<_> = filtered_matches
            .iter()
            // Unwrap is safe since it won't be a Divergent matching
            .map(|x| (Step::Down, x.get_local_migration().unwrap()))
            .rev()
            .take(self.count.unwrap_or(1))
            .collect();

        let mut plan_up: Vec<_> = filtered_matches
            .iter()
            // Unwrap is safe since it won't be a Divergent matching
            .map(|x| (Step::Up, x.get_local_migration().unwrap()))
            .take(self.count.unwrap_or(1))
            .collect();

        let mut plan = plan_down;
        plan.append(&mut plan_up);
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

pub type Plan<'a> = Vec<(Step, &'a Migration)>;

#[derive(Debug, PartialEq, Eq)]
pub enum Step {
    Up,
    Down,
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
                down_sql: None,
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
    fn test_up_1() {
        let local = [Migration::new(&"test")];
        let db = [];
        let plan = PlanBuilder::new()
            .local_migrations(&local)
            .db_migrations(&db)
            .up()
            .unwrap();
        assert_eq!(plan, [(Step::Up, &local[0])])
    }

    #[test]
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
    fn test_fix_1() {
        let local = [
            Migration::new(&"test"),
            Migration::new(&"test_1"),
            Migration::new(&"test_2"),
        ];
        let db = [
            Migration::new(&"test"),
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
            .redo()
            .unwrap();
        assert_eq!(
            plan,
            [
                (Step::Down, &local[1]),
                (Step::Down, &local[0]),
                (Step::Up, &local[0]),
                (Step::Up, &local[1]),
            ]
        )
    }
}
