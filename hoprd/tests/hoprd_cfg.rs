use std::process::Command;

/// `hoprd-cfg --validate` must surface the deprecation warnings emitted while parsing
/// a config that still carries removed `funding` options.
#[test]
fn validate_reports_deprecated_funding_keys_on_stderr() -> anyhow::Result<()> {
    let fixture = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/hoprd-legacy-funding.cfg.yaml"
    );

    let output = Command::new(env!("CARGO_BIN_EXE_hoprd-cfg"))
        .args(["--validate-args", "--", "--configurationFilePath", fixture])
        .args(["--password", "a-securely-provided-password"])
        .env_remove("RUST_LOG")
        .output()?;

    let stderr = String::from_utf8(output.stderr)?;
    assert!(output.status.success(), "validation failed: {stderr}");
    assert!(
        stderr.contains("min_safe_capacity_required"),
        "stderr: {stderr}"
    );
    assert!(stderr.contains("stop_when_unfunded"), "stderr: {stderr}");
    assert!(
        output.stdout.is_empty(),
        "warnings must not go to stdout: {}",
        String::from_utf8_lossy(&output.stdout)
    );

    Ok(())
}
