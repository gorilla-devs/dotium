//! # Dotium
//!
//! The uniform object provider for Minecraft Mod hosting platforms.
//! Implements traits for api wrappers Ferinth, Furse, and Octocrab to then be used platform independedly in GDLauncher

#[cfg(feature = "curseforge")]
mod curseforge;
#[cfg(feature = "github")]
mod github;
#[cfg(feature = "modrinth")]
mod modrinth;
pub mod project;
pub mod request;
pub mod version;

use async_trait::async_trait;
use project::{Author, Project};
use serde::{Deserialize, Serialize};
use version::Version;

#[derive(Debug, Clone)]
pub struct Platform {
    /// The name of the platform
    pub name: String,
    /// The two-letter nick name of the Platform
    pub short_name: String,
    /// Logo of the platform
    pub logo: request::Asset,
}

#[derive(thiserror::Error, Debug)]
#[error("The trait is not implemented")]
pub struct UnimplementedError;

pub(crate) type Datetime = chrono::DateTime<chrono::Utc>;

#[async_trait]
pub trait Container<E: From<UnimplementedError>, ID: Sync + Send> {
    fn new() -> Self;
    fn get_platform() -> Platform;

    // GATs unstable
    // type Result<T> = std::result::Result<T, E>;

    async fn get_project(&self, id: &ID) -> Result<Project<ID>, E>;

    async fn get_projects(&self, ids: Vec<&ID>) -> Result<Vec<Project<ID>>, E> {
        let mut projects = Vec::new();
        for id in ids {
            projects.push(self.get_project(id).await?);
        }
        Ok(projects)
    }

    async fn get_project_body(&self, project_id: &ID) -> Result<String, E>;

    async fn get_project_authors(&self, project_id: &ID) -> Result<Vec<Author<ID>>, E>;

    async fn get_project_dependencies(
        &self,
        project_id: &ID,
    ) -> Result<(Vec<Project<ID>>, Vec<Version<ID>>), E>;

    async fn get_version(&self, project_id: &ID, id: &ID) -> Result<Version<ID>, E>;

    async fn get_project_versions(&self, project_id: &ID) -> Result<Vec<Version<ID>>, E>;

    async fn get_versions(&self, ids: Vec<(&ID, &ID)>) -> Result<Vec<Version<ID>>, E> {
        let mut versions = Vec::new();
        for id in ids {
            versions.push(self.get_version(id.0, id.1).await?);
        }
        Ok(versions)
    }

    // async fn search(
    //     &self,
    //     query: &str,
    //     project_type: Option<&str>,
    //     mc_version: Vec<&str>,
    //     modloader: &str,
    //     category: &str,
    // ) -> Result<Vec<project::Project>, E>;
}
