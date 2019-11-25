use crate::config;
extern crate base64_lib;

use chrono::SecondsFormat::Millis;
use chrono::Utc;
use log::Level;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

use minihttp::request::Request;

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
fn worklog_path(issueKey: &str, conf: &config::RustyConfig) -> String {
    format!(
        "{host}/rest/api/latest/issue/{issueKey}/worklog",
        host = conf.jira_host.trim_matches('/'),
        issueKey = issueKey
    )
}

fn auth_header(conf: config::RustyConfig) -> String {
    let input = format!("{}:{}", conf.jira_username, conf.jira_api_key);
    let input_vector: Vec<u8> = String::from(input).into_bytes();
    let result_string: String = base64_lib::encode(&input_vector);
    format!("Basic {}", result_string)
}

pub fn add_worklog(work: WorkLog) {
    let conf = config::load().expect("Configuration is not present.");
    let path = worklog_path(&work.issue_key, &conf);
    let json = serde_json::to_string(&work).expect("Work log could not be serialized.");

    let mut http = Request::new(&path).unwrap();
    let mut headers = HashMap::new();
    let result = auth_header(conf);
    headers.insert("Content-Type", "application/json");
    headers.insert("Authorization", &result);
    // println!("URL: {}", path);
    // println!("JSON: {}", json);
    let res = http.post().headers(headers).body_str(&json).send().unwrap();
    println!("status code {}", res.status_code());

    ()
}

#[test]
fn test_issues() {
    add_worklog(WorkLog {
        time_spent_seconds: 3600,
        issue_key: "PROJ-1234".to_owned(),
        comment: "# trerer\n# 3333\n# {color:#ff5630}jo jo {color}\n\n".to_owned(),
        started: Utc::now()
            .to_rfc3339_opts(Millis, true)
            .replace("Z", "+0100"), // JIRA API does not support rfc3339 format with : between hours and minus on the timezone
    });
    //  issues_list();
}
