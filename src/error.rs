#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),

    #[error("serde yaml error: {0}")]
    SerdeYaml(#[from] serde_yaml::Error),
}

pub type Result<T> = core::result::Result<T, Error>;
