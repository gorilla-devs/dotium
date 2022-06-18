//! # Dotium
//!
//! Dotium is the uniform object provider for Minecraft Mod hosting platforms,
//! who get traits in Libraries like Ferinth and Furse to then be used platform independedly in Libium.
//!
//!
use async_trait::async_trait;
use project::{Author, Project};
use serde::{Deserialize, Serialize};
use version::Version;

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

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("The Trait is not implemented.")]
    Unimplemented
}

#[async_trait]
pub trait DotiumTrait {
    fn new() -> Self;

    fn get_platform() -> Platform;

    async fn get_project(&self, id: &str) -> Result<project::Project>;

    async fn get_projects(&self, ids: Vec<&str>) -> Result<Vec<project::Project>> {
        let mut deps = Vec::new();
        for i in ids {
            deps.push(self.get_project(i).await?);
        }
        Ok(deps)
    }

    async fn get_project_body(&self, project_id: &str) -> Result<String>;

    async fn get_project_authors(&self, project_id: &str) -> Result<Vec<Author>>;

    async fn get_project_dependencies(
        &self,
        project_id: &str,
    ) -> Result<(Vec<Project>, Vec<Version>)> {
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

    async fn get_project_version(&self, project_id: &str, id: &str) -> Result<Version>;

    async fn get_versions(&self, project_id: &str) -> Result<Vec<Version>>;

    /// Map consists of project, version ID
    async fn get_project_versions(&self, ids: Vec<(&str, &str)>) -> Result<Vec<version::Version>> {
        let mut deps = Vec::new();
        for i in ids {
            deps.push(self.get_project_version(i.0, i.1).await?);
        }
        Ok(deps)
    }

    // Parameters will get extended
    async fn search(
        &self,
        query: &str,
        project_type: Option<&str>,
        mc_version: Vec<&str>,
        modloader: &str,
        category: &str,
    ) -> Result<Vec<project::Project>>;
}

pub(crate) type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send>>;
pub(crate) type Datetime = chrono::DateTime<chrono::Utc>;
