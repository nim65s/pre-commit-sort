/// ref. <https://pre-commit.com/#repository-local-hooks>
use crate::DeclareHook;

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct Local {
    repo: String,
    hooks: Vec<DeclareHook>,
}

impl Local {
    #[must_use]
    pub fn new() -> Self {
        Self {
            repo: String::from("local"),
            hooks: Vec::new(),
        }
    }

    pub fn add_hook(&mut self, hook: DeclareHook) {
        self.hooks.push(hook);
    }

    pub fn sort(&mut self) {
        self.hooks.sort();
        self.hooks.dedup();
    }
}

impl Default for Local {
    fn default() -> Self {
        Self::new()
    }
}
