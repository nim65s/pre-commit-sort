use crate::Repo;

/// ref. https://pre-commit.com/#pre-commit-configyaml---top-level

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PreCommitConfig<'a> {
    repos: Vec<Repo<'a>>,
    pub default_install_hook_types: Option<Vec<&'a str>>,
    pub default_language_version: Option<std::collections::BTreeMap<&'a str, &'a str>>,
    pub default_stages: Option<Vec<&'a str>>,
    pub files: Option<&'a str>,
    pub exclude: Option<&'a str>,
    pub fail_fast: Option<bool>,
    pub minimum_pre_commit_version: Option<&'a str>,
}

impl<'a> PreCommitConfig<'a> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            repos: Vec::new(),
            default_install_hook_types: None,
            default_language_version: None,
            default_stages: None,
            files: None,
            exclude: None,
            fail_fast: None,
            minimum_pre_commit_version: None,
        }
    }

    pub fn add_repo(&mut self, repo: Repo<'a>) {
        self.repos.push(repo);
    }

    pub fn sort(&mut self) {
        for repo in &mut self.repos {
            repo.sort();
        }
        self.repos.sort();
    }
}

impl Default for PreCommitConfig<'_> {
    fn default() -> Self {
        Self::new()
    }
}
