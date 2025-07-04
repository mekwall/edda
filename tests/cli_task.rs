use assert_cmd::Command;
use predicates::str::contains;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// Helper to create a config file in the temp directory
fn create_config_file(temp_dir: &Path) -> std::path::PathBuf {
    let config_path = temp_dir.join("edda.toml");
    let db_path = temp_dir.join("edda.db");
    let config_content = format!(
        r#"
data_dir = "{}"
log_level = "warn"
output_format = "text"

[database]
url = "sqlite:{}"
max_connections = 5

[github]
sync_interval = 300
"#,
        temp_dir.to_string_lossy().replace('\\', "/"),
        db_path.to_string_lossy().replace('\\', "/")
    );
    fs::write(&config_path, config_content).unwrap();
    config_path
}

/// Helper to run the CLI with a custom config file
fn cli_with_config(config_path: &Path) -> Command {
    let mut cmd = Command::cargo_bin("edda").unwrap();
    cmd.arg("--config").arg(config_path);
    cmd
}

#[test]
fn test_system_init_creates_database() {
    let temp = TempDir::new().unwrap();
    let config_path = create_config_file(temp.path());
    let db_path = temp.path().join("edda.db");

    let mut cmd = cli_with_config(&config_path);
    cmd.args(["system", "init"])
        .assert()
        .success()
        .stdout(contains("Database initialized successfully"));

    assert!(db_path.exists());
}

#[test]
fn test_task_add_and_list() {
    let temp = TempDir::new().unwrap();
    let config_path = create_config_file(temp.path());

    // Init system
    let mut cmd = cli_with_config(&config_path);
    cmd.args(["system", "init"]).assert().success();

    // Add a task
    let mut cmd = cli_with_config(&config_path);
    cmd.args(["task", "add", "Integration test task"])
        .assert()
        .success()
        .stdout(contains("Created task"));

    // List tasks
    let mut cmd = cli_with_config(&config_path);
    cmd.args(["task", "list"])
        .assert()
        .success()
        .stdout(contains("Integration test task"));
}

#[test]
fn test_task_list_empty() {
    let temp = TempDir::new().unwrap();
    let config_path = create_config_file(temp.path());

    let mut cmd = cli_with_config(&config_path);
    cmd.args(["system", "init"]).assert().success();

    let mut cmd = cli_with_config(&config_path);
    cmd.args(["task", "list"])
        .assert()
        .success()
        .stdout(contains("No tasks found"));
}

#[test]
fn test_config_file_override() {
    let temp = TempDir::new().unwrap();
    let config_path = create_config_file(temp.path());
    // Test that config file is properly loaded (placeholder: system status not yet implemented)
    let mut cmd = cli_with_config(&config_path);
    cmd.args(["system", "status"])
        .assert()
        .success()
        .stdout(contains("System status not yet implemented"));
}

#[test]
fn test_task_operations_with_config() {
    let temp = TempDir::new().unwrap();
    let config_path = create_config_file(temp.path());

    // Init system
    let mut cmd = cli_with_config(&config_path);
    cmd.args(["system", "init"]).assert().success();

    // Add multiple tasks
    let mut cmd = cli_with_config(&config_path);
    cmd.args(["task", "add", "First task"]).assert().success();

    let mut cmd = cli_with_config(&config_path);
    cmd.args(["task", "add", "Second task"]).assert().success();

    // List tasks and verify both are present
    let mut cmd = cli_with_config(&config_path);
    cmd.args(["task", "list"])
        .assert()
        .success()
        .stdout(contains("First task"))
        .stdout(contains("Second task"));
}
