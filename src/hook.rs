#[serde_with::skip_serializing_none]
#[derive(serde::Serialize, serde::Deserialize, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Hook<'a> {
    id: &'a str,
    pub alias: Option<&'a str>,
    pub name: Option<&'a str>,
    pub language_version: Option<&'a str>,
    pub files: Option<&'a str>,
    pub exclude: Option<&'a str>,
    pub types: Option<&'a str>,
    pub types_or: Option<&'a str>,
    pub exclude_types: Option<&'a str>,
    pub args: Option<&'a str>,
    pub stages: Option<&'a str>,
    pub additional_dependencies: Option<&'a str>,
    pub always_run: Option<bool>,
    pub verbose: Option<bool>,
    pub log_file: Option<&'a str>,
}

impl<'a> Hook<'a> {
    #[must_use]
    pub const fn new(id: &'a str) -> Self {
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
