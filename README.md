# Rust Webinject

**Rust Webinject** is a modern, high-performance, and memory-safe rewrite of 
the classic [WebInject](https://github.com/sni/Webinject) tool. 
It is specifically built to monitor complex web workflows while maintaining 
full compatibility with the **Nagios Plugin Guidelines**.

## How it Works

The tool follows a linear execution path for each scenario:

1. **Parsing**: It detects the file format (YAML, JSON, or XML) and loads the configuration.
2. **Sequential Execution**: It processes each `step` one by one.
3. **State Persistence**: It uses a shared `CookieStore` so that if Step 1 is a "Login", Step 2 remains authenticated.
4. **Validation**: It performs three checks per step (Status Code, Positive Regex, Negative Regex).
5. **Reporting**: It outputs a single line of status and performance data, then exits with the appropriate Nagios code.

---

## Installation

```bash
# Clone the repository
git clone https://github.com/hvad/rust_check_webinject.git
cd rust_check_webinject

# Build the optimized release version
cargo build --release

# Install to your local bin directory
sudo cp target/release/rust_check_webinject /usr/local/bin/

```

---

##  Usage Examples

### Running a YAML Scenario

```bash
rust_check_webinject --config scenarios/example.yaml

```

### Typical Output (Success)

`WEBINJECT OK - 3 steps passed | time=0.8521s;steps=3;`

### Typical Output (Failure)

`WEBINJECT CRITICAL - Step LOGIN_STEP failed: Status 401 (expected 200)`

---

## Configuration Reference

Whether you use JSON, YAML, or XML, the fields remain the same:

| Field | Description | Required |
| --- | --- | --- |
| `global_timeout_ms` | Maximum time for the entire scenario | No (Default: 30s) |
| `id` | Unique identifier for the step | **Yes** |
| `url` | Target URL | **Yes** |
| `method` | HTTP Method (GET, POST, PUT, DELETE) | No (Default: GET) |
| `post_data` | Body content for POST/PUT requests | No |
| `expected_status` | Expected HTTP Response Code | No |
| `verify_positive` | Regex that **must** be present in the body | No |
| `verify_negative` | Regex that **must not** be present in the body | No |

---

## Nagios Integration

To use **Rust Webinject** in your monitoring server:

1. Define the command in your Nagios configuration:

```bash
define command {
    command_name    check_rust_webinject
    command_line    /usr/local/bin/rust_check_webinject --config /etc/nagios/scenarios/$ARG1$
}

```

2. Assign the check to a service:

```bash
define service {
    use                     generic-service
    host_name               my_web_server
    service_description     User_Login_Workflow
    check_command           check_rust_webinject!login_scenario.yaml
}

```
