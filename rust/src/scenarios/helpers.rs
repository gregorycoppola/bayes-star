
use rand::Rng;
pub fn weighted_cointoss(threshold: f64) -> bool {
    let mut rng = rand::thread_rng(); // Get a random number generator
    if rng.gen::<f64>() < threshold {
        true
    } else {
        false
    }
}
