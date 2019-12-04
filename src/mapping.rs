use crate::jira;
use crate::worklog;
use chrono::SecondsFormat::Millis;
use chrono::Utc;

impl From<worklog::Log> for jira::WorkLog {
    fn from(input: worklog::Log) -> jira::WorkLog {
        let duration: std::time::Duration = input.time_spent.into();
        let seconds = duration.as_secs();
        let start_time = match input.when {
            Some(time) => chrono::DateTime::<chrono::Utc>::from_utc(time, chrono::Utc),
            None => (Utc::now() - chrono::Duration::seconds(seconds as i64)),
        };
        jira::WorkLog {
            time_spent_seconds: seconds,
            comment: input.comment.unwrap_or("".to_owned()),
            issue_key: input.issue.to_string(),
            started: start_time
                .to_rfc3339_opts(Millis, true)
                .replace("Z", "+0100"), // JIRA API does not support rfc3339 format with : between hours and minus on the timezone
        }
    }
}
