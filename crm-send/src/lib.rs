mod abi;
mod config;
mod pb;

pub use abi::*;
use chrono::Local;
pub use config::*;
pub use pb::*;
use prost_types::Timestamp;

trait Now {
    fn now() -> Timestamp;
}

impl Now for Timestamp {
    fn now() -> Timestamp {
        let now = Local::now();
        Timestamp {
            seconds: now.timestamp(),
            nanos: now.timestamp_subsec_nanos() as i32,
        }
    }
}
