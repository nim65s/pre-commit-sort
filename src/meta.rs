/// ref. <https://pre-commit.com/#meta-hooks>
use crate::ConfigHook;

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Meta {
    repo: String,
    hooks: Vec<ConfigHook>,
}

impl Meta {
    #[must_use]
    pub fn new() -> Self {
        Self {
            repo: String::from("meta"),
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
}

impl Default for Meta {
    fn default() -> Self {
        Self::new()
    }
}
