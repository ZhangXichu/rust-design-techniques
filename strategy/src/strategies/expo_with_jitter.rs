use rand::Rng;

use super::DelayStrategy;

/// Doubles the wait, caps it, then picks a random time up to that, so that many
/// clients retrying at once do not all come back in one wave.
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
        let exp: std::time::Duration = self.delay.saturating_mul(2u32.saturating_pow(attempt));
        let backoff = exp.min(self.cap);
        // this method can return 0 delay
        let max_ms = u64::try_from(backoff.as_millis()).unwrap_or(u64::MAX);
        let jitter_ms = rand::thread_rng().gen_range(1..=max_ms);
        std::time::Duration::from_millis(jitter_ms)
    }
}
