//! Helper for parsing CLI friendly timestamps
//!
//! ## Basic usage
//!
//! ```rust
//! use flexible_time::timestamp::StartTimestamp;
//! use std::str::FromStr;
//! use time::macros::datetime;
//!
//! fn test() {
//!   assert_eq!(datetime!(2023-01-01 0:00 UTC), StartTimestamp::from_str("2023").unwrap().0);
//!   assert_eq!(datetime!(2023-02-01 0:00 UTC), StartTimestamp::from_str("2023-02").unwrap().0);
//! }
//! ```

pub mod error;
pub mod primitives;
pub mod timestamp;
