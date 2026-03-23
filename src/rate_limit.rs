use std::time::{Instant, Duration};

pub struct RateLimiter {
    tokens: f64,
    max_tokens: f64,
    refill_rate: f64,
    last_check: Instant,
}

impl RateLimiter {
    pub fn new(max_per_sec: f64) -> Self {
        Self { tokens: max_per_sec, max_tokens: max_per_sec, refill_rate: max_per_sec, last_check: Instant::now() }
    }
    pub fn try_acquire(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_check).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_check = now;
        if self.tokens >= 1.0 { self.tokens -= 1.0; true } else { false }
    }
}
