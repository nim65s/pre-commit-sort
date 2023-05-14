use std::fs::File;
use std::path::Path;

use pre_commit_sort::{
    PreCommitConfig, PreCommitHooks, PRE_COMMIT_CONFIG_PATH, PRE_COMMIT_HOOKS_PATH,
};

fn main() -> anyhow::Result<()> {
    if Path::new(PRE_COMMIT_CONFIG_PATH).exists() {
        let mut pre_commit_config = PreCommitConfig::read()?;
        pre_commit_config.install();
        pre_commit_config.sort();
        pre_commit_config.write()?;
    }
    if Path::new(PRE_COMMIT_HOOKS_PATH).exists() {
        let mut pre_commit_hooks = PreCommitHooks::read()?;
        pre_commit_hooks.sort();
        let output = File::create(PRE_COMMIT_HOOKS_PATH)?;
        serde_yaml::to_writer(output, &pre_commit_hooks)?;
    }
    Ok(())
}
