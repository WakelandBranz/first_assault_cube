use linkme::distributed_slice;

// Core feature trait
pub trait FeatureTrait {
    fn new() -> Self
    where
        Self: Sized + Default,
    {
        Default::default()
    }

    fn tick(&mut self) {}
}

// Type alias for boxed trait object
// As a reminder for myself: Box<dyn FeatureTrait> holds any type that implement FeatureTrait
pub type FeatureBox = Box<dyn FeatureTrait>;

// Feature storage
pub struct Feature {
    name: String,
    enabled: bool,
    feature: FeatureBox,  // Using the type alias here
}

// Distributed slice for feature collection from across the program
#[distributed_slice]
pub static FEATURES: [fn() -> Feature] = [..];

impl Feature {
    pub fn new(name: impl ToString, feature: FeatureBox) -> Self {
        Feature {
            name: name.to_string(),
            enabled: false,
            feature
        }
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}