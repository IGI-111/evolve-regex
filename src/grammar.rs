use std::collections::HashMap;

#[derive(Debug)]
pub struct Grammar {
    root: String,
    rules: HashMap<String, Rule>,
}

impl Grammar {
    pub fn new(root: String, rules: HashMap<String, Rule>) -> Self {
        Self { root, rules }
    }
    pub fn root(&self) -> &Rule {
        self.rules.get(&self.root).unwrap()
    }
    pub fn get_rule(&self, name: &str) -> &Rule {
        self.rules.get(name).unwrap()
    }
}

impl std::fmt::Display for Grammar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (name, rule) in self.rules.iter() {
            writeln!(f, "<{}> ::= {}", name, rule)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum Rule {
    And(Vec<Rule>),
    Or(Vec<Rule>),
    Terminal(String),
    NonTerminal(String),
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rule::And(rules) => {
                write!(
                    f,
                    "{}",
                    rules
                        .iter()
                        .map(|rule| format!("{}", rule))
                        .collect::<Vec<String>>()
                        .join("")
                )?;
            }
            Rule::Or(rules) => {
                write!(
                    f,
                    "{}",
                    rules
                        .iter()
                        .map(|rule| format!("{}", rule))
                        .collect::<Vec<String>>()
                        .join(" | ")
                )?;
            }
            Rule::Terminal(s) => write!(f, "{}", s)?,
            Rule::NonTerminal(i) => write!(f, "<{}>", i)?,
        }
        Ok(())
    }
}
