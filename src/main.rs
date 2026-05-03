use anyhow::{anyhow, bail, Context, Result};
use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Parser)]
#[command(
    name = "fledge-gitleaks",
    version,
    about = "Scan your repo for committed secrets via gitleaks"
)]
struct Cli {
    #[command(subcommand)]
    command: Sub,
}

#[derive(clap::Subcommand)]
enum Sub {
    /// Scan and pretty-print findings (always exits 0)
    Scan {
        /// Scan only staged changes — useful in pre-commit hooks
        #[arg(long)]
        staged: bool,
    },
    /// Scan and exit non-zero if any findings (CI mode)
    Check {
        #[arg(long)]
        staged: bool,
    },
    /// Install a pre-commit hook that runs `gitleaks check --staged`
    InstallHook,
    /// Remove the fledge-installed pre-commit hook
    UninstallHook,
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match cli.command {
        Sub::Scan { staged } => match run_scan(staged, false) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => fatal(e),
        },
        Sub::Check { staged } => match run_scan(staged, true) {
            Ok(true) => ExitCode::from(1),
            Ok(false) => ExitCode::SUCCESS,
            Err(e) => fatal(e),
        },
        Sub::InstallHook => match install_hook() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => fatal(e),
        },
        Sub::UninstallHook => match uninstall_hook() {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => fatal(e),
        },
    }
}

fn fatal(err: anyhow::Error) -> ExitCode {
    eprintln!("error: {err:#}");
    ExitCode::from(2)
}

#[derive(Debug, Deserialize)]
struct Finding {
    #[serde(rename = "Description")]
    description: String,
    #[serde(rename = "RuleID")]
    rule_id: String,
    #[serde(rename = "File")]
    file: String,
    #[serde(rename = "StartLine")]
    start_line: i64,
    #[serde(rename = "Match", default)]
    #[allow(dead_code)]
    match_text: String,
    #[serde(rename = "Commit", default)]
    commit: String,
}

/// Returns true when at least one finding was reported.
fn run_scan(staged: bool, strict: bool) -> Result<bool> {
    let version = require_gitleaks()?;

    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mode = if staged {
        "staged changes"
    } else {
        "working tree"
    };
    println!("scanning {} in {}", mode, cwd.display());
    println!("  using gitleaks {version}");

    let report_path = std::env::temp_dir().join(format!(
        "fledge-gitleaks-{}-{}.json",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos()
    ));

    let mut args: Vec<String> = vec![
        if staged { "protect" } else { "detect" }.to_string(),
        "--no-banner".to_string(),
        "--redact".to_string(),
        "--report-format".to_string(),
        "json".to_string(),
        "--report-path".to_string(),
        report_path.to_string_lossy().into_owned(),
    ];
    if staged {
        args.push("--staged".to_string());
    }

    let started = std::time::Instant::now();
    // Inherit stderr so gitleaks's own progress is visible to the user.
    let status = Command::new("gitleaks")
        .args(&args)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::inherit())
        .status()
        .context("running gitleaks")?;
    let elapsed = started.elapsed();

    let findings = if report_path.exists() {
        let raw = fs::read_to_string(&report_path).unwrap_or_default();
        let _ = fs::remove_file(&report_path);
        if raw.trim().is_empty() {
            vec![]
        } else {
            serde_json::from_str::<Vec<Finding>>(&raw).unwrap_or_default()
        }
    } else {
        vec![]
    };

    println!();
    print_findings(&findings, strict);
    println!("scan finished in {:.1}s", elapsed.as_secs_f64());

    // gitleaks exits 1 when it found something — that's expected, not an error.
    // Only bail if it failed for some other reason (config error, etc.).
    if !status.success() && findings.is_empty() {
        bail!(
            "gitleaks exited with code {} but no findings were reported — check gitleaks output above.",
            status.code().unwrap_or(-1)
        );
    }

    Ok(!findings.is_empty())
}

fn print_findings(findings: &[Finding], strict: bool) {
    if findings.is_empty() {
        println!("✓ no secrets detected");
        return;
    }

    let label = if strict { "blocked" } else { "found" };
    println!("✗ {} {} possible secret(s):", findings.len(), label);
    for f in findings {
        let location = if f.commit.is_empty() {
            format!("{}:{}", f.file, f.start_line)
        } else {
            format!(
                "{}:{} ({})",
                f.file,
                f.start_line,
                &f.commit[..7.min(f.commit.len())]
            )
        };
        println!("  - [{}] {}", f.rule_id, location);
        println!("      {}", f.description);
    }
    if strict {
        println!();
        println!("To allowlist a false positive, add it to .gitleaks.toml.");
    }
}

