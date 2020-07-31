use crate::match_maker::Matching;
use crate::migration::Migration;
use crate::plan_builder::Step;
use ansi_term::Color;
use std::io::{self, Write};

const LIGHT_RED: u8 = 9;

pub fn print_status(matchings: &[Matching]) {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    use Matching::*;
    for matching in matchings.iter().rev() {
        let reversable_str = if matching.is_reversable() {
            "".to_owned()
        } else {
            Color::Red.paint(" [unreversable]").to_string()
        };

        let (color, status) = match matching {
            // Add spaces in front to make them all the same length
            Applied(_) => (Color::Green, "  Applied"),
            Divergent(_) => (Color::Red, "Divergent"),
            Pending(_) => (Color::Yellow, "  Pending"),
            Variant(_, _) => (Color::Fixed(LIGHT_RED), "  Variant"),
        };

        writeln!(
            handle,
            "{status}{reversable} - {name}",
            name = matching.get_name(),
            status = color.paint(status),
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
        let step = match step {
            // Add spaces in front to make them all the same length
            Up => "  Up",
            Down => "Down",
        };

        println!(
            "{step} - {name}",
            name = migration.name,
            step = Color::Green.paint(step),
        );
    } else {
        println!(
            "{unreversable} - {name}",
            name = migration.name,
            unreversable = Color::Red.paint("Unreversable migration"),
        );
    }
}
