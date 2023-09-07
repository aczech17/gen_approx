use crate::individual;
use individual::Individual;
use crate::polynomial::Polynomial;
use crate::report::Report;
use crate::random_util::happens_with_probability;

const MUTATION_PROBABILITY: u8 = 10;

pub struct Ecosystem
{
    individuals: Vec<Individual>,
    //points: Vec<(f64, f64)>,
}

impl Ecosystem
{
    pub fn new(individual_count: usize, polynomial_degree: u64, points: Vec<(f64, f64)>) -> Self
    {
        let mut individuals = vec![];
        for _ in 0..individual_count
        {
            let new_polynomial = Polynomial::new_random(polynomial_degree);
            let new_individual = Individual::new(new_polynomial, &points);
            individuals.push(new_individual);
        }

        Self
        {
            individuals,
            //points,
        }
    }

    fn run_generation(&mut self)
    {
        self.individuals.sort(); // Sort by fitness.

        // remove the worst 1/3 of population
        let length = self.individuals.len();
        for _i in 0..length / 3
        {
            self.individuals.pop();
        }

        let mut children = vec![];
        for i in 0..self.individuals.len() - 1
        {
            let parent1 = &self.individuals[i];
            let parent2 = &self.individuals[i + 1];

            let mut child = parent1.cross_over(parent2);
            if happens_with_probability(MUTATION_PROBABILITY)
            {
                child.mutate();
            }

            children.push(child);
        }

        self.individuals.append(&mut children);
    }
    
    pub fn run(&mut self, generation_count: usize) -> Report
    {
        let mut generation = 0;
        while generation < generation_count
        {
            self.run_generation();
            let best = self.individuals.iter().min()
                .unwrap();

            //self.print();
            //best.print();
            
            if best.get_fitness() == 0.0 // perfect
            {
                break;
            }
            
            generation += 1;
        }
        
        let best = self.individuals.iter().min()
            .unwrap();
        let report = Report::new(best.to_string(),
                                 best.get_fitness(),
                                 generation);

        report
    }
}

impl Ecosystem
{
    #[allow(unused)]
    #[cfg(debug_assertions)]
    fn print(&self)
    {
        println!();
        for individual in &self.individuals
        {
            individual.print();
        }
        println!();
    }
}

#[cfg(test)]
mod test
{
    use crate::individual::Individual;
    use crate::polynomial::Polynomial;

    #[test]
    fn fitness_sorting_simple_test()
    {
        let points = vec![(0.0, 0.0), (1.0, 1.0), (-2.0, 4.0), (3.0, 9.0)];

        let x_square = Polynomial::new(vec![0.0, 0.0, 1.0]);
        let random_shit = Polynomial::new(vec![21.0, 37.0, -5.5]);

        let mut individuals = vec!
        [
            Individual::new(x_square, &points),
            Individual::new(random_shit, &points)
        ];

        //let ecosystem = Ecosystem { individuals };

        individuals.sort();

        assert_eq!(individuals[0].get_fitness(), 0.0)
    }
}