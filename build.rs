use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    let vendor_skills_path = Path::new("vendor/skills");
    
    // Check if vendor/skills directory exists
    if !vendor_skills_path.exists() {
        println!("cargo:warning=Skills repository not found at vendor/skills");
        println!("cargo:warning=Attempting to clone Skills repository...");
        
        // Create vendor directory if it doesn't exist
        let vendor_path = Path::new("vendor");
        if !vendor_path.exists() {
            fs::create_dir_all(vendor_path)
                .expect("Failed to create vendor directory");
        }
        
        // Clone Skills repository with depth 1 for faster download
        let clone_result = Command::new("git")
            .args([
                "clone",
                "--depth",
                "1",
                "https://github.com/ComposioHQ/skills.git",
                "vendor/skills"
            ])
            .status();
        
        match clone_result {
            Ok(status) if status.success() => {
                println!("cargo:warning=Successfully cloned Skills repository");
            }
            Ok(status) => {
                panic!(
                    "Failed to clone Skills repository. Git exited with status: {}. \
                    Please ensure git is installed and you have internet connectivity.",
                    status
                );
            }
            Err(e) => {
                panic!(
                    "Failed to execute git command: {}. \
                    Please ensure git is installed and available in PATH.",
                    e
                );
            }
        }
    } else {
        println!("cargo:warning=Skills repository found at vendor/skills");
    }
    
    // Verify Skills content is accessible
    verify_skills_content();
    
    // Add rerun-if-changed directives for Skills content
    add_rerun_directives();
}

fn verify_skills_content() {
    let skills_path = Path::new("vendor/skills/skills");
    
    // Check for key directories and files
    let agents_md = skills_path.join("composio/AGENTS.md");
    let rules_dir = skills_path.join("composio/rules");
    
    if !agents_md.exists() {
        panic!(
            "Skills content verification failed: AGENTS.md not found at {}. \
            The Skills repository may be incomplete or corrupted.",
            agents_md.display()
        );
    }
    
    if !rules_dir.exists() {
        panic!(
            "Skills content verification failed: rules directory not found at {}. \
            The Skills repository may be incomplete or corrupted.",
            rules_dir.display()
        );
    }
    
    // Check for at least some rule files
    let rule_files: Vec<_> = fs::read_dir(&rules_dir)
        .expect("Failed to read rules directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry.path().extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "md")
                .unwrap_or(false)
        })
        .collect();
    
    if rule_files.is_empty() {
        panic!(
            "Skills content verification failed: No markdown files found in rules directory. \
            The Skills repository may be incomplete."
        );
    }
    
    println!("cargo:warning=Skills content verified successfully");
    println!("cargo:warning=Found {} rule files", rule_files.len());
}

fn add_rerun_directives() {
    let skills_path = Path::new("vendor/skills/skills");
    
    // Rerun if the entire vendor/skills directory changes
    println!("cargo:rerun-if-changed=vendor/skills");
    
    // Rerun if AGENTS.md changes
    let agents_md = skills_path.join("composio/AGENTS.md");
    if agents_md.exists() {
        println!("cargo:rerun-if-changed={}", agents_md.display());
    }
    
    // Rerun if rules directory changes
    let rules_dir = skills_path.join("composio/rules");
    if rules_dir.exists() {
        println!("cargo:rerun-if-changed={}", rules_dir.display());
        
        // Add directives for individual rule files
        if let Ok(entries) = fs::read_dir(&rules_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if path.extension().and_then(|ext| ext.to_str()) == Some("md") {
                    println!("cargo:rerun-if-changed={}", path.display());
                }
            }
        }
    }
    
    // Rerun if build.rs itself changes
    println!("cargo:rerun-if-changed=build.rs");
}
