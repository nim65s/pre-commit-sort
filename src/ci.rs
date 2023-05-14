/// ref. <https://pre-commit.ci/#configuration>

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CI {
    autofix_commit_msg: Option<String>,
    autofix_prs: Option<bool>,
    autoupdate_branch: Option<String>,
    autoupdate_commit_msg: Option<String>,
    autoupdate_schedule: Option<String>,
    skip: Option<Vec<String>>,
    submodules: Option<bool>,
}
