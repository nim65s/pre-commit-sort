use indoc::indoc;
use pre_commit_sort::{Hook, PreCommitConfig, Repo};

#[test]
fn test_serialize() {
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Repo::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(Hook::new(hook.to_string()));
    }
    example.add_repo(pre_commit);

    let mut black = Repo::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(Hook::new("black".to_string()));
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

#[test]
fn test_sort() {
    let mut example = PreCommitConfig::new();

    let mut black = Repo::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(Hook::new("black".to_string()));
    example.add_repo(black);

    let mut pre_commit = Repo::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["end-of-file-fixer", "check-yaml", "trailing-whitespaces"] {
        pre_commit.add_hook(Hook::new(hook.to_string()));
    }
    example.add_repo(pre_commit);

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
    assert_ne!(serde_yaml::to_string(&example).unwrap(), yaml);
    example.sort();
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
}

#[test]
fn test_deserialize() {
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Repo::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(Hook::new(hook.to_string()));
    }
    example.add_repo(pre_commit);

    let mut black = Repo::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(Hook::new("black".to_string()));
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
    assert_eq!(example, serde_yaml::from_str(yaml).unwrap());
}

#[test]
fn test_dedup() {
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Repo::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(Hook::new(hook.to_string()));
    }
    example.add_repo(pre_commit);

    let mut black = Repo::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(Hook::new("black".to_string()));
    example.add_repo(black.clone());
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
    assert_ne!(serde_yaml::to_string(&example).unwrap(), yaml);
    example.sort();
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
}

#[test]
fn test_install() {
    let mut example = PreCommitConfig::new();

    let mut pre_commit = Repo::new(
        "https://github.com/pre-commit/pre-commit-hooks".to_string(),
        "v2.3.0".to_string(),
    );
    for hook in ["check-yaml", "end-of-file-fixer", "trailing-whitespaces"] {
        pre_commit.add_hook(Hook::new(hook.to_string()));
    }
    example.add_repo(pre_commit);

    let mut black = Repo::new(
        "https://github.com/psf/black".to_string(),
        "22.10.0".to_string(),
    );
    black.add_hook(Hook::new("black".to_string()));
    example.add_repo(black);

    example.install();

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
        - repo: https://github.com/nim65s/pre-commit-sort
          rev: 0.1.0
          hooks:
          - id: pre-commit-sort
        "};
    assert_eq!(serde_yaml::to_string(&example).unwrap(), yaml);
}
