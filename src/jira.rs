use crate::config;
use chrono::SecondsFormat::Millis;
use chrono::Utc;
use log::Level;
use reqwest::header::CONTENT_TYPE;
use reqwest::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;

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

pub fn add_worklog(work: WorkLog) {
    let client = reqwest::Client::new();
    let conf = config::load().expect("Configuration is not present.");
    let path = worklog_path(&work.issue_key, &conf);
    let json = serde_json::to_string(&work).expect("Work log could not be serialized.");
    let res = client
        .post(&path)
        .basic_auth(conf.jira_username, Some(conf.jira_api_key))
        .header(CONTENT_TYPE, "application/json")
        .body(json)
        .send();
    match res {
        Ok(response) => println!("{:?}", response),
        Err(e) => println!("{:?}", e),
    }
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
