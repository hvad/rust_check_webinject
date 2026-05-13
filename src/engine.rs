use crate::config::WebScenario;
use crate::nagios::{NagiosStatus, print_nagios_msg};
use reqwest::blocking::Client;
use std::time::{Duration, Instant};
use regex::Regex;

pub fn run_scenario(scenario: WebScenario) -> NagiosStatus {
    let timeout = Duration::from_millis(scenario.global_timeout_ms.unwrap_or(30000));
    
    // Initialisation du client avec support des cookies (session)
    let client = Client::builder()
        .cookie_store(true)
        .timeout(timeout)
        .build()
        .unwrap_or_else(|_| {
            print_nagios_msg(NagiosStatus::Unknown, "Could not init HTTP client", None);
            std::process::exit(3);
        });

    let start_time = Instant::now();
    let mut steps_passed = 0;

    for step in scenario.steps {
        let method = step.method.as_deref().unwrap_or("GET").to_uppercase();
        
        let req_builder = match method.as_str() {
            "POST" => client.post(&step.url).body(step.post_data.clone().unwrap_or_default()),
            "PUT" => client.put(&step.url).body(step.post_data.clone().unwrap_or_default()),
            _ => client.get(&step.url),
        };

        match req_builder.send() {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let body = resp.text().unwrap_or_default();

                // 1. Check HTTP Status
                if let Some(expected) = step.expected_status {
                    if status != expected {
                        let msg = format!("Step {} failed: Status {} (expected {})", step.id, status, expected);
                        print_nagios_msg(NagiosStatus::Critical, &msg, None);
                        return NagiosStatus::Critical;
                    }
                }

                // 2. Check Regex Positive
                if let Some(pattern) = step.verify_positive {
                    let re = Regex::new(&pattern).unwrap_or_else(|_| Regex::new(".*").unwrap());
                    if !re.is_match(&body) {
                        let msg = format!("Step {} failed: Pattern '{}' not found", step.id, pattern);
                        print_nagios_msg(NagiosStatus::Critical, &msg, None);
                        return NagiosStatus::Critical;
                    }
                }

                // 3. Check Regex Negative
                if let Some(pattern) = step.verify_negative {
                    let re = Regex::new(&pattern).unwrap_or_else(|_| Regex::new(".^").unwrap());
                    if re.is_match(&body) {
                        let msg = format!("Step {} failed: Forbidden pattern '{}' found", step.id, pattern);
                        print_nagios_msg(NagiosStatus::Critical, &msg, None);
                        return NagiosStatus::Critical;
                    }
                }

                steps_passed += 1;
            }
            Err(e) => {
                let msg = format!("Step {} connection error: {}", step.id, e);
                print_nagios_msg(NagiosStatus::Critical, &msg, None);
                return NagiosStatus::Critical;
            }
        }
    }

    let total_duration = start_time.elapsed().as_secs_f64();
    let perf = format!("time={:.4}s;steps={};", total_duration, steps_passed);
    print_nagios_msg(NagiosStatus::Ok, &format!("{} steps passed", steps_passed), Some(perf));
    NagiosStatus::Ok
}
