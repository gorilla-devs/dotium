use super::*;
use crate::request::Asset;

/// Version of a project
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Version<ID> {
    /// The ID of the version
    pub id: ID,
    /// The Display name of the version
    pub name: String,
    /// The identifier of the version
    pub identifier: String,
    /// The ID of the parent project
    pub project_id: ID,
    /// Jar files of the Mod
    pub files: Vec<Asset>,
    /// Downloads of this version
    pub downloads: usize,
    /// Loaders this version supports
    pub loaders: Vec<ModLoader>,
    /// Game versions this version supports
    pub game_versions: Vec<String>,
    /// When this version was published
    pub published: Datetime,
    /// The type of version, Release, Beta or Alpha
    pub version_type: VersionType,
    /// Dependencies on other projects
    pub dependencies: Vec<VersionDependency<ID>>,
}

/// Dependency on a Project's version
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VersionDependency<ID> {
    /// The ID of the project to depend on
    pub project_id: Option<ID>,
    /// The version of the project it depends on
    /// If no version is specified, try to find the latest compatible version
    pub version: Option<ID>,
    /// The dependency type
    pub dependency_type: DependencyType,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum DependencyType {
    OptionalDependency,
    RequiredDependency,
    EmbeddedLibrary,
    Incompatible,
    Include,
    Tool,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ModLoader {
    Modloader,
    Unknown,
    Fabric,
    Quilt,
    Forge,
    Rift,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum VersionType {
    Alpha,
    Beta,
    Release,
}
