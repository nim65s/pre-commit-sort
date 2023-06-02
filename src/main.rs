use pre_commit_sort::{PreCommit, PreCommitConfig, PreCommitHooks};
use std::env;

fn main() -> anyhow::Result<()> {
    let install = env::args().nth(1) == Some("-i".to_string());
    PreCommitConfig::main(install)?;
    PreCommitHooks::main(false)?;
    Ok(())
}
