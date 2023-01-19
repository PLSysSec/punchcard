// Fail to build if there are Cargo warnings.
#![deny(warnings)]
// Dead code is typical when writing a library, especially one with
// a shifting API surface.
#![allow(dead_code)]

// These are the lint groups we want enabled. This is... all of them!
#![deny(clippy::cargo)]
#![deny(clippy::suspicious)]
#![deny(clippy::style)]
#![deny(clippy::perf)]
#![deny(clippy::correctness)]
#![deny(clippy::complexity)]
#![deny(clippy::pedantic)]
#![deny(rustdoc::broken_intra_doc_links)]
// These are lints which are not always resolvable.
#![allow(clippy::multiple_crate_versions)]
// These are lints that are taxing to fix when you're trying to
// iterate quickly. Once this project becomes more stable, we should
// enable these lints.
#![allow(clippy::doc_markdown)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_errors_doc)]

pub use config::Config;
pub use lints::Lint;

pub mod bindings;
pub mod cmd;

mod config;
mod lints;
