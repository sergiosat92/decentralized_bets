use moka::sync::Cache;
use once_cell::sync::Lazy;
use std::time::Duration;

pub static CACHE: Lazy<Cache<u8, String>> = Lazy::new(|| {
    Cache::builder()
        .time_to_live(Duration::from_secs(10 * 60))
        .time_to_idle(Duration::from_secs(2 * 60))
        .build()
});
