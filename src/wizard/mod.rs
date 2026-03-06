//! Wizard instruction generation module
//!
//! This module provides utilities for extracting Composio Skills content
//! and generating wizard instructions for AI agents. It integrates with the
//! official Composio Skills repository to provide production-ready guidance
//! based on best practices and anti-patterns.
//!
//! # Overview
//!
//! The wizard module consists of three main components:
//!
//! - **[`SkillsExtractor`]**: Extracts rules and best practices from the Composio Skills repository
//! - **[`WizardInstructionGenerator`]**: Generates formatted wizard instructions for AI agents
//! - **[`InstructionValidator`]**: Validates generated instructions against official patterns
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │              Composio Skills Repository                      │
//! │  (https://github.com/ComposioHQ/skills)                     │
//! │  - AGENTS.md (consolidated reference)                        │
//! │  - rules/tr-*.md (Tool Router rules)                        │
//! │  - rules/triggers-*.md (Trigger rules)                      │
//! └───────────────────────┬─────────────────────────────────────┘
//!                         │
//!                         │ extracts
//!                         ▼
//!                 ┌───────────────┐
//!                 │ SkillsExtractor│
//!                 └───────┬───────┘
//!                         │
//!          ┌──────────────┼──────────────┐
//!          │              │              │
//!          ▼              ▼              ▼
//!   ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
//!   │  Generator  │ │  Validator  │ │   Rules     │
//!   └─────────────┘ └─────────────┘ └─────────────┘
//! ```
//!
//! # Usage Examples
//!
//! ## Basic Usage: Generate Wizard Instructions
//!
//! ```rust,no_run
//! use composio_sdk::wizard::{SkillsExtractor, WizardInstructionGenerator};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Initialize the skills extractor with path to Skills repository
//! let skills = SkillsExtractor::new("vendor/skills");
//!
//! // Verify the Skills repository is accessible
//! skills.verify_path()?;
//!
//! // Create the instruction generator
//! let generator = WizardInstructionGenerator::new(skills);
//!
//! // Generate generic Composio instructions
//! let instructions = generator.generate_composio_instructions(None)?;
//! println!("{}", instructions);
//!
//! // Generate toolkit-specific instructions (e.g., for GitHub)
//! let github_instructions = generator.generate_composio_instructions(Some("github"))?;
//! println!("{}", github_instructions);
//! # Ok(())
//! # }
//! ```
//!
//! ## Advanced Usage: Extract and Filter Rules
//!
//! ```rust,no_run
//! use composio_sdk::wizard::{SkillsExtractor, Impact};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let skills = SkillsExtractor::new("vendor/skills");
//!
//! // Get all Tool Router rules
//! let tool_router_rules = skills.get_tool_router_rules()?;
//! println!("Found {} Tool Router rules", tool_router_rules.len());
//!
//! // Get all Trigger rules
//! let trigger_rules = skills.get_trigger_rules()?;
//! println!("Found {} Trigger rules", trigger_rules.len());
//!
//! // Filter rules by tag
//! let session_rules = skills.get_rules_by_tag("session")?;
//! println!("Found {} session-related rules", session_rules.len());
//!
//! // Get consolidated content from AGENTS.md
//! let consolidated = skills.get_consolidated_content()?;
//! println!("Consolidated content: {} bytes", consolidated.len());
//!
//! // Inspect individual rules
//! for rule in tool_router_rules.iter().take(5) {
//!     println!("Rule: {}", rule.title);
//!     println!("Impact: {:?}", rule.impact);
//!     println!("Tags: {:?}", rule.tags);
//!     println!("Correct examples: {}", rule.correct_examples.len());
//!     println!("Incorrect examples: {}", rule.incorrect_examples.len());
//!     println!("---");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! ## Validation: Check Instructions Against Official Patterns
//!
//! ```rust,no_run
//! use composio_sdk::wizard::{SkillsExtractor, InstructionValidator};
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let skills = SkillsExtractor::new("vendor/skills");
//! let validator = InstructionValidator::new(skills);
//!
//! // Validate some instructions
//! let instructions = r#"
//! # Composio Integration Guide
//!
//! Always use composio.create(user_id) to create a session.
//! Use session.tools() for native tool integration.
//! "#;
//!
//! let result = validator.validate(instructions)?;
//!
//! if result.is_valid() {
//!     println!("✓ Instructions are valid!");
//! } else {
//!     println!("✗ Validation failed:");
//!     println!("{}", result.format());
//! }
//!
//! if result.has_warnings() {
//!     println!("⚠ Warnings found:");
//!     println!("{}", result.format());
//! }
//!
//! println!("Total issues: {}", result.total_issues());
//! # Ok(())
//! # }
//! ```
//!
//! ## Working with Rules
//!
//! ```rust,no_run
//! use composio_sdk::wizard::{Rule, Impact};
//! use std::path::Path;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! // Load a rule from a file
//! let rule = Rule::from_file(Path::new("vendor/skills/composio/rules/tr-001.md"))?;
//!
//! println!("Title: {}", rule.title);
//! println!("Impact: {:?}", rule.impact);
//! println!("Description: {}", rule.description);
//! println!("Tags: {:?}", rule.tags);
//!
//! // Access examples
//! for (i, example) in rule.correct_examples.iter().enumerate() {
//!     println!("✅ Correct example {}: {}", i + 1, example);
//! }
//!
//! for (i, example) in rule.incorrect_examples.iter().enumerate() {
//!     println!("❌ Incorrect example {}: {}", i + 1, example);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Integration with Build Process
//!
//! The Skills repository is automatically downloaded during the build process
//! via `build.rs`. The build script:
//!
//! 1. Checks if `vendor/skills` directory exists
//! 2. If not, clones the Skills repository from GitHub
//! 3. Sets up rerun triggers for Skills content changes
//!
//! This ensures that the Skills content is always available at build time
//! for wizard instruction generation.
//!
//! # Skills Repository Structure
//!
//! ```text
//! vendor/skills/
//! ├── composio/
//! │   ├── AGENTS.md              # Consolidated reference (150+ KB)
//! │   └── rules/
//! │       ├── tr-*.md            # Tool Router rules
//! │       └── triggers-*.md      # Trigger rules
//! ```
//!
//! # Rule Format
//!
//! Rules are markdown files with YAML frontmatter:
//!
//! ```markdown
//! ---
//! title: Always use composio.create(user_id)
//! impact: critical
//! tags: [session, user-scoping]
//! ---
//!
//! # Description
//! Always create sessions with user_id for proper isolation.
//!
//! ## Correct ✅
//! ```python
//! session = composio.create(user_id="user_123")
//! ```
//!
//! ## Incorrect ❌
//! ```python
//! session = composio.create()  # Missing user_id
//! ```
//!
//! # Impact Levels
//!
//! Rules are categorized by impact:
//!
//! - **Critical**: Must be followed, causes failures if violated
//! - **High**: Should be followed, causes issues if violated
//! - **Medium**: Recommended, improves quality
//! - **Low**: Optional, nice to have
//!
//! # Error Handling
//!
//! All operations return `Result<T, SkillsError>` for proper error handling:
//!
//! ```rust,no_run
//! use composio_sdk::wizard::{SkillsExtractor, SkillsError};
//!
//! # fn main() {
//! let skills = SkillsExtractor::new("vendor/skills");
//!
//! match skills.verify_path() {
//!     Ok(_) => println!("Skills repository found"),
//!     Err(SkillsError::PathNotFound(path)) => {
//!         eprintln!("Skills repository not found at: {}", path.display());
//!     }
//!     Err(e) => eprintln!("Error: {}", e),
//! }
//! # }
//! ```

