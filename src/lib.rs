/// ref. https://pre-commit.com/#pre-commit-configyaml---top-level

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
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
}

impl Default for PreCommitConfig<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Repo<'a> {
    repo: &'a str,
    rev: &'a str,
    hooks: Vec<Hook<'a>>,
}

impl<'a> Repo<'a> {
    #[must_use]
    pub const fn new(repo: &'a str, rev: &'a str) -> Self {
        Self {
            repo,
            rev,
            hooks: Vec::new(),
        }
    }

    pub fn add_hook(&mut self, hook: Hook<'a>) {
        self.hooks.push(hook);
    }
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Hook<'a> {
    id: &'a str,
    pub alias: Option<&'a str>,
    pub name: Option<&'a str>,
    pub language_version: Option<&'a str>,
    pub files: Option<&'a str>,
    pub exclude: Option<&'a str>,
    pub types: Option<&'a str>,
    pub types_or: Option<&'a str>,
    pub exclude_types: Option<&'a str>,
    pub args: Option<&'a str>,
    pub stages: Option<&'a str>,
    pub additional_dependencies: Option<&'a str>,
    pub always_run: Option<bool>,
    pub verbose: Option<bool>,
    pub log_file: Option<&'a str>,
}

impl<'a> Hook<'a> {
    #[must_use]
    pub const fn new(id: &'a str) -> Self {
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
