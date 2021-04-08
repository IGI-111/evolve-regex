use crate::grammar::*;
use rand::Rng;

const CODON_COUNT: usize = 20;

#[derive(Debug)]
pub struct Individual<'a> {
    grammar: &'a Grammar,
    codons: Vec<u8>,
    expression: String,
}

impl<'a> Individual<'a> {
    pub fn new(codon_count: usize, grammar: &'a Grammar) -> Self {
        let mut indiv = Self {
            grammar,
            codons: std::iter::repeat_with(|| rand::random::<u8>())
                .take(codon_count)
                .collect(),
            expression: String::new(),
        };
        indiv.update();
        indiv
    }
    pub fn expression(&self) -> &str {
        &self.expression
    }
    fn update(&mut self) {
        let root = self.grammar.root();
        let mut codon_index = 0;
        self.expression = self.expand_rule(root, &mut codon_index);
    }

    pub fn mutate(&mut self, method: &MutationMethod) {
        match method {
            MutationMethod::CodonFlip(p) => {
                let mut rng = rand::thread_rng();
                for codon in self.codons.iter_mut() {
                    if rng.gen_range(0.0..1.0) < *p {
                        *codon = rng.gen();
                    }
                }
            }
        }
        self.update();
    }

    pub fn crossover(&mut self, method: &CrossoverMethod, other: &mut Self) {
        match method {
            CrossoverMethod::FixedOnePoint(midpoint) => {
                self.codons[0..*midpoint].swap_with_slice(&mut other.codons[0..*midpoint]);
            }
        }
    }

    // FIXME: consider moving to grammar module
    fn expand_rule(&mut self, rule: &Rule, codon_index: &mut usize) -> String {
        if *codon_index >= self.codons.len() {
            "".into()
        } else {
            match rule {
                Rule::And(rules) => rules
                    .iter()
                    .map(|r| self.expand_rule(r, codon_index))
                    .collect::<Vec<String>>()
                    .join(""),
                Rule::Or(rules) => {
                    let selected = self.codons[*codon_index] as usize % rules.len();
                    *codon_index += 1;

                    self.expand_rule(&rules[selected], codon_index)
                }
                Rule::Terminal(s) => s.clone(),
                Rule::NonTerminal(n) => {
                    let r = self.grammar.get_rule(n);
                    self.expand_rule(r, codon_index)
                }
            }
        }
    }
}

pub enum MutationMethod {
    CodonFlip(f32),
}
pub enum CrossoverMethod {
    FixedOnePoint(usize),
}
