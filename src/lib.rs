//! # Dotium
//!
//! Dotium is the uniform object provider for Minecraft Mod hosting platforms,
//! who get traits in Libraries like Ferinth and Furse to then be used platform independedly in Libium.
//!
//!
use async_trait::async_trait;
use project::Author;
use serde::{Deserialize, Serialize};
use version::{Version, VersionDependency};

pub mod project;
pub mod request;
pub mod version;

pub struct Platform {
    /// The name of the platform
    pub name: String,
    /// The two-letter nick name of the Platform
    pub short_name: String,
    /// Logo of the platform
    pub logo: request::Asset,
}

#[async_trait]
pub trait DotiumTrait {
    fn new() -> Self;

    fn get_platform() -> Platform;

    async fn get_project(&self, id: String) -> Result<project::Project>;

    async fn get_projects(&self, ids: Vec<String>) -> Result<Vec<project::Project>> {
        let mut deps = Vec::new();
        for i in ids {
            deps.push(self.get_project(i).await?);
        }
        Ok(deps)
    }

    async fn get_project_body(&self, project_id: String) -> Result<String>;

    async fn get_project_authors(&self, project_id: String) -> Result<Vec<Author>>;

    async fn get_project_dependencies(&self, project_id: String) -> Result<Vec<VersionDependency>> {
        let mut deps = Vec::new();
        for mut i in self.get_project_versions(project_id).await? {
            deps.append(&mut i.dependencies);
        }
        Ok(deps)
    }

    async fn get_project_version(&self, project_id: String, id: String) -> Result<Version>;

    async fn get_project_versions(&self, project_id: String) -> Result<Vec<Version>>;

    // Parameters will get extended
    async fn search(
        query: String,
        project_type: Option<String>,
        mc_version: Vec<String>,
        modloader: String,
        category: String,
    ) -> Result<Vec<project::Project>>;
}

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send>>;
pub(crate) type Datetime = chrono::DateTime<chrono::Utc>;
