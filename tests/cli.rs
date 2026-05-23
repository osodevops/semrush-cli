use assert_cmd::cargo::cargo_bin_cmd;
use assert_cmd::Command;
use predicates::str::contains;

fn semrush_cmd() -> Command {
    let mut cmd = cargo_bin_cmd!("semrush");
    cmd.env_remove("SEMRUSH_API_KEY")
        .env_remove("SEMRUSH_CONFIG")
        .env_remove("SEMRUSH_OUTPUT")
        .env_remove("SEMRUSH_DATABASE");
    cmd
}

#[test]
fn dry_run_estimates_without_api_key() {
    semrush_cmd()
        .args(["domain", "overview", "example.com", "--dry-run"])
        .assert()
        .success()
        .stdout(contains("Estimated cost"));
}

#[test]
fn api_commands_fail_with_documented_auth_exit_code_without_api_key() {
    semrush_cmd()
        .args(["domain", "overview", "example.com"])
        .assert()
        .code(1)
        .stderr(contains("AUTH_FAILED"));
}

#[test]
fn batch_estimate_accepts_readme_style_command_names_without_api_key() {
    let dir = tempfile::tempdir().expect("tempdir");
    let recipe_path = dir.path().join("recipe.toml");
    std::fs::write(
        &recipe_path,
        r#"
[meta]
name = "Smoke"

[[steps]]
command = "domain_overview"
output_key = "overview"
[steps.args]
domain = "example.com"
"#,
    )
    .expect("write recipe");

    semrush_cmd()
        .args(["batch", "estimate", recipe_path.to_str().unwrap()])
        .assert()
        .success()
        .stdout(contains("Step 1 (domain_overview)"))
        .stdout(contains("Total estimated cost"));
}
