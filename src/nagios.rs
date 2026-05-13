#[allow(dead_code)]
pub enum NagiosStatus {
    Ok,
    Warning,
    Critical,
    Unknown,
}

impl NagiosStatus {
    pub fn to_exit_code(&self) -> i32 {
        match self {
            Self::Ok => 0,
            Self::Warning => 1,
            Self::Critical => 2,
            Self::Unknown => 3,
        }
    }
}

pub fn print_nagios_msg(status: NagiosStatus, message: &str, perf_data: Option<String>) {
    let status_str = match status {
        NagiosStatus::Ok => "OK",
        NagiosStatus::Warning => "WARNING",
        NagiosStatus::Critical => "CRITICAL",
        NagiosStatus::Unknown => "UNKNOWN",
    };
    
    match perf_data {
        Some(perf) => println!("WEBINJECT {} - {} | {}", status_str, message, perf),
        None => println!("WEBINJECT {} - {}", status_str, message),
    }
}
