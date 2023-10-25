//! ref. <https://pre-commit.com/#creating-new-hooks>

#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
pub struct DeclareHook {
    id: String,
    name: String,
    entry: String,
    language: String,
    files: Option<String>,
    exclude: Option<String>,
    types: Option<Vec<String>>,
    types_or: Option<Vec<String>>,
    exclude_types: Option<String>,
    always_run: Option<bool>,
    fail_fast: Option<bool>,
    verbose: Option<bool>,
    pass_filenames: Option<bool>,
    require_serial: Option<bool>,
    description: Option<String>,
    language_version: Option<String>,
    minimum_pre_commit_version: Option<String>,
    args: Option<Vec<String>>,
    stages: Option<Vec<String>>,
}

impl DeclareHook {
    #[must_use]
    pub const fn new(id: String, name: String, entry: String, language: String) -> Self {
        Self {
            id,
            name,
            entry,
            language,
            files: None,
            exclude: None,
            types: None,
            types_or: None,
            exclude_types: None,
            always_run: None,
            fail_fast: None,
            verbose: None,
            pass_filenames: None,
            require_serial: None,
            description: None,
            language_version: None,
            minimum_pre_commit_version: None,
            args: None,
            stages: None,
        }
    }
}
