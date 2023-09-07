use rand::{Rng, thread_rng};
use rand_distr::Normal;
use rand_distr::Uniform;

pub fn happens_with_probability(probability: u8) -> bool
{
    let mut rng = thread_rng();
    let distribution = Uniform::new(0, 100);
    let value = rng.sample(&distribution);

    value < probability
}

pub fn get_from_gaussian(mean: f64, std_dev: f64) -> Result<f64, String>
{
    let mut rng = thread_rng();
    let distribution = Normal::new(mean, std_dev)
        .map_err(|e| e.to_string())?;
    let value = rng.sample(&distribution);
    Ok(value)
}
