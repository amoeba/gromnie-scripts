use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let task = args.get(1).map(|s| s.as_str()).unwrap_or("build");

    match task {
        "build" => build()?,
        "install" => install()?,
        "build-install" | "bi" => {
            build()?;
            install()?;
        }
        "clean" => clean()?,
        _ => {
            eprintln!("Unknown task: {}", task);
            eprintln!("Available tasks: build, install, build-install (bi), clean");
            std::process::exit(1);
        }
    }

    Ok(())
}

fn build() -> Result<()> {
    println!("Building all scripts...");

    let root = get_workspace_root()?;
    let scripts = find_script_dirs(&root)?;

    for script_dir in scripts {
        println!("Building {}...", script_dir.display());

        let package_name = get_package_name(&script_dir)?;
        let status = Command::new("cargo")
            .args(&[
                "build",
                "--release",
                "--target",
                "wasm32-wasip2",
                "-p",
                &package_name,
            ])
            .current_dir(&root)
            .status()
            .context("Failed to run cargo build")?;

        if !status.success() {
            anyhow::bail!("Build failed for {}", script_dir.display());
        }
    }

    Ok(())
}

fn install() -> Result<()> {
    println!("Installing scripts...");

    let root = get_workspace_root()?;
    let scripts = find_script_dirs(&root)?;
    let install_dir = get_install_dir()?;

    // Create target directory
    fs::create_dir_all(&install_dir)
        .context(format!("Failed to create {}", install_dir.display()))?;

    // Build set of expected wasm filenames (package names with underscores)
    let mut expected_wasm_files = std::collections::HashSet::new();
    for script_dir in &scripts {
        let package_name = get_package_name(script_dir)?;
        let wasm_name = format!("{}.wasm", package_name.replace("-", "_"));
        expected_wasm_files.insert(wasm_name);
    }

    // Install only the expected wasm files
    let workspace_wasm_dir = root.join("target/wasm32-wasip2/release");

    if workspace_wasm_dir.exists() {
        for entry in fs::read_dir(&workspace_wasm_dir).context("Failed to read wasm directory")? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().map(|e| e == "wasm").unwrap_or(false) {
                let filename = path.file_name().unwrap().to_string_lossy().to_string();

                // Only copy if it's an expected script wasm file
                if expected_wasm_files.contains(&filename) {
                    let target_path = install_dir.join(&filename);

                    fs::copy(&path, &target_path).context(format!(
                        "Failed to copy {} to install directory",
                        path.display()
                    ))?;

                    println!("  Installed {}", filename);
                }
            }
        }
    }

    println!("Installation complete to {}", install_dir.display());
    Ok(())
}

fn clean() -> Result<()> {
    println!("Cleaning build artifacts...");

    let root = get_workspace_root()?;

    let status = Command::new("cargo")
        .args(&["clean"])
        .current_dir(&root)
        .status()
        .context("Failed to run cargo clean")?;

    if !status.success() {
        anyhow::bail!("Clean failed");
    }

    Ok(())
}

fn get_package_name(script_dir: &Path) -> Result<String> {
    let cargo_toml =
        fs::read_to_string(script_dir.join("Cargo.toml")).context("Failed to read Cargo.toml")?;

    for line in cargo_toml.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("name = \"") {
            if let Some(name) = rest.strip_suffix('"') {
                return Ok(name.to_string());
            }
        }
    }

    anyhow::bail!(
        "Could not find package name in {}/Cargo.toml",
        script_dir.display()
    )
}

fn get_workspace_root() -> Result<PathBuf> {
    let mut root = env::current_dir()?;

    // Look for workspace Cargo.toml
    while !root.join("Cargo.toml").exists() {
        if !root.pop() {
            anyhow::bail!("Could not find workspace root (no Cargo.toml found)");
        }
    }

    Ok(root)
}

fn find_script_dirs(root: &Path) -> Result<Vec<PathBuf>> {
    let mut scripts = Vec::new();

    for entry in fs::read_dir(root).context("Failed to read workspace root")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && path.join("Cargo.toml").exists() {
            // Skip xtask
            if path.file_name().map(|n| n != "xtask").unwrap_or(false) {
                scripts.push(path);
            }
        }
    }

    scripts.sort();
    Ok(scripts)
}

fn get_install_dir() -> Result<PathBuf> {
    let mut path = dirs::home_dir().context("Could not determine home directory")?;
    path.push(".local/share/gromnie/scripts");
    Ok(path)
}
