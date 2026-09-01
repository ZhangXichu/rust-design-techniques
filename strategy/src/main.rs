use crate::{retrier::Retrier, strategies::{expo::ExponentialDelay, fixed_delay::FixedDelay}};

mod retrier;
mod strategies;

fn main() {
    let retrier = Retrier::new(ExponentialDelay::new(std::time::Duration::from_secs(1)), 5);

    let _ = retrier.run(|| {
        let status = std::process::Command::new("false")
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to execute"))
        }
    });
}
