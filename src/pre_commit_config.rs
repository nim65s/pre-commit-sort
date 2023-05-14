/// ref. <https://pre-commit.com/#pre-commit-configyaml---top-level>
use std::collections::BTreeMap;
use std::fs::File;

use crate::{Hook, Repo, Result, CI};

pub static PRE_COMMIT_CONFIG_PATH: &str = ".pre-commit-config.yaml";

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct PreCommitConfig {
    ci: Option<CI>,
    repos: Vec<Repo>,
    default_install_hook_types: Option<Vec<String>>,
    default_language_version: Option<BTreeMap<String, String>>,
    default_stages: Option<Vec<String>>,
    files: Option<String>,
    exclude: Option<String>,
    fail_fast: Option<bool>,
    minimum_pre_commit_version: Option<String>,
}

impl PreCommitConfig {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ci: None,
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
        let input = File::open(PRE_COMMIT_CONFIG_PATH)?;
        Ok(serde_yaml::from_reader(input)?)
    }

    pub fn write(&self) -> Result<()> {
        let output = File::create(PRE_COMMIT_CONFIG_PATH)?;
        Ok(serde_yaml::to_writer(output, &self)?)
    }

    pub fn add_repo(&mut self, repo: Repo) {
        self.repos.push(repo);
    }

    /// Sort and deduplicate repos and their hooks
    pub fn sort(&mut self) {
        for repo in &mut self.repos {
            repo.sort();
        }
        self.repos.sort();
        self.repos.dedup();
        self.dedup_rev();
    }

    /// If two repos differ only by their rev, keep only the latest
    fn dedup_rev(&mut self) {
        while let Some(i) = self.find_first_rev_dup() {
            self.repos.remove(i);
        }
    }

    fn find_first_rev_dup(&self) -> Option<usize> {
        for (i, w) in self.repos.windows(2).enumerate() {
            if w[0].equal_but_rev(&w[1]) {
                return Some(i);
            }
        }
        None
    }

    /// Install pre-commit-sort in this .pre-commit-config.yaml
    pub fn install(&mut self) {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let mut repo = Repo::new(
            env!("CARGO_PKG_REPOSITORY").to_string(),
            format!("v{VERSION}"),
        );
        let hook = Hook::new(env!("CARGO_PKG_NAME").to_string());
        repo.add_hook(hook);
        self.add_repo(repo);
    }
}

impl Default for PreCommitConfig {
    fn default() -> Self {
        Self::new()
    }
}
