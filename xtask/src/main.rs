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
        
        let status = Command::new("cargo")
            .args(&[
                "build",
                "--release",
                "--target", "wasm32-wasip2",
                "-p", script_dir.file_name().unwrap().to_str().unwrap(),
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
    let install_dir = get_install_dir()?;
    
    // Create target directory
    fs::create_dir_all(&install_dir)
        .context(format!("Failed to create {}", install_dir.display()))?;
    
    // Check both workspace-level and script-level target directories
    let workspace_wasm_dir = root.join("target/wasm32-wasip2/release");
    
    if workspace_wasm_dir.exists() {
        for entry in fs::read_dir(&workspace_wasm_dir).context("Failed to read wasm directory")? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().map(|e| e == "wasm").unwrap_or(false) && path.is_file() {
                let filename = path.file_name().unwrap();
                let target_path = install_dir.join(filename);
                
                fs::copy(&path, &target_path)
                    .context(format!("Failed to copy {} to install directory", path.display()))?;
                
                println!("  Installed {}", filename.to_string_lossy());
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
    if cfg!(target_os = "macos") {
        let mut path = dirs::home_dir().context("Could not determine home directory")?;
        path.push("Library/Application Support/gromnie/scripts");
        Ok(path)
    } else {
        let mut path = dirs::home_dir().context("Could not determine home directory")?;
        path.push(".local/share/gromnie/scripts");
        Ok(path)
    }
}