mod generator;
mod skills;
mod validator;

pub use generator::WizardInstructionGenerator;
pub use skills::{Impact, Rule, SkillsExtractor, SkillsError};
pub use validator::{InstructionValidator, ValidationResult};

/// Generate wizard instructions for Composio integration
///
/// This is a convenience function that creates a SkillsExtractor and
/// WizardInstructionGenerator, then generates comprehensive wizard instructions
/// for AI agents using Composio Skills content.
///
/// # Arguments
///
/// * `toolkit` - Optional toolkit name for context-aware instructions (e.g., "github", "gmail", "slack")
///
/// # Returns
///
/// A formatted markdown string with wizard instructions, or an error if the Skills
/// repository is not accessible or parsing fails.
///
/// # Examples
///
/// ## Generate Generic Instructions
///
/// ```no_run
/// use composio_sdk::wizard::generate_wizard_instructions;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Generate generic Composio instructions
/// let instructions = generate_wizard_instructions(None)?;
/// println!("{}", instructions);
/// # Ok(())
/// # }
/// ```
///
/// ## Generate Toolkit-Specific Instructions
///
/// ```no_run
/// use composio_sdk::wizard::generate_wizard_instructions;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// // Generate GitHub-specific instructions
/// let github_instructions = generate_wizard_instructions(Some("github"))?;
/// println!("{}", github_instructions);
///
/// // Generate Gmail-specific instructions
/// let gmail_instructions = generate_wizard_instructions(Some("gmail"))?;
/// println!("{}", gmail_instructions);
///
/// // Generate Slack-specific instructions
/// let slack_instructions = generate_wizard_instructions(Some("slack"))?;
/// println!("{}", slack_instructions);
/// # Ok(())
/// # }
/// ```
///
/// # Skills Repository
///
/// This function expects the Composio Skills repository to be available at
/// `vendor/skills/skills/composio`. The repository is automatically downloaded
/// during the build process via `build.rs`.
///
/// If the Skills repository is not found, the function will return a
/// `SkillsError::PathNotFound` error.
///
/// # Generated Content
///
/// The generated instructions include:
///
/// - **Overview**: Introduction from AGENTS.md consolidated reference
/// - **Critical Rules**: Must-follow rules with CRITICAL impact
/// - **Session Management**: Best practices for session creation and management
/// - **Authentication**: Patterns for in-chat and manual authentication
/// - **Toolkit-Specific Guidance**: Context-aware rules for the specified toolkit (if provided)
///
/// Each rule includes:
/// - Description and impact level
/// - Correct examples (✅)
/// - Incorrect examples (❌)
/// - Relevant tags
///
/// # Supported Toolkits
///
/// Common toolkits include:
/// - `github` - GitHub integration
/// - `gmail` - Gmail integration
/// - `slack` - Slack integration
/// - `jira` - Jira integration
/// - `notion` - Notion integration
/// - And 900+ more toolkits
///
/// For unknown toolkits, generic instructions are provided with a note that
/// no toolkit-specific rules were found.
///
/// # Error Handling
///
/// ```no_run
/// use composio_sdk::wizard::{generate_wizard_instructions, SkillsError};
///
/// # fn main() {
/// match generate_wizard_instructions(Some("github")) {
///     Ok(instructions) => {
///         println!("Generated {} bytes of instructions", instructions.len());
///         println!("{}", instructions);
///     }
///     Err(SkillsError::PathNotFound(path)) => {
///         eprintln!("Skills repository not found at: {}", path.display());
///         eprintln!("Run the build script to download it automatically.");
///     }
///     Err(e) => {
///         eprintln!("Error generating instructions: {}", e);
///     }
/// }
/// # }
/// ```
pub fn generate_wizard_instructions(toolkit: Option<&str>) -> Result<String, SkillsError> {
    // Default Skills repository path (downloaded by build.rs)
    let skills_path = "vendor/skills/skills/composio";
    
    // Create SkillsExtractor
    let skills = SkillsExtractor::new(skills_path);
    
    // Verify the Skills repository is accessible
    skills.verify_path()?;
    
    // Create WizardInstructionGenerator
    let generator = WizardInstructionGenerator::new(skills);
    
    // Generate instructions
    generator.generate_composio_instructions(toolkit)
}
