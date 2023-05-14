/// ref. <https://pre-commit.com/#pre-commit-configyaml---hooks>

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hook {
    id: String,
    pub alias: Option<String>,
    pub name: Option<String>,
    pub language_version: Option<String>,
    pub files: Option<String>,
    pub exclude: Option<String>,
    pub types: Option<String>,
    pub types_or: Option<String>,
    pub exclude_types: Option<String>,
    pub args: Option<Vec<String>>,
    pub stages: Option<String>,
    pub additional_dependencies: Option<Vec<String>>,
    pub always_run: Option<bool>,
    pub verbose: Option<bool>,
    pub log_file: Option<String>,
}

impl Hook {
    #[must_use]
    pub const fn new(id: String) -> Self {
        Self {
            id,
            alias: None,
            name: None,
            language_version: None,
            files: None,
            exclude: None,
            types: None,
            types_or: None,
            exclude_types: None,
            args: None,
            stages: None,
            additional_dependencies: None,
            always_run: None,
            verbose: None,
            log_file: None,
        }
    }
}
