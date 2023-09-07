use std::cmp::Ordering;
use crate::polynomial::Polynomial;
use crate::random_util::{get_from_gaussian, happens_with_probability};

type Point = (f64, f64);
pub struct Individual
{
    polynomial: Polynomial,
    points: Vec<Point>,
    fitness: f64,
}

impl Individual
{
    pub fn new(polynomial: Polynomial, points: &Vec<(f64, f64)>) -> Self
    {
        let fitness = points.iter().map(|(x, y)|
            (y - polynomial.value(*x)) * (y - polynomial.value(*x)))
            .sum();

        let points = points.clone();
        Self
        {
            polynomial,
            points,
            fitness,
        }
    }

    pub fn get_fitness(&self) -> f64
    {
        self.fitness
    }

    pub fn cross_over(&self, other: &Self) -> Self
    {
        let polynomial = self.polynomial.cross_over(&other.polynomial);
        Self::new(polynomial, &other.points)
    }

    pub fn mutate(&mut self)
    {
        let polynomial_degree = self.polynomial.degree();
        for exp in 0..polynomial_degree
        {
            if happens_with_probability(50)
            {
                let coefficient = self.polynomial.get_coefficient(exp)
                    .unwrap();

                let new_coefficient = get_from_gaussian(coefficient, 0.5)
                    .unwrap();

                self.polynomial.set_coefficient(new_coefficient, exp)
                    .unwrap();
            }
        }
    }
}

impl Eq for Individual
{}

impl PartialEq for Individual
{
    fn eq(&self, other: &Self) -> bool
    {
        self.fitness == other.fitness
    }
}

impl PartialOrd for Individual
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        self.fitness.partial_cmp(&other.fitness)
    }
}
impl Ord for Individual
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        match self.fitness.partial_cmp(&other.fitness)
        {
            Some(ordering) => ordering,
            None => Ordering::Equal,
        }
    }
}

impl ToString for Individual
{
    fn to_string(&self) -> String
    {
        self.polynomial.to_string()
    }
}

impl Individual
{
    #[allow(unused)]
    #[cfg(debug_assertions)]
    pub fn print(&self)
    {
        println!("{}", self.polynomial.to_string())
    }
}
