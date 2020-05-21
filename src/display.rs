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
        let reversable_str = if matching.is_reversable() {
            "".to_owned()
        } else {
            format!(
                "{color} [unreversable]{reset}",
                color = color::Fg(color::Red),
                reset = color::Fg(color::Reset),
            )
        };

        writeln!(
            handle,
            "{color}{status}{reset}{reversable} - {name}",
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
            reversable = reversable_str,
        )
        .unwrap();
    }
}

pub fn print_plan(plan: &[(Step, &Migration)]) {
    for step in plan.iter() {
        print_step(step);
    }
}

pub fn print_step((step, migration): &(Step, &Migration)) {
    use Step::*;
    if migration.is_reversable() || step == &Step::Up {
        println!(
            "{color}{step}{reset} - {name}",
            name = migration.name,
            step = match step {
                // Add spaces in front to make them all the same length
                Up => "  Up",
                Down => "Down",
            },
            color = color::Fg(color::Green),
            reset = color::Fg(color::Reset),
        );
    } else {
        println!(
            "{color}Unreversable migration{reset} - {name}",
            name = migration.name,
            color = color::Fg(color::Red),
            reset = color::Fg(color::Reset),
        );
    }
}
