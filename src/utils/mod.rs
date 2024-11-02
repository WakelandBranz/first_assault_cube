use rand::{thread_rng, Rng};

// Using const array for compile-time generation
pub fn random_name_ascii() -> String {
    let mut rng = thread_rng();
    (0..4)
        .map(|_| (rng.gen_range(b'A'..=b'Z') as char))
        .collect()
}