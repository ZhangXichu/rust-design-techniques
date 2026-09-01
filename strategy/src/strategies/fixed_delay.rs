use super::DelayStrategy;

pub struct FixedDelay {
    pub delay: std::time::Duration,
}

impl FixedDelay {
    pub fn new(delay: std::time::Duration) -> Self {
        Self { delay }
    }
}

impl DelayStrategy for FixedDelay {
    fn delay(&self) -> std::time::Duration {
        self.delay
    }
}