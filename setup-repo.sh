#!/bin/bash
# Script to set up composio-sdk as a separate git repository

set -e

echo "Setting up composio-sdk-rust repository..."

# Check if we're in the composio-sdk directory
if [ ! -f "Cargo.toml" ]; then
    echo "Error: Please run this script from the composio-sdk directory"
    exit 1
fi

# Initialize git repository
echo "Initializing git repository..."
git init

# Create .gitignore if it doesn't exist
if [ ! -f ".gitignore" ]; then
    cat > .gitignore << 'EOF'
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
EOF
    echo "Created .gitignore"
fi

# Add all files
echo "Adding files to git..."
git add .

# Create initial commit
echo "Creating initial commit..."
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
echo "Setting default branch to main..."
git branch -M main

# Add remote (you'll need to update this with your actual repository URL)
echo "Adding remote origin..."
git remote add origin https://github.com/DotViegas/composio-sdk-rust.git

# Create release tag
echo "Creating release tag v0.1.0..."
git tag -a v0.1.0 -m "Release v0.1.0

Initial release of Composio Rust SDK

Features:
- Session management and tool execution
- All 5 meta tools supported
- Comprehensive error handling
- Memory-efficient (~2 MB footprint)
- Full documentation and examples
- Python SDK compatibility"

echo ""
echo "Repository setup complete!"
echo ""
echo "Next steps:"
echo "1. Review the changes: git log"
echo "2. Push to GitHub: git push -u origin main"
echo "3. Push tags: git push --tags"
echo "4. Create GitHub release at: https://github.com/DotViegas/composio-sdk-rust/releases/new"
echo "5. Publish to crates.io: cargo publish"
