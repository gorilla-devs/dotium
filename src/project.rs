use crate::version::Version;

use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    /// The project's ID
    pub id: String,
    /// The project's slug
    pub slug: String,
    /// The project's type
    pub project_type: Type,
    /// The project's Name
    pub name: String,
    /// The Description of the project
    pub description: String,
    /// Links to external project pages
    pub links: Links,
    /// Requirements on server and/or client
    pub requirements: Requirement,
    /// The project's categories
    // TODO: Icons, not only the names
    pub categories: Vec<String>,
    /// Download count of the project
    pub downloads: i64,
    /// The count of the current project followers
    /// CurseForge: Thumbs up count
    pub followers: i32,
    /// Icon of the project
    pub icon: request::Asset,
    /// The current status of approval for the mod
    pub status: Status,
    /// The date when the project was published
    /// Curseforge: Date released
    pub published: Datetime,
    /// The date the project was last updated
    pub updated: Datetime,
    /// The date the project was created
    pub created: Datetime,
    /// A mod's pictures / gallery
    pub gallery: Vec<request::Asset>,
    /// If the project allows for distribution
    /// Modrinth: Depends on license
    pub allows_distribution: bool,
    /// The project's authors
    pub authors: Vec<Author>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Mod,
    Modpack,
    Ressourcepack,
    Shader,
    Addon,
    Plugin
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Links {
    ///A link to the mod's GitHub page
    github: Option<String>,
    ///A link to the mod's issue tracker
    issues: Option<String>,
    ///A link to the mod's wiki
    wiki: Option<String>,
    ///A link to the mod's discord
    discord: Option<String>,
    ///An array of to the mod's donation platforms
    donations: Option<Vec<DonationLink>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationLink {
    ///The platform ID
    id: String,
    ///The platform name
    platform: String,
    ///The link to the donations page of the platform
    url: String,
}

/// Requirements of a projects, for server and Client
/// Modrinth only, CF will show up as Unkown
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectRequirements {
    server: Requirement,
    client: Requirement,
}

/// Requirement status
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Requirement {
    Required,
    Optional,
    Unsupported,
    Unkown,
}

/// Status of a Project
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum Status {
    /// Modrinth: Draft
    New, 
    /// Modrinth only
    Unlisted,
    /// CurseForge only
    ChangesRequired, 
    /// CurseForge only
    UnderSoftReview, 
    /// Modrinth: Approved
    Approved, 
    /// Modrinth: Rejected
    Rejected, 
    /// CurseForge only
    ChangesMade, 
    /// CurseForge only
    Inactive, 
    /// Modrinth: Archived
    Abandoned, 
    /// Modrinth: Unkown
    Deleted, 
    /// Modrinth: Processing
    UnderReview, 
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Author {
    /// The unique username of the user
    username: String,
    /// The display name of the user
    name: String,
    // The ID of the user
    id: String,
    /// The Avatar URL of the user
    avatar_url: Option<String>
}

pub trait ProjectTrait {
    fn get_body(&self) -> Result<String>;

    fn get_versions(&self) -> Result<Vec<Version>>;

    fn get_version(&self, id: String) -> Result<Version>;
}
