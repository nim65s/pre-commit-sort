use crate::{Local, Meta, Remote};

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
#[serde(untagged)]
pub enum Repo {
    Remote(Remote),
    Local(Local),
    Meta(Meta),
}

// TODO: remove that
impl Repo {
    pub fn sort(&mut self) {
        match self {
            Self::Remote(repo) => repo.sort(),
            Self::Local(repo) => repo.sort(),
            Self::Meta(repo) => repo.sort(),
        }
    }

    #[must_use]
    pub fn equal_but_rev(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Remote(a), Self::Remote(b)) => a.equal_but_rev(b),
            (Self::Local(a), Self::Local(b)) => a == b,
            (Self::Meta(a), Self::Meta(b)) => a == b,
            _ => false,
        }
    }
}
