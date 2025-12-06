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
    let expected = r#"# AGENTS

- be concise and factual.
- always test after changes are made.
- create tests after a new feature is added.
"#;
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
    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "some content")?;

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
    // Setup stash
    let project_name = dir.path().file_name().unwrap().to_string_lossy();
    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    std::fs::write(&stash_path, "Stashed Content")?;

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
    assert_eq!(content, "Stashed Content");

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
fn list_shows_stashed_projects() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    std::fs::write(stash_dir.join("stash-myproject.md"), "content")?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("list")
        .assert()
        .success()
        .stdout(predicate::str::contains("myproject"));

    Ok(())
}

#[test]
fn apply_prompts_on_existing_file_abort() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempdir()?;
    let project_name = dir.path().file_name().unwrap().to_string_lossy();

    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "Original Content")?;

    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    std::fs::write(&stash_path, "Stashed Content")?;

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
    let project_name = dir.path().file_name().unwrap().to_string_lossy();

    let file_path = dir.path().join("AGENTS.md");
    std::fs::write(&file_path, "Original Content")?;

    let stash_dir = dir.path().join(".agstash").join("stashes");
    std::fs::create_dir_all(&stash_dir)?;
    let stash_path = stash_dir.join(format!("stash-{}.md", project_name));
    std::fs::write(&stash_path, "Stashed Content")?;

    let mut cmd = Command::new(env!("CARGO_BIN_EXE_agstash"));

    cmd.env("HOME", dir.path())
        .current_dir(&dir)
        .arg("apply")
        .write_stdin("y\n")
        .assert()
        .success()
        .stdout(predicate::str::contains("Warning").and(predicate::str::contains("Applied")));

    let content = std::fs::read_to_string(file_path)?;
    assert_eq!(content, "Stashed Content");

    Ok(())
}
