use crate::{retrier::Retrier, strategies::fixed_delay::FixedDelay};

mod retrier;
mod strategies;

fn main() {
    let retrier = Retrier::new(FixedDelay::new(std::time::Duration::from_secs(1)), 3);

    retrier.run(|| {
        let status = std::process::Command::new("ping")
            .args(["-c", "1", "google.com"])
            .status()?;

        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Failed to execute"))
        }
    });
}
