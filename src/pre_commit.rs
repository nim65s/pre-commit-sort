use std::{fs::File, path::Path};

use crate::Result;

pub trait PreCommit: serde::de::DeserializeOwned + serde::Serialize {
    const PATH: &'static str;

    fn process(&mut self);

    fn main() -> Result<()> {
        if Path::new(Self::PATH).exists() {
            let input = File::open(Self::PATH)?;
            let mut pre_commit: Self = serde_yaml::from_reader(input)?;

            pre_commit.process();

            let output = File::create(Self::PATH)?;
            serde_yaml::to_writer(output, &pre_commit)?;
        }
        Ok(())
    }
}
