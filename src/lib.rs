//! # rodash
//!
//! [![Latest Version](https://img.shields.io/crates/v/rodash.svg)](https://crates.io/crates/rodash)
//! [![Rust Documentation](https://docs.rs/rodash/badge.svg)](https://docs.rs/rodash)
//!
//! This crate provides performant general-purpose utility functions inspired by the [lodash]
//! library for the JavaScript ecosystem.
//!
//! [lodash]: https://lodash.com

#![forbid(unsafe_code)]
#![warn(future_incompatible, missing_docs)]

mod difference;
mod difference_all;
mod escape;
mod index_of;
mod initial;
mod intersect;
mod intersect_all;
mod mean;
mod pull;
mod pull_all;
mod sample;
mod shuffle;
mod tail;
mod unescape;
mod uniq;

pub use difference::*;
pub use difference_all::*;
pub use escape::*;
pub use index_of::*;
pub use initial::*;
pub use intersect::*;
pub use intersect_all::*;
pub use mean::*;
pub use pull::*;
pub use pull_all::*;
pub use sample::*;
pub use shuffle::*;
pub use tail::*;
pub use unescape::*;
pub use uniq::*;
