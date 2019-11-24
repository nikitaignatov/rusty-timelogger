extern crate humantime;
use humantime::Duration;
use std::num::ParseIntError;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq)]
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

#[derive(StructOpt, PartialEq, Debug)]
struct Log {
    /// How much time was spent on this worklog. Examples: is 1h or 1h30m or 30m
    time_spent: Duration,
    /// Issue Key for this worklog. Examples: PROJ-1234
    issue: IssueKey,
    /// Describe what you have done
    comment: Option<String>,
}

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Rusty TimeLogger is a cli for the timelogging")]
enum App {
    /// Log command allows you to create a work log for a specific issue.
    Log(Log),
}

fn main() {
    println!("{:?}", App::from_args());
}

#[cfg(test)]
mod tests {
    extern crate test_case_derive;
    use super::*;
    use test_case_derive::test_case;

    #[test_case(vec!["log", "1h", "a-1"],                  None)]
    #[test_case(vec!["log", "1h", "a-1","test"],           Some("test".to_string()))]
    #[test_case(vec!["log", "1h", "a-1",""],               Some("".to_string()))]
    #[test_case(vec!["log", "1h", "a-1","some work log"],  Some("some work log".to_string()))]
    fn comment(input: Vec<&str>, expeced: Option<String>) {
        match Log::from_clap(&Log::clap().get_matches_from(input)) {
            Log {
                comment,
                issue,
                time_spent,
            } => assert_eq!(comment, expeced),
        }
    }

    #[test_case(vec!["log", "1h",       "a-1"],         3600)]
    #[test_case(vec!["log", "1h30m",    "a-1"],         5400)]
    #[test_case(vec!["log", "1m",       "a-1"],         60)]
    fn time_spent(input: Vec<&str>, expeced: u64) {
        match Log::from_clap(&Log::clap().get_matches_from(input)) {
            Log {
                comment,
                issue,
                time_spent,
            } => assert_eq!(
                time_spent,
                Duration::from(std::time::Duration::new(expeced, 0))
            ),
        }
    }

    #[test_case(vec!["log", "1h", "a-1"],       "A",1)]
    #[test_case(vec!["log", "1h", "a-2"],       "A",2)]
    #[test_case(vec!["log", "1m", "b-0"],       "B",0)]
    #[test_case(vec!["log", "1m", "proj-1234"], "PROJ",1234)]
    fn issue_key(input: Vec<&str>, project: &str, key: i16) {
        match Log::from_clap(&Log::clap().get_matches_from(input)) {
            Log {
                comment,
                issue,
                time_spent,
            } => assert_eq!(
                issue,
                IssueKey {
                    project: project.to_string(),
                    key: key
                }
            ),
        }
    }
}
