use crate::request::Asset;

use super::*;

/// Version of a project
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Version {
    /// The ID of the version
    pub id: String,
    /// The Display name of the version
    pub name: String,
    /// The identifier of the version
    pub identifier: String,
    /// The ID of the parent project
    pub project_id: String,
    /// Jar Files of the Mod
    pub files: Vec<Asset>,
    /// Downloads of this version
    pub downloads: usize,
    /// Loaders this version supports
    pub loaders: Vec<ModLoader>,
    /// Game versions this version supports
    pub game_versions: Vec<String>,
    /// The Date when this version was published
    pub published: Datetime,
    /// The type of version, Release, Beta or Alpha
    pub version_type: VersionType,
    /// Dependencies on other projects
    pub dependencies: Vec<VersionDependency>,
}

/// Dependency on a Project's version
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionDependency {
    /// The ID of the project to depend on
    pub project_id: Option<String>,
    /// The version of the project it depends on
    /// If no version is specified, try to find the latest compatible version
    pub version: Option<String>,
    /// The dependency type
    pub dependency_type: DependencyType,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DependencyType {
    EmbeddedLibrary,
    OptionalDependency,
    RequiredDependency,
    Tool,
    Incompatible,
    Include,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ModLoader {
    Quilt,
    Fabric,
    Forge,
    Rift,
    Modloader,
    Unkown,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum VersionType {
    Alpha,
    Beta,
    Release,
}
