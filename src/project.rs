use super::*;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project<ID> {
    /// The project's ID
    pub id: ID,
    /// The project's slug
    pub slug: String,
    /// The project's type
    pub project_type: ProjectType,
    /// The project's Name
    pub name: String,
    /// The description of the project
    pub description: String,
    /// Links to external project pages
    pub links: Links,
    /// Requirements on server and/or client
    pub requirements: ProjectRequirements,
    /// The project's categories
    // TODO: Icons, not only the names
    pub categories: Vec<String>,
    /// Download count of the project
    pub downloads: usize,
    /// The count of the current project followers
    /// CurseForge: Thumbs up count
    pub followers: usize,
    /// Icon of the project
    pub icon: Option<request::Asset>,
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
    /// Team id of the project's team
    /// CurseForge: Project ID, as authors are included in the project
    pub team_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum ProjectType {
    Mod,
    Modpack,
    Ressourcepack,
    Shader,
    Addon,
    Plugin,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Links {
    /// GitHub page
    pub github: Option<String>,
    /// Issue tracker
    pub issues: Option<String>,
    /// Wiki page
    pub wiki: Option<String>,
    /// Discord invite
    pub discord: Option<String>,
    /// Donation platforms
    pub donations: Vec<DonationLink>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DonationLink {
    /// The platform's ID
    pub id: String,
    /// The platform name
    pub platform: String,
    /// The link to the donations page
    pub url: String,
}

/// Requirements of a project, for server and client
/// Modrinth only, CF will show up as Unkown
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectRequirements {
    pub server: Requirement,
    pub client: Requirement,
}

/// Requirement status
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
pub enum Requirement {
    Required,
    Optional,
    Unsupported,
    Unknown,
}

/// Status of a Project

// We should combine some of these (e.g. Abandoned & Inactive)
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
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
pub struct Author<ID> {
    /// The unique username of the user
    pub username: String,
    /// The display name of the user
    pub name: String,
    // The ID of the user
    pub id: ID,
    /// The user's avatar
    pub avatar_url: Option<request::Asset>,
}
