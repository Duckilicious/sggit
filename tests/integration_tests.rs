mod common;

use assert_cmd::Command;
use predicates::prelude::*;
use anyhow::Result;

use common::TestEnv;

#[test]
fn test_sggit_init() -> Result<()> {
    let test_env = TestEnv::new()?;
    let work_dir = test_env.work_path();
    
    let mut cmd = Command::cargo_bin("sggit")?;
    cmd.arg("init")
        .current_dir(work_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Initialized empty sggit repository"));

    assert!(work_dir.join(".git").exists(), ".git directory should exist");
    assert!(work_dir.join(".sggit").exists(), ".sggit directory should exist");
    assert!(work_dir.join(".sggit/config.json").exists(), "config.json should exist");
    
    let config_content = std::fs::read_to_string(work_dir.join(".sggit/config.json"))?;
    assert!(config_content.contains(r#""files": {}"#));
    
    Ok(())
}

#[test]
fn test_sggit_add() -> Result<()> {
    let test_env = TestEnv::new()?;
    let work_dir = test_env.work_path();
    
    let remote_file = test_env.create_remote_file("test.txt", "Hello, World!")?;
    
    Command::cargo_bin("sggit")?
        .arg("init")
        .current_dir(work_dir)
        .assert()
        .success();
    
    let mut cmd = Command::cargo_bin("sggit")?;
    cmd.arg("add")
        .arg(&remote_file)
        .current_dir(work_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Added").and(predicate::str::contains("test.txt")));

    let config_content = std::fs::read_to_string(work_dir.join(".sggit/config.json"))?;
    assert!(config_content.contains("test.txt"));
    assert!(config_content.contains(&format!("{}", remote_file.display())));
    
    Ok(())
}

#[test]
fn test_sggit_add_nonexistent_file() -> Result<()> {
    let test_env = TestEnv::new()?;
    let work_dir = test_env.work_path();
    
    Command::cargo_bin("sggit")?
        .arg("init")
        .current_dir(work_dir)
        .assert()
        .success();
    
    let mut cmd = Command::cargo_bin("sggit")?;
    cmd.arg("add")
        .arg("/nonexistent/file.txt")
        .current_dir(work_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("File does not exist"));
    
    Ok(())
}

#[test]
fn test_sggit_update() -> Result<()> {
    let test_env = TestEnv::new()?;
    let work_dir = test_env.work_path();
    
    let remote_file = test_env.create_remote_file("test.txt", "Hello, World!")?;
    
    Command::cargo_bin("sggit")?
        .arg("init")
        .current_dir(work_dir)
        .assert()
        .success();
    
    Command::cargo_bin("sggit")?
        .arg("add")
        .arg(&remote_file)
        .current_dir(work_dir)
        .assert()
        .success();
    
    let mut cmd = Command::cargo_bin("sggit")?;
    cmd.arg("update")
        .current_dir(work_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Updated test.txt"))
        .stdout(predicate::str::contains("Committed 1 updated file to git repository"));

    let local_file = work_dir.join("test.txt");
    assert!(local_file.exists());
    assert_eq!(std::fs::read_to_string(&local_file)?, "Hello, World!");
    
    // Verify git commit was created
    let git_log_output = std::process::Command::new("git")
        .arg("log")
        .arg("--oneline")
        .arg("-1")
        .current_dir(work_dir)
        .output()?;
    
    let log_str = String::from_utf8(git_log_output.stdout)?;
    assert!(log_str.contains("Update 1 file from remote locations"));
    
    Ok(())
}

#[test]
fn test_sggit_sync() -> Result<()> {
    let test_env = TestEnv::new()?;
    let work_dir = test_env.work_path();
    
    let remote_file = test_env.create_remote_file("test.txt", "Original content")?;
    
    Command::cargo_bin("sggit")?
        .arg("init")
        .current_dir(work_dir)
        .assert()
        .success();
    
    Command::cargo_bin("sggit")?
        .arg("add")
        .arg(&remote_file)
        .current_dir(work_dir)
        .assert()
        .success();
    
    Command::cargo_bin("sggit")?
        .arg("update")
        .current_dir(work_dir)
        .assert()
        .success();
    
    std::fs::write(work_dir.join("test.txt"), "Modified content")?;
    
    let mut cmd = Command::cargo_bin("sggit")?;
    cmd.arg("sync")
        .current_dir(work_dir)
        .assert()
        .success()
        .stdout(predicate::str::contains("Synced test.txt"));

    assert_eq!(std::fs::read_to_string(&remote_file)?, "Modified content");
    
    Ok(())
}