/// ref. <https://pre-commit.com/#pre-commit-configyaml---repos>
use crate::ConfigHook;

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Remote {
    repo: String,
    rev: String,
    hooks: Vec<ConfigHook>,
}

impl Remote {
    #[must_use]
    pub const fn new(repo: String, rev: String) -> Self {
        Self {
            repo,
            rev,
            hooks: Vec::new(),
        }
    }

    pub fn add_hook(&mut self, hook: ConfigHook) {
        self.hooks.push(hook);
    }

    pub fn sort(&mut self) {
        self.hooks.sort();
        self.hooks.dedup();
    }

    #[must_use]
    pub fn equal_but_rev(&self, other: &Self) -> bool {
        self.repo == other.repo && self.hooks == other.hooks
    }
}
