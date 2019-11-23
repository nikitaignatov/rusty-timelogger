extern crate humantime;
use humantime::Duration;
use std::num::ParseIntError;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug)]
struct IssueKey {
    project: String,
    key: i16,
}

/// Jira IssueKey parses PROJ-1234 into a struct of IssueKey { project: "PROJ", key: 1234 }
impl FromStr for IssueKey {
    type Err = ParseIntError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = input.split("-").collect();
        let project = parts[0].to_string().to_uppercase();
        let key = parts[1]
            .parse()
            .expect(&("Failed to extract key from the issue. ".to_owned() + input));
        Ok(IssueKey {
            project: project,
            key: key,
        })
    }
}

#[derive(StructOpt, Debug)]
struct Log {
    /// How much time was spent on this worklog. Examples: is 1h or 1h30m or 30m
    time_spent: Duration,
    /// Issue Key for this worklog. Examples: PROJ-1234
    issue: IssueKey,
    /// Describe what you have done
    comment: Option<String>,
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Rusty TimeLogger is a cli for the timelogging")]
enum App {
    /// Log command allows you to create a work log for a specific issue.
    Log(Log),
}

fn main() {
    println!("{:?}", App::from_args());
}
