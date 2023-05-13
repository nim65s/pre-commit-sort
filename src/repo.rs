use crate::Hook;

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
