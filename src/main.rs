/*
use std::fs::File;
use std::io::{BufRead, BufReader, Error, Write};
use std::path::Path;

use pre_commit_config_sort::PreCommitConfig;
*/

fn main() -> anyhow::Result<()> {
    let _path = ".pre-commit-config.yaml";

    /*
    if Path::new(path).exists() {
        println!("file exist");
        let input = File::open(path)?;
        //let mut data: PreCommitConfig = serde_yaml::from_reader(input)?;
        data.sort();
        let input = File::open(path)?;
        serde_yaml::to_writer(input, &data)?;
    }
    */
    Ok(())
}
