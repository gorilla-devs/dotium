pub mod converters;

use crate::{
    project::{Author, Project},
    request::Asset,
    version::Version,
    Container, Platform,
};
use async_trait::async_trait;
use ferinth::Ferinth;

pub type ID = String;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{}", .0)]
    FerinthError(#[from] ferinth::Error),
    #[error("{}", .0)]
    Unimplemented(#[from] crate::UnimplementedError),
}

#[derive(Default)]
pub struct ModrinthContainer {
    ferinth: Ferinth,
}

#[async_trait]
impl Container<Error, ID> for ModrinthContainer {
    fn new() -> Self {
        Self::default()
    }

    fn get_platform() -> Platform {
        Platform {
            name: "Modrinth".into(),
            short_name: "mr".into(),
            logo: Asset::by_description(
                url::Url::parse(
                    "https://github.com/modrinth/knossos/blob/master/assets/images/logo.svg",
                )
                .expect("URL parsing of a constant has unexpectedly failed!"),
                Some("logo.svg".into()),
                Some("Modrinth platform logo".into()),
            ),
        }
    }

    async fn get_project(&self, id: &ID) -> Result<Project<ID>, Error> {
        Ok(self.ferinth.get_project(id).await.map(Into::into)?)
    }

    async fn get_projects(&self, ids: Vec<&ID>) -> Result<Vec<Project<ID>>, Error> {
        Ok(self
            .ferinth
            .get_multiple_projects(
                &ids.into_iter()
                    .map(|string| string as &str)
                    .collect::<Vec<_>>(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn get_project_body(&self, project_id: &ID) -> Result<String, Error> {
        Ok(self.ferinth.get_project(project_id).await?.body)
    }

    async fn get_project_authors(&self, team_id: &ID) -> Result<Vec<Author<ID>>, Error> {
        Ok(self
            .ferinth
            .list_team_members(team_id)
            .await?
            .into_iter()
            .map(|a| a.user.into())
            .collect())
    }

    async fn get_project_dependencies(
        &self,
        project_id: &ID,
    ) -> Result<(Vec<Project<ID>>, Vec<Version<ID>>), Error> {
        let raw = self.ferinth.get_project_dependencies(project_id).await?;
        Ok((
            raw.projects.into_iter().map(Into::into).collect(),
            raw.versions.into_iter().map(Into::into).collect(),
        ))
    }

    async fn get_versions(&self, ids: Vec<(&ID, &ID)>) -> Result<Vec<Version<ID>>, Error> {
        Ok(self
            .ferinth
            .get_multiple_versions(
                &ids.into_iter()
                    .map(|(_, string)| string as &str)
                    .collect::<Vec<_>>(),
            )
            .await?
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>())
    }

    async fn get_project_versions(&self, project_id: &ID) -> Result<Vec<Version<ID>>, Error> {
        Ok(self
            .ferinth
            .list_versions(project_id)
            .await?
            .into_iter()
            .map(Into::into)
            .collect())
    }

    async fn get_version(&self, _: &ID, id: &ID) -> Result<Version<ID>, Error> {
        Ok(self.ferinth.get_version(id).await?.into())
    }
}
