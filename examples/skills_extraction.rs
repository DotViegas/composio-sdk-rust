//! Example demonstrating Skills extraction for wizard instruction generation
//!
//! This example shows how to use the SkillsExtractor to extract Composio Skills
//! content from the official Skills repository.
//!
//! Prerequisites:
//! - Clone the Skills repository: `git clone https://github.com/ComposioHQ/skills.git vendor/skills`
//!
//! Run with:
//! ```bash
//! cargo run --example skills_extraction
//! ```

use composio_sdk::wizard::{Impact, SkillsExtractor};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Composio Skills Extraction Example ===\n");

    // Path to the Skills repository
    let skills_path = "vendor/skills/skills/composio";

    // Create extractor
    let extractor = SkillsExtractor::new(skills_path);

    // Verify path exists
    match extractor.verify_path() {
        Ok(_) => println!("✓ Skills repository found at: {}\n", skills_path),
        Err(e) => {
            eprintln!("✗ Skills repository not found: {}", e);
            eprintln!("\nPlease clone the Skills repository:");
            eprintln!("  git clone https://github.com/ComposioHQ/skills.git vendor/skills");
            return Ok(());
        }
    }

    // Extract Tool Router rules
    println!("--- Tool Router Rules ---");
    match extractor.get_tool_router_rules() {
        Ok(rules) => {
            println!("Found {} Tool Router rules:\n", rules.len());
            for rule in rules.iter().take(3) {
                println!("  • {} ({})", rule.title, rule.impact.as_str());
                println!("    Tags: {:?}", rule.tags);
                println!("    Correct examples: {}", rule.correct_examples.len());
                println!("    Incorrect examples: {}", rule.incorrect_examples.len());
                println!();
            }
            if rules.len() > 3 {
                println!("  ... and {} more rules\n", rules.len() - 3);
            }
        }
        Err(e) => eprintln!("Error extracting Tool Router rules: {}", e),
    }

    // Extract Trigger rules
    println!("--- Trigger Rules ---");
    match extractor.get_trigger_rules() {
        Ok(rules) => {
            println!("Found {} Trigger rules:\n", rules.len());
            for rule in rules.iter().take(3) {
                println!("  • {} ({})", rule.title, rule.impact.as_str());
                println!("    Tags: {:?}", rule.tags);
                println!();
            }
            if rules.len() > 3 {
                println!("  ... and {} more rules\n", rules.len() - 3);
            }
        }
        Err(e) => eprintln!("Error extracting Trigger rules: {}", e),
    }

    // Extract rules by tag
    println!("--- Rules Tagged 'sessions' ---");
    match extractor.get_rules_by_tag("sessions") {
        Ok(rules) => {
            println!("Found {} rules with 'sessions' tag:\n", rules.len());
            for rule in &rules {
                println!("  • {} ({})", rule.title, rule.impact.as_str());
            }
            println!();
        }
        Err(e) => eprintln!("Error extracting rules by tag: {}", e),
    }

    // Get consolidated content
    println!("--- Consolidated AGENTS.md ---");
    match extractor.get_consolidated_content() {
        Ok(content) => {
            println!("AGENTS.md size: {} bytes ({:.1} KB)", content.len(), content.len() as f64 / 1024.0);
            println!("First 200 characters:");
            println!("{}", &content.chars().take(200).collect::<String>());
            println!("...\n");
        }
        Err(e) => eprintln!("Error reading AGENTS.md: {}", e),
    }

    // Show critical rules
    println!("--- Critical Rules ---");
    match extractor.get_all_rules() {
        Ok(rules) => {
            let critical_rules: Vec<_> = rules
                .iter()
                .filter(|r| matches!(r.impact, Impact::Critical))
                .collect();

            println!("Found {} critical rules:\n", critical_rules.len());
            for rule in critical_rules {
                println!("  • {}", rule.title);
                println!("    Description: {}", rule.description);
                println!("    Tags: {:?}", rule.tags);
                println!();
            }
        }
        Err(e) => eprintln!("Error extracting all rules: {}", e),
    }

    println!("=== Example Complete ===");

    Ok(())
}
