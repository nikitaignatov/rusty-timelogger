extern crate humantime;
use humantime::Duration;
use std::num::ParseIntError;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, PartialEq)]
pub struct IssueKey {
    project: String,
    key: i16,
}

#[derive(StructOpt, PartialEq, Debug)]
pub struct Log {
    /// How much time was spent on this worklog. Examples: is 1h or 1h30m or 30m
    pub time_spent: Duration,
    /// Issue Key for this worklog. Examples: PROJ-1234
    pub issue: IssueKey,
    /// When did the work started
    #[structopt(short = "w", long = "when")]
    pub when: Option<chrono::NaiveDateTime>,
    /// Describe what you have done
    pub comment: Option<String>,
}

impl IssueKey {
    pub fn to_string(self) -> String {
        format!("{}-{}", self.project, self.key)
    }
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

#[cfg(test)]
mod tests {
    extern crate test_case_derive;
    use super::*;
    use test_case_derive::test_case;
    fn convert(input: Vec<&str>) -> Result<Log, &str> {
        Ok(Log::from_clap(&Log::clap().get_matches_from(input)))
    }

    #[test_case(vec!["log", "1h", "a-1"],                  None)]
    #[test_case(vec!["log", "1h", "a-1","test"],           Some("test".to_string()))]
    #[test_case(vec!["log", "1h", "a-1",""],               Some("".to_string()))]
    #[test_case(vec!["log", "1h", "a-1","some work log"],  Some("some work log".to_string()))]
    fn comment(input: Vec<&str>, expeced: Option<String>) {
        match convert(input) {
            Ok(log) => assert_eq!(log.comment, expeced),
            _ => assert!(false, "failed to match the pattern"),
        }
    }

    #[test_case(vec!["log", "1h",    "a-1"], 3600)]
    #[test_case(vec!["log", "1h30m", "a-1"], 5400)]
    #[test_case(vec!["log", "1m",    "a-1"], 60)]
    //  #[test_case(vec!["log", "m2",    "a-1"], 60)]
    fn time_spent(input: Vec<&str>, expeced: u64) {
        match convert(input) {
            Ok(log) => assert_eq!(
                log.time_spent,
                Duration::from(std::time::Duration::new(expeced, 0))
            ),
            _ => assert!(false, "failed to match the pattern"),
        }
    }

    #[test_case(vec!["log", "1h", "a-1"], "A",1)]
    #[test_case(vec!["log", "1h", "a-2"], "A",2)]
    #[test_case(vec!["log", "1m", "b-0"], "B",0)]
    #[test_case(vec!["log", "1m", "proj-1234"], "PROJ",1234)]
    fn issue_key(input: Vec<&str>, project: &str, key: i16) {
        match convert(input) {
            Ok(log) => assert_eq!(
                log.issue,
                IssueKey {
                    project: project.to_string(),
                    key: key
                }
            ),
            _ => assert!(false, "failed to match the pattern"),
        }
    }

    #[test_case(vec!["log", "1h", "a-1", "-w","2019-12-10T23:20:12"], "2019-12-10T23:20:12")]
    #[test_case(vec!["log", "1h", "a-1", "-w","2019-11-10T00:00:00"], "2019-11-10T00:00:00")]
    fn when(input: Vec<&str>, expected: &str) {
        match convert(input) {
            Ok(log) => assert_eq!(
                log.when.unwrap(),
                chrono::NaiveDateTime::from_str(expected).unwrap()
            ),
            _ => assert!(false, "failed to match the pattern"),
        }
    }
}
