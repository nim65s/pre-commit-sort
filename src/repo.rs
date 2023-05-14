/// ref. <https://pre-commit.com/#pre-commit-configyaml---repos>

use crate::Hook;

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

    pub fn sort(&mut self) {
        self.hooks.sort();
    }
}
