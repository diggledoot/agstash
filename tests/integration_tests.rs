use assert_cmd::Command;
use predicates::prelude::*;
use tempfile::tempdir;

#[test]
fn init_creates_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("AGENTS.md");

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.current_dir(&dir)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("Created AGENTS.md"));

    assert!(file_path.exists());
    let content = std::fs::read_to_string(file_path)?;
    let expected = "# AGENTS\n\n- be concise and factual.\n- always test after changes are made.\n- create tests after a new feature is added.\n";
    assert_eq!(content, expected);

    Ok(())
}

#[test]
fn init_does_not_overwrite() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "Existing content")?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.current_dir(&dir)
        .arg("init")
        .assert()
        .success()
        .stdout(predicate::str::contains("AGENTS.md already exists"));

    let content = std::fs::read_to_string(file_path)?;
    assert_eq!(content, "Existing content");

    Ok(())
}

#[test]
fn clean_removes_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "some content")?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.current_dir(&dir)
        .arg("clean")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed AGENTS.md"));

    assert!(!file_path.exists());

    Ok(())
}

#[test]
fn clean_does_not_error_on_missing_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.current_dir(&dir)
        .arg("clean")
        .assert()
        .success()
        .stdout(predicate::str::contains("AGENTS.md does not exist"));

    Ok(())
}

#[test]
fn stash_creates_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "# AGENTS\n\n- some content\n")?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    // Set HOME to temp dir so .agstash is created there
    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("stash")
        .assert()
        .success()
        .stdout(predicate::str::contains("Stashed AGENTS.md for"));

    // Check if stash exists (dir name is the last component of temp path)
    let project_name = dir.path().file_name().unwrap().to_string_lossy();
    let stash_path = dir
        .path()
        .join(".agstash")
        .join("stashes")
        .join(format!("stash-{}.md", project_name));
    assert!(stash_path.exists());

    Ok(())
}

#[test]
fn apply_restores_file() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    // Setup stash
    let project_name = dir.path().file_name().unwrap().to_string_lossy();
    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    let stash_content = "# AGENTS\n\nStashed Content";
    std::fs::write(&stash_path, stash_content)?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("apply")
        .assert()
        .success()
        .stdout(predicate::str::contains("Applied AGENTS.md for"));

    let file_path = dir.path().join("AGENTS.md");
    assert!(file_path.exists());
    let content = std::fs::read_to_string(file_path)?;
    assert_eq!(content, stash_content);

    Ok(())
}

#[test]
fn uninstall_removes_directory() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let agstash_dir = dir.path().join(".agstash");
    std::fs::create_dir_all(&agstash_dir)?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("uninstall")
        .assert()
        .success()
        .stdout(predicate::str::contains("Removed").and(predicate::str::contains(".agstash"))); // Check for path fragment

    assert!(!agstash_dir.exists());

    Ok(())
}

#[test]
fn apply_prompts_on_existing_file_abort() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    let project_name = dir.path().file_name().unwrap().to_string_lossy();

    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "Original Content")?;

    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    let stash_content = "# AGENTS\n\nStashed Content";
    std::fs::write(&stash_path, stash_content)?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("apply")
        .write_stdin("n\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Warning").and(predicate::str::contains("Aborted")));

    let content = std::fs::read_to_string(file_path)?;
    assert_eq!(content, "Original Content");

    Ok(())
}

#[test]
fn apply_prompts_on_existing_file_overwrite() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    let project_name = dir.path().file_name().unwrap().to_string_lossy();

    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "Original Content")?;

    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    let stash_content = "# AGENTS\n\nStashed Content";
    std::fs::write(&stash_path, stash_content)?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("apply")
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Warning").and(predicate::str::contains("Applied")));

    let content = std::fs::read_to_string(file_path)?;
    assert_eq!(content, stash_content);

    Ok(())
}

#[test]
fn stash_fails_when_agents_missing() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    // Don't create AGENTS.md

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("stash")
        .assert()
        .success() // Should still return 0 exit code according to main.rs logic (it prints error and returns Ok(()))
        .stdout(
            predicate::str::contains("AGENTS.md").and(predicate::str::contains("does not exist")),
        );

    Ok(())
}

#[test]
fn apply_fails_when_stash_missing() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    let project_name = dir.path().file_name().unwrap().to_string_lossy();
    // Don't create stash

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("apply")
        .assert()
        .success() // Should still return 0 exit code
        .stdout(
            predicate::str::contains("No stash found for project")
                .and(predicate::str::contains(project_name)),
        );

    Ok(())
}

#[test]
fn stash_errors_without_project_root() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    // Note: no .git or .gitignore created here on purpose

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("stash")
        .assert()
        .failure()
        .stderr(predicate::str::contains(
            "Could not find project root (no .git or .gitignore found)",
        ));

    Ok(())
}

#[test]
fn apply_force_overwrites_without_prompt() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    let project_name = dir.path().file_name().unwrap().to_string_lossy();

    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "Original Content")?;

    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    std::fs::write(
        &stash_path,
        "# AGENTS\n\n- valid content so validation passes\n",
    )?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("apply")
        .arg("--force")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Applied AGENTS.md for")
                .and(predicate::str::contains("Warning").not()),
        );

    let content = std::fs::read_to_string(file_path)?;
    assert!(content.contains("valid content"));

    Ok(())
}

#[test]
fn stash_rejects_invalid_agents_content() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    let file_path = dir.path().join("AGENTS.md");
    // Missing "# AGENTS" header
    std::fs::write(&file_path, "Some invalid content")?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("stash")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("AGENTS.md content is invalid")
                .and(predicate::str::contains("Stash aborted")),
        );

    let project_name = dir.path().file_name().unwrap().to_string_lossy();
    let stash_path = dir
        .path()
        .join(".agstash")
        .join("stashes")
        .join(format!("stash-{}.md", project_name));
    assert!(!stash_path.exists());

    Ok(())
}

#[test]
fn apply_rejects_invalid_stash_content() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    std::fs::create_dir(dir.path().join(".git"))?;
    let project_name = dir.path().file_name().unwrap().to_string_lossy();

    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    // Missing "# AGENTS" header
    std::fs::write(&stash_path, "Invalid stash content")?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("apply")
        .assert()
        .success()
        .stdout(
            predicate::str::contains("Stash content is invalid")
                .and(predicate::str::contains("Apply aborted")),
        );

    let file_path = dir.path().join("AGENTS.md");
    assert!(!file_path.exists());

    Ok(())
}
