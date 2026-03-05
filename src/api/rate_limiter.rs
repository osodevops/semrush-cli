use governor::{Quota, RateLimiter as GovRateLimiter};
use std::num::NonZeroU32;
use std::sync::Arc;

pub type Limiter = GovRateLimiter<
    governor::state::NotKeyed,
    governor::state::InMemoryState,
    governor::clock::DefaultClock,
    governor::middleware::NoOpMiddleware,
>;

pub fn create(requests_per_second: u32) -> Arc<Limiter> {
    let rps = NonZeroU32::new(requests_per_second).unwrap_or(NonZeroU32::new(10).unwrap());
    let quota = Quota::per_second(rps);
    Arc::new(GovRateLimiter::direct(quota))
}
