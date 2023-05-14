use std::path::Path;

use pre_commit_config_sort::{PreCommitConfig, PATH};

fn main() -> anyhow::Result<()> {
    if Path::new(PATH).exists() {
        println!("file exist");
        let mut data = PreCommitConfig::read()?;
        data.sort();
    }
    Ok(())
}