fn require_gitleaks() -> Result<String> {
    let output = Command::new("gitleaks").arg("version").output().context(
        "gitleaks not found in PATH. Install it: `brew install gitleaks` \
         (macOS) or grab a binary from https://github.com/gitleaks/gitleaks/releases.",
    )?;
    if !output.status.success() {
        bail!("gitleaks is installed but failed to run (`gitleaks version` exited non-zero).");
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

const HOOK_MARKER: &str = "# fledge-plugin-gitleaks managed hook";
const HOOK_BODY: &str = r#"#!/bin/sh
# fledge-plugin-gitleaks managed hook
exec fledge gitleaks check --staged
"#;

fn hook_path() -> Result<PathBuf> {
    let root = git_root()?;
    Ok(root.join(".git").join("hooks").join("pre-commit"))
}

fn git_root() -> Result<PathBuf> {
    let cwd = std::env::current_dir().context("getting cwd")?;
    let mut current: &Path = &cwd;
    loop {
        if current.join(".git").exists() {
            return Ok(current.to_path_buf());
        }
        match current.parent() {
            Some(parent) => current = parent,
            None => bail!("Not inside a git repository."),
        }
    }
}

fn install_hook() -> Result<()> {
    let path = hook_path()?;
    if path.exists() {
        let existing = fs::read_to_string(&path).context("reading existing hook")?;
        if existing.contains(HOOK_MARKER) {
            println!("Pre-commit hook is already installed.");
            return Ok(());
        }
        bail!(
            "A pre-commit hook already exists at {} and was not installed by fledge.\n  \
             Inspect it and either remove it or add `exec fledge gitleaks check --staged` manually.",
            path.display()
        );
    }
    fs::create_dir_all(path.parent().ok_or_else(|| anyhow!("invalid hook path"))?)?;
    fs::write(&path, HOOK_BODY).context("writing pre-commit hook")?;
    make_executable(&path)?;
    println!("Installed pre-commit hook at {}", path.display());
    Ok(())
}

fn uninstall_hook() -> Result<()> {
    let path = hook_path()?;
    if !path.exists() {
        println!("No pre-commit hook to remove.");
        return Ok(());
    }
    let existing = fs::read_to_string(&path).context("reading hook")?;
    if !existing.contains(HOOK_MARKER) {
        bail!(
            "Refusing to remove pre-commit hook at {} — it was not installed by fledge.",
            path.display()
        );
    }
    fs::remove_file(&path).context("removing hook")?;
    println!("Removed pre-commit hook.");
    Ok(())
}

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut perms = fs::metadata(path)
        .context("reading hook permissions")?
        .permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).context("setting hook permissions")?;
    Ok(())
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_findings_prints_clean_message() {
        // Smoke: no findings, no panic.
        print_findings(&[], false);
        print_findings(&[], true);
    }

    #[test]
    fn finding_renders_short_commit() {
        let f = Finding {
            description: "AWS Access Key".to_string(),
            rule_id: "aws-access-key".to_string(),
            file: "config.yml".to_string(),
            start_line: 12,
            match_text: "AKIA...".to_string(),
            commit: "abcdef1234567890".to_string(),
        };
        // Smoke test that the format string with [..7] doesn't panic on long commits.
        print_findings(&[f], false);
    }

    #[test]
    fn finding_handles_short_commit_hash() {
        let f = Finding {
            description: "Generic API Key".to_string(),
            rule_id: "generic".to_string(),
            file: "src/lib.rs".to_string(),
            start_line: 1,
            match_text: "key=...".to_string(),
            commit: "abc".to_string(),
        };
        // Should not panic when commit is shorter than 7 chars.
        print_findings(&[f], true);
    }

    #[test]
    fn finding_handles_empty_commit() {
        let f = Finding {
            description: "Generic API Key".to_string(),
            rule_id: "generic".to_string(),
            file: "src/lib.rs".to_string(),
            start_line: 1,
            match_text: "key=...".to_string(),
            commit: String::new(),
        };
        print_findings(&[f], false);
    }
}
