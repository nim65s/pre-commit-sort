use pre_commit_sort::{PreCommit, PreCommitConfig, PreCommitHooks};

fn main() -> anyhow::Result<()> {
    PreCommitConfig::main()?;
    PreCommitHooks::main()?;
    Ok(())
}
