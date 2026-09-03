use crate::{retrier::Retrier, strategies::expo_with_jitter::ExponentialDelayWithJitter};

mod retrier;
mod strategies;

fn main() {
    let retrier = Retrier::new(ExponentialDelayWithJitter::new(std::time::Duration::from_secs(1), std::time::Duration::from_secs(10)), 5);

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
