use std::time::Duration;

use crate::{
    retrier::Retrier,
    strategies::{
        expo::ExponentialDelay, expo_with_jitter::ExponentialDelayWithJitter,
        fixed_delay::FixedDelay, DelayStrategy,
    },
};

mod retrier;
mod strategies;

fn failing_command() -> anyhow::Result<()> {
    let status = std::process::Command::new("false").status()?;
    if status.success() {
        Ok(())
    } else {
        Err(anyhow::anyhow!("Failed to execute"))
    }
}

fn run_demo(name: &str, strategy: impl DelayStrategy + 'static) {
    println!("\n=== {name} ===");
    let retrier = Retrier::new(strategy, 4);
    let _ = retrier.run(failing_command);
}

fn main() {
    let base = Duration::from_secs(1);

    run_demo("Fixed delay (1s)", FixedDelay::new(base));
    run_demo("Exponential delay", ExponentialDelay::new(base));
    run_demo(
        "Exponential delay with jitter (cap 10s)",
        ExponentialDelayWithJitter::new(base, Duration::from_secs(10)),
    );
}
