/// ref. <https://pre-commit.com/#pre-commit-configyaml---hooks>

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hook {
    id: String,
    alias: Option<String>,
    name: Option<String>,
    language_version: Option<String>,
    files: Option<String>,
    exclude: Option<String>,
    types: Option<String>,
    types_or: Option<String>,
    exclude_types: Option<String>,
    args: Option<Vec<String>>,
    stages: Option<String>,
    additional_dependencies: Option<Vec<String>>,
    always_run: Option<bool>,
    verbose: Option<bool>,
    log_file: Option<String>,
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
