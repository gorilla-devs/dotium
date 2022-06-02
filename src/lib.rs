//! # Dotium
//!
//! Dotium is the uniform object provider for Minecraft Mod hosting platforms,
//! who get traits in Libraries like Ferinth and Furse to then be used platform independedly in Libium.
//!
use serde::{Deserialize, Serialize};

pub mod project;
pub mod request;
pub mod version;

pub struct Platform {
    /// The name of the platform
    name: String,
    /// The two-letter nick name of the Platform
    short_name: String,
    /// Logo of the platform
    logo: request::Asset,
}

pub trait PlatformTrait {
    fn get_project(id: String) -> Result<project::Project>;

    // TODO Search
    fn search() -> Result<Vec<project::Project>>;
}

#[derive(thiserror::Error, Debug)]
pub enum Error {}

pub(crate) type Result<T> = std::result::Result<T, Error>;
pub type Datetime = chrono::DateTime<chrono::Utc>;
