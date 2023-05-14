/// ref. <https://pre-commit.com/#pre-commit-configyaml---top-level>
use std::fs::File;

use crate::{Repo, Result};

pub static PATH: &str = ".pre-commit-config.yaml";

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct PreCommitConfig {
    repos: Vec<Repo>,
    pub default_install_hook_types: Option<Vec<String>>,
    pub default_language_version: Option<std::collections::BTreeMap<String, String>>,
    pub default_stages: Option<Vec<String>>,
    pub files: Option<String>,
    pub exclude: Option<String>,
    pub fail_fast: Option<bool>,
    pub minimum_pre_commit_version: Option<String>,
}

impl PreCommitConfig {
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

    pub fn read() -> Result<Self> {
        let input = File::open(PATH)?;
        Ok(serde_yaml::from_reader(input)?)
    }

    pub fn write(&self) -> Result<()> {
        let output = File::create(PATH)?;
        Ok(serde_yaml::to_writer(output, &self)?)
    }

    pub fn add_repo(&mut self, repo: Repo) {
        self.repos.push(repo);
    }

    pub fn sort(&mut self) {
        for repo in &mut self.repos {
            repo.sort();
        }
        self.repos.sort();
    }
}

impl Default for PreCommitConfig {
    fn default() -> Self {
        Self::new()
    }
}
