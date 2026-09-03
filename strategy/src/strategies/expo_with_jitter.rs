use rand::Rng;

use super::DelayStrategy;

pub struct ExponentialDelayWithJitter {
    pub delay: std::time::Duration,
    pub cap: std::time::Duration,
}

impl ExponentialDelayWithJitter {
    pub fn new(delay: std::time::Duration, cap: std::time::Duration) -> Self {
        Self { delay, cap }
    }
}

impl DelayStrategy for ExponentialDelayWithJitter {
    fn delay(&self, attempt: u32) -> std::time::Duration {
        let exp = self.delay.saturating_mul(2u32.saturating_pow(attempt));
        let backoff = exp.min(self.cap);
        let jitter_ms = rand::thread_rng().gen_range(0..=backoff.as_millis() as u64);
        std::time::Duration::from_millis(jitter_ms)
    }
}