use super::ID;
use crate::{project, request, version};
use ferinth::structures::*;

impl From<project_structs::Project> for project::Project<ID> {
    fn from(from: project_structs::Project) -> Self {
        project::Project {
            id: from.id,
            slug: from.slug,
            project_type: from.project_type.into(),
            name: from.title,
            description: from.description,
            links: project::Links {
                github: from.source_url,
                issues: from.issues_url,
                wiki: from.wiki_url,
                discord: from.discord_url,
                donations: from.donation_urls.into_iter().map(Into::into).collect(),
            },
            requirements: project::ProjectRequirements {
                server: from.server_side.into(),
                client: from.client_side.into(),
            },
            categories: from.categories,
            downloads: from.downloads,
            followers: from.followers,
            icon: from.icon_url.map(request::Asset::by_url),
            status: from.status.into(),
            published: from.published,
            updated: from.updated,
            created: from.published,
            gallery: from.gallery.into_iter().map(Into::into).collect(),
            allows_distribution: &from.license.id != "custom" && &from.license.id != "arr",
        }
    }
}

impl From<version_structs::Version> for version::Version<ID> {
    fn from(from: version_structs::Version) -> Self {
        Self {
            id: from.id,
            name: from.name,
            identifier: from.version_number,
            project_id: from.project_id,
            files: from.files.into_iter().map(Into::into).collect(),
            downloads: from.downloads,
            loaders: from
                .loaders
                .into_iter()
                .map(|loader| version::ModLoader::from(loader.as_ref()))
                .collect(),
            game_versions: from.game_versions,
            published: from.date_published,
            version_type: from.version_type.into(),
            dependencies: from.dependencies.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<version_structs::VersionFile> for request::Asset {
    fn from(from: version_structs::VersionFile) -> Self {
        Self {
            url: from.url,
            name: Some(from.filename),
            description: None,
            headers: std::collections::HashMap::new(),
            request_type: request::RequestType::GET,
            hash: Some(request::AssetHash {
                hash: from.hashes.sha512,
                algorithm: request::HashAlgorithm::SHA512,
            }),
            size: Some(from.size),
        }
    }
}

impl From<version_structs::Dependency> for version::VersionDependency<ID> {
    fn from(from: version_structs::Dependency) -> Self {
        Self {
            project_id: from.project_id,
            version: from.version_id,
            dependency_type: from.dependency_type.into(),
        }
    }
}

impl From<user_structs::User> for project::Author<ID> {
    fn from(from: user_structs::User) -> Self {
        Self {
            username: from.username,
            name: from.name,
            id: from.id,
            avatar_url: Some(request::Asset::by_url(from.avatar_url)),
        }
    }
}

impl From<version_structs::DependencyType> for version::DependencyType {
    fn from(from: version_structs::DependencyType) -> Self {
        match from {
            version_structs::DependencyType::Required => Self::RequiredDependency,
            version_structs::DependencyType::Optional => Self::OptionalDependency,
            version_structs::DependencyType::Incompatible => Self::Incompatible,
        }
    }
}

impl From<project_structs::ProjectType> for project::ProjectType {
    fn from(from: project_structs::ProjectType) -> Self {
        match from {
            project_structs::ProjectType::Mod => project::ProjectType::Mod,
            project_structs::ProjectType::Modpack => project::ProjectType::Modpack,
        }
    }
}

impl From<version_structs::VersionType> for version::VersionType {
    fn from(from: version_structs::VersionType) -> Self {
        match from {
            version_structs::VersionType::Alpha => Self::Alpha,
            version_structs::VersionType::Beta => Self::Beta,
            version_structs::VersionType::Release => Self::Release,
        }
    }
}

impl From<&str> for version::ModLoader {
    fn from(from: &str) -> Self {
        match from.to_lowercase().as_str() {
            "modloader" => Self::Modloader,
            "fabric" => Self::Fabric,
            "quilt" => Self::Quilt,
            "forge" => Self::Forge,
            "rift" => Self::Rift,
            _ => Self::Unknown,
        }
    }
}

impl From<project_structs::ProjectSupportRange> for project::Requirement {
    fn from(from: project_structs::ProjectSupportRange) -> Self {
        match from {
            project_structs::ProjectSupportRange::Required => Self::Required,
            project_structs::ProjectSupportRange::Optional => Self::Optional,
            project_structs::ProjectSupportRange::Unsupported => Self::Unsupported,
        }
    }
}

impl From<project_structs::ProjectStatus> for project::Status {
    fn from(from: project_structs::ProjectStatus) -> Self {
        match from {
            project_structs::ProjectStatus::Approved => Self::Approved,
            project_structs::ProjectStatus::Rejected => Self::Rejected,
            project_structs::ProjectStatus::Draft => Self::New,
            project_structs::ProjectStatus::Unlisted => Self::Unlisted,
            project_structs::ProjectStatus::Archived => Self::Abandoned,
            project_structs::ProjectStatus::Processing => Self::UnderReview,
            project_structs::ProjectStatus::Unknown => Self::Unknown,
        }
    }
}

impl From<project_structs::DonationLink> for project::DonationLink {
    fn from(from: project_structs::DonationLink) -> Self {
        Self {
            platform: from.platform,
            url: from.url,
            id: from.id,
        }
    }
}

impl From<project_structs::GalleryItem> for request::Asset {
    fn from(from: project_structs::GalleryItem) -> Self {
        Self::by_description(from.url, from.title, from.description)
    }
}
