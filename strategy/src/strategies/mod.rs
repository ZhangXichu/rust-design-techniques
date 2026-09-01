pub mod fixed_delay;
pub mod expo;

pub trait DelayStrategy {
    fn delay(&self, attempt: u32) -> std::time::Duration;
}