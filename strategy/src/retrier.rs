use crate::strategies::DelayStrategy;

pub struct Retrier {
    pub strategy: Box<dyn DelayStrategy>,
    pub max_attempts: u32,
}

impl Retrier {
    pub fn new<S: DelayStrategy + 'static>(strategy: S, max_attempts: u32) -> Self {
        Self { strategy: Box::new(strategy), max_attempts }
    }

    pub fn run<F, T, E>(&self, f: F) -> Result<T, E>
    where
        F: Fn() -> Result<T, E>,
        E: std::fmt::Display,
    {
        let mut attempt = 0;

        loop {
            match f() {
                Ok(result) => return Ok(result),
                Err(e) => {
                    attempt += 1;
                    if attempt >= self.max_attempts {
                        return Err(e);
                    }
                    let delay_time = self.strategy.delay(attempt);

                    println!("Attempt {} failed: {}, retrying in {} seconds", attempt, e, delay_time.as_secs());
                    
                    std::thread::sleep(delay_time);
                }
            }
        }
    }

}