use super::DelayStrategy;

/// Doubles the wait after every failure: `delay * 2^attempt`.
pub struct ExponentialDelay {
    pub delay: std::time::Duration,
    pub cap: std::time::Duration,
}

impl ExponentialDelay {
    pub fn new(delay: std::time::Duration, cap: std::time::Duration) -> Self {
        Self { delay, cap }
    }
}

impl DelayStrategy for ExponentialDelay {
    fn delay(&self, attempt: u32) -> std::time::Duration {
        let d = self.delay.saturating_mul(2u32.saturating_pow(attempt));
        d.min(self.cap)
    }
}
