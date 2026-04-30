//! HOPR node daemon binary. Runs the HOPR protocol (via [`hopr-lib`](https://github.com/hoprnet/hoprnet))
//! and exposes a REST API for node management.
//!
//! When the REST API is enabled, interactive API docs are available at:
//! - `http://localhost:3001/scalar` (Scalar UI)
//! - `http://localhost:3001/swagger-ui` (Swagger UI)
//!
//! ## Usage
//! See `hoprd --help` for the full list of options.

pub mod cli;
pub mod config;
pub mod errors;
pub mod strategy;
