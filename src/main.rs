mod config;
mod jira;
mod worklog;

use chrono::SecondsFormat::Millis;
use chrono::Utc;
use colored::*;
use structopt::StructOpt;

#[macro_use]
extern crate serde_derive;
extern crate log;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Rusty TimeLogger is a cli for the timelogging")]
enum App {
    /// Log command allows you to create a work log for a specific issue.
    Log(worklog::Log),
    /// Config command allows to change some of the settings
    Config(config::RustyConfig),
}

impl worklog::Log {
    pub fn to_jira_worklog(self) -> jira::WorkLog {
        let duration: std::time::Duration = self.time_spent.into();
        let seconds = duration.as_secs();
        let start_time = match self.when {
            Some(time) => chrono::DateTime::<chrono::Utc>::from_utc(time, chrono::Utc),
            None => (Utc::now() - chrono::Duration::seconds(seconds as i64)),
        };
        jira::WorkLog {
            time_spent_seconds: seconds,
            comment: self.comment.unwrap_or("".to_owned()),
            issue_key: self.issue.to_string(),
            started: start_time
                .to_rfc3339_opts(Millis, true)
                .replace("Z", "+0100"), // JIRA API does not support rfc3339 format with : between hours and minus on the timezone
        }
    }
}

fn main() -> Result<(), ::std::io::Error> {
    control::set_virtual_terminal(true).expect("Failed to enable virtual terminal");
    let args = App::from_args();
    match args {
        App::Config(conf) => {
            println!("{:?}", conf);
            config::store(conf);
            println!("Configuration is saved.");
        }
        App::Log(input) => {
            jira::add_worklog(input.to_jira_worklog());
        }
    };
    Ok(())
}
