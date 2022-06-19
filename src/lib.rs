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
pub enum Error {
    #[error("The trait is not implemented")]
    Unimplemented,
}

pub(crate) type Datetime = chrono::DateTime<chrono::Utc>;

#[async_trait]
pub trait Container<E: Into<Error>> {
    fn new() -> Self;
    fn get_platform() -> Platform;

    type ID;
    // GATs unstable
    // type Result<T> = std::result::Result<T, E>;

    async fn get_project(&self, id: &Self::ID) -> Result<Project<Self::ID>, E>;

    async fn get_projects(&self, ids: Vec<&Self::ID>) -> Result<Vec<Project<Self::ID>>, E> {
        let mut projects = Vec::new();
        for id in ids {
            projects.push(self.get_project(id).await?);
        }
        Ok(projects)
    }

    async fn get_project_body(&self, project_id: &Self::ID) -> Result<String, E>;

    async fn get_project_authors(&self, project_id: &Self::ID) -> Result<Vec<Author<Self::ID>>, E>;

    async fn get_project_dependencies(
        &self,
        project_id: &Self::ID,
    ) -> Result<(Vec<Project<Self::ID>>, Vec<Version<Self::ID>>), E> {
        let mut projects: Vec<Project> = Vec::new();
        let mut versions: Vec<Version> = Vec::new();
        for i in self.get_versions(project_id).await? {
            for dep in i.dependencies.into_iter() {
                if !projects
                    .iter()
                    .any(|project| Some(&project.id) == dep.project_id.as_ref())
                {
                    match &dep.project_id {
                        Some(id) => {
                            projects.push(self.get_project(id.as_str()).await?);
                        }
                        None => {}
                    }
                }
                if !versions
                    .iter()
                    .any(|version| Some(&version.id) == dep.version.as_ref())
                {
                    match &dep.version {
                        Some(id) => {
                            versions.push(
                                self.get_project_version(
                                    dep.project_id.unwrap_or_default().as_str(),
                                    id.as_str(),
                                )
                                .await?,
                            );
                        }
                        None => {}
                    }
                }
            }
        }
        Ok((projects, versions))
    }

    async fn get_version(
        &self,
        project_id: &Self::ID,
        id: &Self::ID,
    ) -> Result<Version<Self::ID>, E>;

    async fn get_project_versions(
        &self,
        project_id: &Self::ID,
    ) -> Result<Vec<Version<Self::ID>>, E>;

    async fn get_versions(
        &self,
        ids: Vec<(&Self::ID, &Self::ID)>,
    ) -> Result<Vec<Version<Self::ID>>, E> {
        let mut versions = Vec::new();
        for id in ids {
            versions.push(self.get_project_version(id.0, id.1).await?);
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
