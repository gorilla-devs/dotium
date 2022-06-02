use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Version {
    /// The ID of the version
    id: String,

    /// Dependencies on other projects
    pub dependencies: Vec<ProjectDependency>,
}

/// Dependency on a Project's version
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectDependency {
    //The ID of the project to depend on
    project_id: String,
    //The version of the project it depends on
    version: Version,
}
