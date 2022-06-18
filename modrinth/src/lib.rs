use async_trait::async_trait;
use dotium::{
    project::{Author, Project},
    request::Asset,
    version::Version,
    DotiumTrait, Platform,
};
use ferinth::Ferinth;

pub mod converters;

pub(crate) type Result<T> = std::result::Result<T, ferinth::Error>;
pub(crate) type TraitResult<T> =
    std::result::Result<T, Box<(dyn std::error::Error + Send + 'static)>>;

pub fn box_err<T>(result: Result<T>) -> TraitResult<T> {
    match result {
        Ok(i) => Ok(i),
        Err(i) => Err(Box::new(i)),
    }
}

pub struct ModrinthImpl {
    ferinth: Ferinth,
}

#[async_trait]
impl DotiumTrait for ModrinthImpl {
    fn new() -> Self {
        Self {
            ferinth: Ferinth::default(),
        }
    }

    fn get_platform() -> Platform {
        Platform {
            name: "Modrinth".to_string(),
            short_name: "Mr".to_string(),
            logo: Asset::by_description(
                "https://github.com/modrinth/knossos/blob/master/assets/images/logo.svg",
                "logo.svg",
                "Full Modrinth platform logo",
            ),
        }
    }

    async fn get_project(&self, id: &str) -> TraitResult<Project> {
        Ok(converters::project(box_err(
            self.ferinth.get_project(id).await,
        )?))
    }

    async fn get_projects(&self, ids: Vec<&str>) -> TraitResult<Vec<dotium::project::Project>> {
        Ok(box_err(self.ferinth.get_multiple_projects(&ids).await)?
            .into_iter()
            .map(|p| converters::project(p))
            .collect())
    }

    async fn get_project_body(&self, project_id: &str) -> TraitResult<String> {
        Ok(box_err(self.ferinth.get_project(project_id).await)?.body)
    }

    async fn get_project_authors(&self, team_id: &str) -> TraitResult<Vec<Author>> {
        Ok(box_err(self.ferinth.list_team_members(team_id).await)?
            .into_iter()
            .map(|a| converters::author(a.user))
            .collect())
    }

    async fn get_project_dependencies(
        &self,
        project_id: &str,
    ) -> TraitResult<(Vec<Project>, Vec<Version>)> {
        let raw = box_err(self.ferinth.get_project_dependencies(&project_id).await)?;
        Ok((
            raw.projects
                .iter()
                .map(|i| converters::project(i.clone()))
                .collect(),
            raw.versions
                .iter()
                .map(|i| converters::version(i.clone()))
                .collect(),
        ))
    }

    async fn get_project_version(
        &self,
        _: &str,
        id: &str,
    ) -> TraitResult<dotium::version::Version> {
        Ok(converters::version(box_err(
            self.ferinth.get_version(id).await,
        )?))
    }

    async fn get_versions(&self, project_id: &str) -> TraitResult<Vec<dotium::version::Version>> {
        Ok(box_err(self.ferinth.list_versions(&project_id).await)?
            .into_iter()
            .map(|v| converters::version(v))
            .collect())
    }

    async fn search(
        &self,
        _query: &str,
        _project_type: Option<&str>,
        _mc_version: Vec<&str>,
        _modloader: &str,
        _category: &str,
    ) -> TraitResult<Vec<dotium::project::Project>> {
        Err(Box::new(dotium::Error::Unimplemented))
    }

    async fn get_project_versions(
        &self,
        ids: Vec<(&str, &str)>,
    ) -> TraitResult<Vec<dotium::version::Version>> {
        let mut deps = Vec::new();
        for id in ids {
            deps.push(converters::version(box_err(
                self.ferinth.get_version(id.1).await,
            )?));
        }
        Ok(deps)
    }
}
