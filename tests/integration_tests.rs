use assert_cmd::Command;

#[test]
fn test_help_output() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(std::env!("CARGO_BIN_EXE_resend"));
    let assert = cmd.arg("--help").assert();
    let output = assert.get_output();
    let stdout = std::str::from_utf8(&output.stdout)?;

    assert!(stdout.contains("Resend CLI - Manage your emails, domains, and more"));
    Ok(())
}

#[test]
fn test_config_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(std::env!("CARGO_BIN_EXE_resend"));

    // Test that the config command exists and shows help
    let assert = cmd.arg("config").arg("--help").assert();
    let output = assert.get_output();
    let stdout = std::str::from_utf8(&output.stdout)?;

    assert!(stdout.contains("Configure the Resend CLI"));
    Ok(())
}

#[test]
fn test_emails_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(std::env!("CARGO_BIN_EXE_resend"));

    // Test that the emails command exists and shows help
    let assert = cmd.arg("emails").arg("--help").assert();
    let output = assert.get_output();
    let stdout = std::str::from_utf8(&output.stdout)?;

    assert!(stdout.contains("Manage emails"));
    Ok(())
}

#[test]
fn test_api_keys_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(std::env!("CARGO_BIN_EXE_resend"));

    // Test that the api-keys command exists and shows help
    let assert = cmd.arg("api-keys").arg("--help").assert();
    let output = assert.get_output();
    let stdout = std::str::from_utf8(&output.stdout)?;

    assert!(stdout.contains("Manage API keys"));
    Ok(())
}

#[test]
fn test_domains_subcommand() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::new(std::env!("CARGO_BIN_EXE_resend"));

    // Test that the domains command exists and shows help
    let assert = cmd.arg("domains").arg("--help").assert();
    let output = assert.get_output();
    let stdout = std::str::from_utf8(&output.stdout)?;

    assert!(stdout.contains("Manage domains"));
    Ok(())
}