use crate::{grammar::*, individual::*};

pub struct Population<'a, F>
where
    F: Fn(&Individual<'a>) -> f32,
{
    individuals: Vec<Individual<'a>>,
    fitness: F,
    selection_method: SelectionMethod,
    crossover_method: CrossoverMethod,
    mutation_method: MutationMethod,
}

impl<'a, F> Population<'a, F>
where
    F: Fn(&Individual<'a>) -> f32,
{
    pub fn new(
        size: usize,
        fitness: F,
        codon_count: usize,
        selection_method: SelectionMethod,
        crossover_method: CrossoverMethod,
        mutation_method: MutationMethod,
        grammar: &'a Grammar,
    ) -> Self {
        Self {
            individuals: std::iter::repeat_with(|| Individual::new(codon_count, grammar))
                .take(size)
                .collect(),
            fitness,
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn expressions(&mut self) -> Vec<&str> {
        self.individuals
            .iter_mut()
            .map(|indiv| indiv.expression())
            .collect()
    }

    pub fn next_generation(&mut self) {
        // selection
        let breakoff = self.select();
        // crossover
        {
            let to_cross = &mut self.individuals[0..breakoff];
            for chunk in to_cross.chunks_exact_mut(2) {
                let (a, b) = chunk.split_at_mut(1);
                a.first_mut()
                    .unwrap()
                    .crossover(&self.crossover_method, &mut b.first_mut().unwrap());
            }
        }

        // mutate
        {
            let to_mutate = &mut self.individuals[breakoff..];
            for indiv in to_mutate.iter_mut() {
                indiv.mutate(&self.mutation_method);
            }
        }
    }

    fn select(&mut self) -> usize {
        match self.selection_method {
            SelectionMethod::Truncation(breakoff) => {
                let mut individuals = std::mem::take(&mut self.individuals);
                individuals
                    .sort_by(|a, b| (self.fitness)(b).partial_cmp(&(self.fitness)(a)).unwrap());
                self.individuals = individuals;
                breakoff
            }
        }
    }
}

pub enum SelectionMethod {
    Truncation(usize),
    //Tournament
}
