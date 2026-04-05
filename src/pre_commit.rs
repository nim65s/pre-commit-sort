use std::{fs::File, path::Path};

use crate::Result;

pub trait PreCommit: serde::de::DeserializeOwned + serde::Serialize {
    const PATH: &'static str;

    fn process(&mut self, install: bool);

    fn main(install: bool) -> Result<()> {
        if Path::new(Self::PATH).exists() {
            let input = File::open(Self::PATH)?;
            let mut pre_commit: Self = yaml_serde::from_reader(input)?;

            pre_commit.process(install);

            let output = File::create(Self::PATH)?;
            yaml_serde::to_writer(output, &pre_commit)?;
        }
        Ok(())
    }
}
