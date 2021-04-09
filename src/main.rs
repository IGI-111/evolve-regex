#![feature(trace_macros)]

mod grammar;
mod individual;
mod population;
#[macro_use]
mod utils;

use grammar::*;
use individual::*;
use population::*;
use regex::Regex;
use std::array::IntoIter;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let grammar = Grammar::new(
        "start".into(),
        HashMap::<_, _>::from_iter(IntoIter::new([
            rule!(<string> ::= "\\w" | "a-z" | "A-Z" | "\\d" | "\\s"),
            rule!(<repetition> ::= '*' | '+' | '?'),
            rule!(<auxiliary> ::= <symbol> | <string> | <auxiliary> & <auxiliary>),
            rule!(<sep> ::= '|'),
            rule!(<symbol> ::= '/' | ':' | '-' | '=' | '&' | '%' | '#' | ';' | '~' | '\'' | ',' | '!' | '@' | '<' | "\\." | "\\|" | "\\(" | "\\)" | "\\{" | "\\}" | "\\[" | "\\]" | "\\?" | "\\+" | "\\$"),
        ])),
    );

    println!("{}", grammar);

    let to_match = "fjdksqljf 12321 abcdabcabcdabcbc";

    let fitness = |indiv: &Individual| -> f32 {
        let expr = indiv.expression();
        let expr_len = expr.len() as f32;
        let match_ratio = length_matched(expr, to_match) / to_match.len() as f32;
        let length_ratio = 1.0 / (1.0 + 0.0001 * expr_len);

        match_ratio * length_ratio
    };
    let mut pop = Population::new(
        10,
        fitness,
        100,
        SelectionMethod::Truncation(1),
        CrossoverMethod::FixedOnePoint(50),
        MutationMethod::CodonFlip(0.5),
        &grammar,
    );

    loop {
        let expressions = pop.expressions();
        let best = expressions.first().unwrap();
        println!(
            "best '{}' with {}/{}",
            best,
            length_matched(best, to_match),
            to_match.len()
        );
        pop.next_generation();
    }
}

fn length_matched(expr: &str, to_match: &str) -> f32 {
    let re = Regex::new(expr);
    if let Ok(re) = re {
        if let Some(m) = re.find(to_match) {
            (m.end() - m.start()) as f32
        } else {
            0.0
        }
    } else {
        0.0
    }
}
