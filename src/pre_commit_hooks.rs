use crate::{DeclareHook, PreCommit};

pub type PreCommitHooks = Vec<DeclareHook>;

impl PreCommit for PreCommitHooks {
    const PATH: &'static str = ".pre-commit-hooks.yaml";

    fn process(&mut self, _install: bool) {
        self.sort();
    }
}
