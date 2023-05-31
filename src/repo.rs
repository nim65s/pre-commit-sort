use crate::{Local, Remote};

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub enum Repo {
    Remote(Remote),
    Local(Local),
}

// TODO: remove that
impl Repo {
    pub fn sort(&mut self) {
        match self {
            Self::Remote(repo) => repo.sort(),
            Self::Local(repo) => repo.sort(),
        }
    }

    pub fn equal_but_rev(&self, other: &Repo) -> bool {
        match (self, other) {
            (Self::Remote(a), Self::Remote(b)) => a.equal_but_rev(b),
            (Self::Local(a), Self::Local(b)) => a == b,
            _ => false,
        }
    }
}
