# PowerShell script to set up composio-sdk as a separate git repository

$ErrorActionPreference = "Stop"

Write-Host "Setting up composio-sdk-rust repository..." -ForegroundColor Green

# Check if we're in the composio-sdk directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "Error: Please run this script from the composio-sdk directory" -ForegroundColor Red
    exit 1
}

# Initialize git repository
Write-Host "Initializing git repository..." -ForegroundColor Yellow
git init

# Create .gitignore if it doesn't exist
if (-not (Test-Path ".gitignore")) {
    @"
# Rust
/target/
**/*.rs.bk
*.pdb
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Testing
*.profraw
*.profdata

# Documentation
/target/doc/
"@ | Out-File -FilePath ".gitignore" -Encoding UTF8
    Write-Host "Created .gitignore" -ForegroundColor Green
}

# Add all files
Write-Host "Adding files to git..." -ForegroundColor Yellow
git add .

# Create initial commit
Write-Host "Creating initial commit..." -ForegroundColor Yellow
git commit -m "Initial release v0.1.0

- Complete Tool Router API implementation
- Session management with user scoping
- Tool execution (regular and meta tools)
- Comprehensive error handling with retry logic
- Type-safe data models
- Memory-efficient design (~2 MB footprint)
- Full documentation and examples
- Integration tests and benchmarks
- Skills integration for wizard instructions"

# Rename branch to main
Write-Host "Setting default branch to main..." -ForegroundColor Yellow
git branch -M main

# Add remote
Write-Host "Adding remote origin..." -ForegroundColor Yellow
git remote add origin https://github.com/DotViegas/composio-sdk-rust.git

# Create release tag
Write-Host "Creating release tag v0.1.0..." -ForegroundColor Yellow
git tag -a v0.1.0 -m "Release v0.1.0

Initial release of Composio Rust SDK

Features:
- Session management and tool execution
- All 5 meta tools supported
- Comprehensive error handling
- Memory-efficient (~2 MB footprint)
- Full documentation and examples
- Python SDK compatibility"

Write-Host ""
Write-Host "Repository setup complete!" -ForegroundColor Green
Write-Host ""
Write-Host "Next steps:" -ForegroundColor Cyan
Write-Host "1. Review the changes: git log"
Write-Host "2. Push to GitHub: git push -u origin main"
Write-Host "3. Push tags: git push --tags"
Write-Host "4. Create GitHub release at: https://github.com/DotViegas/composio-sdk-rust/releases/new"
Write-Host "5. Publish to crates.io: cargo publish"
