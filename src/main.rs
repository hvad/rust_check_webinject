mod config;
mod nagios;
mod engine;

use clap::Parser;
use std::fs;
use std::path::Path;
use std::process;
use crate::config::WebScenario;
use crate::nagios::{NagiosStatus, print_nagios_msg};

#[derive(Parser, Debug)]
#[command(author, version, about = "WebInject-RS: Modern web monitoring plugin")]
struct Args {
    /// Path to scenario file (.json, .yaml, .xml)
    #[arg(short, long)]
    config: String,
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.config);

    if !path.exists() {
        print_nagios_msg(NagiosStatus::Unknown, "Scenario file not found", None);
        process::exit(3);
    }

    let content = fs::read_to_string(path).unwrap_or_default();
    let extension = path.extension().and_then(|s| s.to_str()).unwrap_or("");

    let scenario: WebScenario = match extension {
        "json" => serde_json::from_str(&content).unwrap_or_else(|e| {
            print_nagios_msg(NagiosStatus::Unknown, &format!("JSON Error: {}", e), None);
            process::exit(3);
        }),
        "yaml" | "yml" => serde_yaml::from_str(&content).unwrap_or_else(|e| {
            print_nagios_msg(NagiosStatus::Unknown, &format!("YAML Error: {}", e), None);
            process::exit(3);
        }),
        "xml" => quick_xml::de::from_str(&content).unwrap_or_else(|e| {
            print_nagios_msg(NagiosStatus::Unknown, &format!("XML Error: {}", e), None);
            process::exit(3);
        }),
        _ => {
            print_nagios_msg(NagiosStatus::Unknown, "Unsupported extension. Use .json, .yaml, or .xml", None);
            process::exit(3);
        }
    };

    let result = engine::run_scenario(scenario);
    process::exit(result.to_exit_code());
}
