pub struct Migration {
    name: String,
    up_sql: String,
    down_sql: String,
    m_type: MigrationType,
}

pub enum MigrationType {
    Up,
    Down,
}

impl Migration {
    pub fn new(name: String, up_sql: String, down_sql: String, m_type: MigrationType) -> Self {
        Self {
            m_type,
            name,
            up_sql,
            down_sql,
        }
    }
}

