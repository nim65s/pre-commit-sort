use indoc::indoc;
use pre_commit_config_sort::{Hook, PreCommitConfig, Repo};

#[test]
fn test_example() {
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Repo::new("https://github.com/pre-commit/pre-commit-hooks", "v2.3.0");
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(Hook::new(hook));
    }
    example.add_repo(pre_commit);

    let mut black = Repo::new("https://github.com/psf/black", "22.10.0");
    black.add_hook(Hook::new("black"));
    example.add_repo(black);

    let yaml = indoc! {"
        repos:
        - repo: https://github.com/pre-commit/pre-commit-hooks
          rev: v2.3.0
          hooks:
          - id: check-yaml
          - id: end-of-file-fixer
          - id: trailing-whitespaces
        - repo: https://github.com/psf/black
          rev: 22.10.0
          hooks:
          - id: black
        "};
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
}