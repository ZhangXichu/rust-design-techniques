pub mod fixed_delay;

pub trait DelayStrategy {
    fn delay(&self) -> std::time::Duration;
}