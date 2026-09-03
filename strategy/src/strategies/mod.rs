pub mod fixed_delay;
pub mod expo;
pub mod expo_with_jitter;

/// Decides how long to wait before the next retry, given how many times the
/// operation has failed so far.
pub trait DelayStrategy {
    fn delay(&self, attempt: u32) -> std::time::Duration;
}
