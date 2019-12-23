use crate::match_maker::Matching;
use crate::migration::Migration;
use crate::plan_builder::Step;
use std::io::{self, Write};
use termion::color;

pub fn print_status(matchings: &[Matching]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    use Matching::*;
    for matching in matchings.iter().rev() {
        writeln!(
            handle,
            "{color}{status}{reset} - {name}",
            name = matching.get_name(),
            status = match matching {
                // Add spaces in front to make them all the same length
                Applied(_) => "  Applied",
                Divergent(_) => "Divergent",
                Pending(_) => "  Pending",
                Variant(_, _) => "  Variant",
            },
            color = match matching {
                Applied(_) => color::Fg(color::Green).to_string(),
                Pending(_) => color::Fg(color::Yellow).to_string(),
                Divergent(_) => color::Fg(color::Red).to_string(),
                Variant(_, _) => color::Fg(color::LightRed).to_string(),
            },
            reset = color::Fg(color::Reset),
        )
        .unwrap();
    }
}

pub fn print_plan(plan: &[(Step, &Migration)]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    use Step::*;
    for (step, migration) in plan.iter() {
        writeln!(
            handle,
            "{color}{step}{reset} - {name}",
            name = migration.name,
            step = match step {
                // Add spaces in front to make them all the same length
                Up => "  Up",
                Down => "Down",
            },
            color = color::Fg(color::Green),
            reset = color::Fg(color::Reset),
        )
        .unwrap();
    }
}
