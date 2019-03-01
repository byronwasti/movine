use structopt::{StructOpt};

#[derive(Debug, StructOpt)]
#[structopt(name = "movine", about = "the simple migration manager")]
pub enum Opt {
    #[structopt(name = "status")]
    /// Get the status of migrations (applied, unapplied, mismatched)
    Status {
        #[structopt(short = "d", long = "debug")]
        debug: bool,
    },

    #[structopt(name = "migrate")]
    /// Run migration plan specified by status
    Migrate {
        #[structopt(short = "d", long = "debug")]
        debug: bool,
    },

    #[structopt(name = "generate")]
    /// Generate a migration with a given name
    Generate {
        #[structopt(short = "d", long = "debug")]
        debug: bool,

        name: String,
    },

    #[structopt(name = "init")]
    /// Initialize the database and the local migration directory
    Init {
        #[structopt(short = "d", long = "debug")]
        debug: bool,
    },
}

pub fn load_params() -> Opt {
    Opt::from_args()
}

