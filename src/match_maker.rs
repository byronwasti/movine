use crate::migration::Migration;
use std::cmp::Ordering;
use std::collections::HashMap;

pub fn find_matches<'a>(
    local_migrations: &'a [Migration],
    db_migrations: &'a [Migration],
) -> Vec<Matching<'a>> {
    let mut matches = Vec::new();
    let mut local_cmp: HashMap<&str, &Migration> = local_migrations
        .iter()
        .map(|x| (x.name.as_ref(), x))
        .collect();

    for m in db_migrations {
        let m_name: &str = m.name.as_ref();
        if let Some((_, loc_m)) = local_cmp.remove_entry(m_name) {
            if loc_m.hash == m.hash {
                matches.push(Matching::Applied(loc_m));
            } else {
                matches.push(Matching::Variant(loc_m, m));
            }
        } else {
            matches.push(Matching::Divergent(m));
        }
    }

    for loc_m in local_cmp.values() {
        matches.push(Matching::Pending(loc_m));
    }

    matches
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Matching<'a> {
    Applied(&'a Migration),
    Divergent(&'a Migration),
    Pending(&'a Migration),
    Variant(&'a Migration, &'a Migration),
}

impl<'a> Matching<'a> {
    pub fn get_name(&self) -> &'a str {
        use Matching::*;
        match self {
            Applied(x) | Divergent(x) | Pending(x) => &x.name,
            Variant(x, _) => &x.name,
        }
    }

    pub fn get_best_down_migration(&self) -> &'a Migration {
        use Matching::*;
        match self {
            Applied(x) | Pending(x) | Divergent(x) => x,
            Variant(x, y) => {
                if x.down_sql.is_some() {
                    x
                } else {
                    y
                }
            }
        }
    }

    pub fn get_local_migration(&self) -> Option<&'a Migration> {
        use Matching::*;
        match self {
            Applied(x) | Pending(x) | Variant(x, _) => Some(x),
            Divergent(_) => None,
        }
    }

    pub fn is_reversable(&self) -> bool {
        use Matching::*;
        match self {
            Applied(x) | Pending(x) | Divergent(x) => x.is_reversable(),
            Variant(x, y) => x.is_reversable() || y.is_reversable(),
        }
    }
}

impl Ord for Matching<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_name = self.get_name();
        let other_name = other.get_name();
        self_name.cmp(other_name)
    }
}

impl PartialOrd for Matching<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
