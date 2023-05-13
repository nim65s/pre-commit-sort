/// ref. https://pre-commit.com/#pre-commit-configyaml---top-level

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
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
    pub fn add_repo(&mut self, repo: Repo) {
        self.repos.push(repo);
    }
}

impl Default for PreCommitConfig {
    fn default() -> Self {
        Self::new()
    }
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Repo {
    repo: String,
    rev: String,
    hooks: Vec<Hook>,
}

impl Repo {
    #[must_use]
    pub const fn new(repo: String, rev: String) -> Self {
        Self {
            repo,
            rev,
            hooks: Vec::new(),
        }
    }

    pub fn add_hook(&mut self, hook: Hook) {
        self.hooks.push(hook);
    }
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Hook {
    id: String,
    pub alias: Option<String>,
    pub name: Option<String>,
    pub language_version: Option<String>,
    pub files: Option<String>,
    pub exclude: Option<String>,
    pub types: Option<String>,
    pub types_or: Option<String>,
    pub exclude_types: Option<String>,
    pub args: Option<String>,
    pub stages: Option<String>,
    pub additional_dependencies: Option<String>,
    pub always_run: Option<bool>,
    pub verbose: Option<bool>,
    pub log_file: Option<String>,
}

impl Hook {
    #[must_use]
    pub const fn new(id: String) -> Self {
        Self {
            id,
            alias: None,
            name: None,
            language_version: None,
            files: None,
            exclude: None,
            types: None,
            types_or: None,
            exclude_types: None,
            args: None,
            stages: None,
            additional_dependencies: None,
            always_run: None,
            verbose: None,
            log_file: None,
        }
    }
}
