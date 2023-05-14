use std::path::Path;

use pre_commit_config_sort::{PreCommitConfig, PATH};

fn main() -> anyhow::Result<()> {
    if Path::new(PATH).exists() {
        let mut pre_commit_config = PreCommitConfig::read()?;
        pre_commit_config.sort();
        pre_commit_config.write()?;
    }
    Ok(())
}
