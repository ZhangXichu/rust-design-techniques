use super::DelayStrategy;

pub struct ExponentialDelay {
    pub delay: std::time::Duration,
}

impl ExponentialDelay {
    pub fn new(delay: std::time::Duration) -> Self {
        Self { delay }
    }
}

impl DelayStrategy for ExponentialDelay {
    fn delay(&self, attempt: u32) -> std::time::Duration {
        self.delay * 2.0f64.powf(attempt as f64) as u32
    }
}