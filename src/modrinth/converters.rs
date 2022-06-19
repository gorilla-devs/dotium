use crate::{project, version};
use ferinth::structures::*;

impl Into<project::Project> for project_structs::Project {
    fn into(project: project_structs::Project) -> project::Project {
        project::Project {
            id: project.id,
            slug: project.slug,
            project_type: project.project_type.into(),
            name: project.title,
            description: project.description,
            links: project::Links {
                github: project.source_url,
                issues: project.issues_url,
                wiki: project.wiki_url,
                discord: project.discord_url,
                donations: project
                    .donation_urls
                    .into_iter()
                    .map(|thing|)
                    .and_then(|i| Some(i.into_iter().map(|d| donation_url(d)).collect())),
            },
            requirements: project::ProjectRequirements {
                server: project_support(project.server_side),
                client: project_support(project.client_side),
            },
            categories: project.categories,
            downloads: project.downloads,
            followers: project.followers,
            icon: project
                .icon_url
                .and_then(|i| Some(Asset::by_url(i.as_str()))),
            status: status(project.status),
            published: project.published,
            updated: project.updated,
            created: project.published,
            gallery: project
                .gallery
                .into_iter()
                .map(|i| gallery_item(i))
                .collect(),
            allows_distribution: project.license.id != "custom".to_string()
                && project.license.id != "arr".to_string(),
            team_id: project.team,
        }
    }
}

// pub fn version(raw: version_structs::Version) -> version::Version {
//     version::Version {
//         id: raw.id,
//         name: raw.name,
//         identifier: raw.version_number,
//         project_id: raw.project_id,
//         files: raw
//             .files
//             .into_iter()
//             .map(|f| Asset {
//                 url: f.url,
//                 name: Some(f.filename),
//                 description: None,
//                 headers: std::collections::HashMap::new(),
//                 request_type: dotium::request::RequestType::GET,
//                 hash: Some(AssetHash {
//                     hash: f.hashes.sha512.unwrap(), // TODO edit in Ferinth, modrinth generated hashes for old projects
//                     algorithm: dotium::request::HashAlgorithm::Sha512,
//                 }),
//                 size: Some(f.size.try_into().unwrap_or_default()), //TODO Edit in Ferinth
//             })
//             .collect(),
//         downloads: raw.downloads,
//         loaders: raw.loaders.iter().map(|l| mod_loader(l.as_str())).collect(),
//         game_versions: raw.game_versions,
//         published: raw.date_published,
//         version_type: version_type(raw.version_type),
//         dependencies: raw
//             .dependencies
//             .into_iter()
//             .map(|d| version::VersionDependency {
//                 project_id: d.project_id,
//                 version: d.version_id,
//                 dependency_type: dependency_type(d.dependency_type),
//             })
//             .collect(),
//     }
// }

// pub fn author(raw: user_structs::User) -> project::Author {
//     Author {
//         username: raw.username.clone(),
//         name: raw.name.unwrap_or(raw.username),
//         id: raw.id,
//         avatar_url: Some(Asset::by_url(raw.avatar_url.as_str())),
//     }
// }

// pub fn dependency_type(raw: version_structs::DependencyType) -> version::DependencyType {
//     match raw {
//         version_structs::DependencyType::Required => version::DependencyType::RequiredDependency,
//         version_structs::DependencyType::Optional => version::DependencyType::OptionalDependency,
//         version_structs::DependencyType::Incompatible => version::DependencyType::Incompatible,
//     }
// }

impl Into<project::ProjectType> for project_structs::ProjectType {
    fn into(self) -> project::ProjectType {
        match self {
            project_structs::ProjectType::Mod => project::ProjectType::Mod,
            project_structs::ProjectType::Modpack => project::ProjectType::Modpack,
        }
    }
}

// pub fn version_type(raw: version_structs::VersionType) -> version::VersionType {
//     match raw {
//         version_structs::VersionType::Alpha => version::VersionType::Alpha,
//         version_structs::VersionType::Beta => version::VersionType::Beta,
//         version_structs::VersionType::Release => version::VersionType::Release,
//     }
// }

// pub fn mod_loader(raw: &str) -> version::ModLoader {
//     match raw {
//         "quilt" => version::ModLoader::Quilt,
//         "fabric" => version::ModLoader::Fabric,
//         "forge" => version::ModLoader::Forge,
//         "rift" => version::ModLoader::Rift,
//         "modloader" => version::ModLoader::Modloader,
//         _ => version::ModLoader::Unkown,
//     }
// }

// pub fn project_support(raw: project_structs::ProjectSupportRange) -> project::Requirement {
//     match raw {
//         project_structs::ProjectSupportRange::Required => project::Requirement::Required,
//         project_structs::ProjectSupportRange::Optional => project::Requirement::Optional,
//         project_structs::ProjectSupportRange::Unsupported => project::Requirement::Unsupported,
//     }
// }

// pub fn status(raw: project_structs::ProjectStatus) -> project::Status {
//     match raw {
//         project_structs::ProjectStatus::Approved => project::Status::Approved,
//         project_structs::ProjectStatus::Rejected => project::Status::Rejected,
//         project_structs::ProjectStatus::Draft => project::Status::New,
//         project_structs::ProjectStatus::Unlisted => project::Status::Unlisted,
//         project_structs::ProjectStatus::Archived => project::Status::Abandoned,
//         project_structs::ProjectStatus::Processing => project::Status::UnderReview,
//         project_structs::ProjectStatus::Unknown => project::Status::Deleted,
//     }
// }

impl Into<project::DonationLink> for project_structs::DonationLink {
    fn into(self) -> project::DonationLink {
        project::DonationLink {
            platform: self.platform,
            url: self.url,
            id: self.id,
        }
    }
}

// pub fn gallery_item(raw: project_structs::GalleryItem) -> Asset {
//     Asset {
//         url: raw.url,
//         name: raw.title,
//         description: raw.description,
//         headers: std::collections::HashMap::new(),
//         request_type: dotium::request::RequestType::GET,
//         hash: None,
//         size: None,
//     }
// }
