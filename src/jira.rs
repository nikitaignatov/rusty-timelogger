use crate::config;
use base64_lib;
use colored::*;
use minihttp::request::Request;
use std::collections::HashMap;

/// Worklog structure for Jira REST API
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkLog {
    pub time_spent_seconds: u64,
    pub comment: String,
    pub started: String,
    pub issue_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    name: String,
    key: String,
    account_id: String,
    display_name: String,
    email_address: Option<String>,
    active: bool,
}

fn worklog_path(issue_key: &str, conf: &config::RustyConfig) -> String {
    format!(
        "{host}/rest/api/latest/issue/{issueKey}/worklog",
        host = conf.jira_host.trim_matches('/'),
        issueKey = issue_key
    )
}

fn auth_header(conf: config::RustyConfig) -> String {
    let input = format!("{}:{}", conf.jira_username, conf.jira_api_key);
    let input_vector: Vec<u8> = String::from(input).into_bytes();
    let result_string: String = base64_lib::encode(&input_vector);
    format!("Basic {}", result_string)
}

fn error_message(input: usize, url: String, json: String) {
    let code = input.to_string();
    match input {
        201 => println!("{} {}", "SUCCESS".green().bold(), code.green()),
        _ => {
            println!("{} {}", "ERROR".yellow().bold(), code.to_string().yellow());
            println!("Check the JSON that has been sent to BitBucket.");
            println!("{}  :{}", "url".bold(), url);
            println!("{} :{}", "json".bold(), json.green());
        }
    }
}

pub fn add_worklog(work: WorkLog) {
    let conf = config::load().expect("Configuration is not present.");
    let path = worklog_path(&work.issue_key, &conf);
    let json = serde_json::to_string_pretty(&work).expect("Work log could not be serialized.");
    post(path, json);
    ()
}

pub fn post(path: String, json: String) {
    let conf = config::load().expect("Configuration is not present.");

    let mut http = Request::new(&path).unwrap();
    let mut headers = HashMap::new();
    let result = auth_header(conf);
    headers.insert("Content-Type", "application/json");
    headers.insert("Authorization", &result);
    let res = http.post().headers(headers).body_str(&json).send().unwrap();
    error_message(res.status_code(), path, json);
    ()
}

#[test]
fn test_issues() {
    add_worklog(WorkLog {
        time_spent_seconds: 3600,
        issue_key: "PROJ-1234".to_owned(),
        comment: "# trerer\n# 3333\n# {color:#ff5630}jo jo {color}\n\n".to_owned(),
        started: chrono::Utc::now()
            .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
            .replace("Z", "+0100"), // JIRA API does not support rfc3339 format with : between hours and minus on the timezone
    });
    //  issues_list();
}
