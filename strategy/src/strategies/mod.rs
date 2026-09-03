pub mod fixed_delay;
pub mod expo;
pub mod expo_with_jitter;

pub trait DelayStrategy {
    fn delay(&self, attempt: u32) -> std::time::Duration;
}