use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

#[derive(Clone)]
pub struct Polynomial
{
    coefficients: Vec<f64>,
}

impl Polynomial
{
    pub fn new(coefficients: Vec<f64>) -> Self
    {
        Self { coefficients }
    }

    pub fn new_random(degree: u64) -> Self
    {
        let mut rng = thread_rng();

        let range = Uniform::new(-20.0, 20.0);
        let coefficients = (0..=degree).map(|_| rng.sample(&range))
            .collect();
        Polynomial::new(coefficients)
    }

    pub fn value(&self, x: f64) -> f64
    {
        self.coefficients.iter().enumerate().map(|(exp, coef)| coef * x.powi(exp as i32))
            .sum()
    }

    pub fn degree(&self) -> u64
    {
        (self.coefficients.len() - 1) as u64
    }

    pub fn get_coefficient(&self, exponent: u64) -> Result<f64, String>
    {
        if exponent > self.degree()
        {
            return Err("Bad degree".to_string());
        }

        let coefficient = self.coefficients[exponent as usize];
        Ok(coefficient)
    }

    pub fn set_coefficient(&mut self, coefficient: f64, exponent: u64) -> Result<(), String>
    {
        if exponent > self.degree()
        {
            return Err("Bad degree".to_string());
        }

        self.coefficients[exponent as usize] = coefficient;
        Ok(())
    }

    pub fn cross_over(&self, other: &Self) -> Self
    {
        let mut coefficients = vec![];
        for exp in 0..self.coefficients.len()
        {
            let coef1 = self.coefficients[exp];
            let coef2 = other.coefficients[exp];

            let child = 0.5 * (coef1 + coef2);
            coefficients.push(child);
        }

        Self {coefficients}
    }
}

impl ToString for Polynomial
{
    fn to_string(&self) -> String
    {
        let mut string = String::new();

        for exp in 0..self.coefficients.len()
        {
            let coef = self.coefficients[exp];
            if coef > 0.0
            {
                string += &format!("+{:.2}x^{} ", coef, exp);
            }
            else
            {
                string += &format!("{:.2}x^{} ", coef, exp);
            }
        }

        string
    }
}

#[cfg(test)]
mod test
{
    use crate::polynomial::Polynomial;

    #[test]
    fn test_constant()
    {
        let polynomial = Polynomial::new(vec![21.0]);
        assert_eq!(polynomial.value(37.0), 21.0);
    }

    #[test]
    fn test_x_plus_one_square()
    {
        let polynomial = Polynomial::new(vec!
        [
            // x^2 + 2x + 1
            1.0, 2.0, 1.0,
        ]);

        assert_eq!(polynomial.value(2.0), 9.0);
    }

}