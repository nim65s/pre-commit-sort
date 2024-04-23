//! ref. <https://pre-commit.ci/#configuration>

#[allow(non_camel_case_types)]
#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub enum CIAutoupdateSchedule {
    weekly,
    monthly,
    quarterly,
}

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct CI {
    autofix_commit_msg: Option<String>,
    autofix_prs: Option<bool>,
    autoupdate_branch: Option<String>,
    autoupdate_commit_msg: Option<String>,
    autoupdate_schedule: Option<CIAutoupdateSchedule>,
    skip: Option<Vec<String>>,
    submodules: Option<bool>,
}
