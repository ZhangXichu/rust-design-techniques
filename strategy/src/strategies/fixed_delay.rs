use super::DelayStrategy;

/// Waits the same amount of time after every failure.
pub struct FixedDelay {
    pub delay: std::time::Duration,
}

impl FixedDelay {
    pub fn new(delay: std::time::Duration) -> Self {
        Self { delay }
    }
}

impl DelayStrategy for FixedDelay {
    fn delay(&self, attempt: u32) -> std::time::Duration {
        let _ = attempt;
        self.delay
    }
}
