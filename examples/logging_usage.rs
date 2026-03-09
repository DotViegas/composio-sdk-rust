/// Example demonstrating logging utilities
///
/// Run with: cargo run --example logging_usage --features local-debug
///
/// This example shows:
/// - Environment-based log configuration
/// - Verbosity levels and message truncation
/// - WithLogger trait usage
/// - Custom logger names

use composio_sdk::{
    setup_logging, setup_logging_from_env, set_verbosity, truncate_message,
    LogLevel, Verbosity, WithLogger,
};

struct MyService {
    name: String,
}

impl WithLogger for MyService {
    fn logger_name(&self) -> &str {
        &self.name
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Composio SDK Logging Examples\n");

    // Example 1: Basic setup with log level
    println!("1️⃣ Basic Setup");
    setup_logging(LogLevel::Debug);
    println!("   ✅ Logging configured with Debug level\n");

    // Example 2: Setup from environment variables
    println!("2️⃣ Environment-based Setup");
    println!("   Set COMPOSIO_LOGGING_LEVEL=debug");
    println!("   Set COMPOSIO_LOG_VERBOSITY=2");
    setup_logging_from_env();
    println!("   ✅ Logging configured from environment\n");

    // Example 3: Verbosity levels
    println!("3️⃣ Verbosity Levels");
    for verbosity in [
        Verbosity::Minimal,
        Verbosity::Normal,
        Verbosity::Verbose,
        Verbosity::Full,
    ] {
        set_verbosity(verbosity);
        let max_size = verbosity
            .max_line_size()
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unlimited".to_string());
        println!("   {:?}: max {} chars", verbosity, max_size);
    }
    println!();

    // Example 4: Message truncation
    println!("4️⃣ Message Truncation");
    set_verbosity(Verbosity::Minimal);

    let short_msg = "Short message";
    let long_msg = "a".repeat(300);

    println!("   Short message: {}", truncate_message(short_msg));
    println!("   Long message (300 chars): {}", truncate_message(&long_msg));
    println!("   Truncated to: {} chars\n", truncate_message(&long_msg).len());

    // Example 5: WithLogger trait
    println!("5️⃣ WithLogger Trait");
    let service = MyService {
        name: "my_service".to_string(),
    };

    println!("   Logger name: {}", service.logger_name());
    service.log_info("This is an info message");
    service.log_debug("This is a debug message");
    service.log_warning("This is a warning message");
    service.log_error("This is an error message");
    println!("   ✅ Messages logged (check output with --features local-debug)\n");

    // Example 6: Different verbosity levels
    println!("6️⃣ Verbosity Impact on Truncation");
    let test_msg = "x".repeat(600);

    for verbosity in [
        Verbosity::Minimal,
        Verbosity::Normal,
        Verbosity::Verbose,
        Verbosity::Full,
    ] {
        set_verbosity(verbosity);
        let truncated = truncate_message(&test_msg);
        println!(
            "   {:?}: {} chars -> {} chars",
            verbosity,
            test_msg.len(),
            truncated.len()
        );
    }
    println!();

    // Example 7: Log levels from string
    println!("7️⃣ Log Level Parsing");
    for level_str in ["debug", "info", "warning", "error"] {
        if let Some(level) = LogLevel::from_str(level_str) {
            println!("   '{}' -> {:?}", level_str, level);
        }
    }
    println!();

    println!("✨ All examples completed!");
    println!("\n💡 Tips:");
    println!("   - Use COMPOSIO_LOGGING_LEVEL to set log level");
    println!("   - Use COMPOSIO_LOG_VERBOSITY (0-3) to control truncation");
    println!("   - Enable 'local-debug' feature for actual logging output");
    println!("   - Implement WithLogger trait for custom types");

    Ok(())
}
