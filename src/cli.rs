use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "movine", about = "the simple migration manager")]
pub enum Opt {
    #[structopt(name = "status")]
    /// Get the status of migrations (applied, unapplied, mismatched).
    Status {
        #[structopt(short = "v", long = "verbose")]
        /// Run with verbose logging
        debug: bool,
    },

    #[structopt(name = "up")]
    /// Run all pending migrations.
    Up {
        #[structopt(short = "n", long = "number")]
        /// Number of up or down migrations to run.
        number: Option<usize>,

        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "s", long = "strict")]
        /// Error out on out-of-order pending migrations.
        strict: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Run with verbose logging
        debug: bool,
    },

    #[structopt(name = "down")]
    /// Rollback the latest migration.
    Down {
        #[structopt(short = "n", long = "number")]
        /// Number of up or down migrations to run.
        number: Option<usize>,

        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "i", long = "ignore-divergent")]
        /// Ignore any divergent migrations.
        ignore_divergent: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Run with verbose logging
        debug: bool,
    },

    #[structopt(name = "fix")]
    /// Rollback all divergent migrations and variant migrations, and then run all pending.
    Fix {
        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Run with verbose logging
        debug: bool,
    },

    #[structopt(name = "redo")]
    /// Rollback the most recent migration and then run it.
    Redo {
        #[structopt(short = "n", long = "number")]
        /// Number of up or down migrations to run.
        number: Option<usize>,

        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "i", long = "ignore-divergent")]
        /// Ignore any divergent migrations.
        ignore_divergent: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Run with verbose logging
        debug: bool,
    },

    #[structopt(name = "custom")]
    /// [unimplemented]
    Custom {
        #[structopt(short = "p", long = "plan")]
        /// Do a dry run and show the migration plan.
        show_plan: bool,

        #[structopt(short = "v", long = "verbose")]
        /// Run with verbose logging
        debug: bool,

        plan: Vec<String>,
    },

    #[structopt(name = "generate")]
    /// Generate a migration with a given name.
    Generate {
        #[structopt(short = "v", long = "verbose")]
        /// Run with verbose logging
        debug: bool,

        name: String,
    },

    #[structopt(name = "init")]
    /// Initialize the database and the local migration directory.
    Init {
        #[structopt(short = "v", long = "verbose")]
        /// Run with verbose logging
        debug: bool,
    },
}
