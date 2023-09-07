pub struct Report
 {
    polynomial: String,
    fitness: f64,
    iteration_count: usize,
}

impl Report
{
    pub fn new(polynomial: String, fitness: f64, iteration_count: usize) -> Self
    {
        Report
        {
            polynomial,
            fitness,
            iteration_count,
        }
    }
}

impl ToString for Report
{
    fn to_string(&self) -> String
    {
        format!("polynomial: {}\n\
                fitness: {}\n\
                iteration count: {}",
        self.polynomial.to_string(), self.fitness, self.iteration_count)
    }
}