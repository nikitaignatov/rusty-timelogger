use crate::commands::log::IssueKey;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
pub struct Hours {
    /// Filters out worklogs where date is before 'from' date
    #[structopt(short = "f", long = "from")]
    pub from: Option<chrono::NaiveDate>,
    /// Filters out worklogs where date is after 'to' date
    #[structopt(short = "t", long = "to")]
    pub to: Option<chrono::NaiveDate>,
    /// Filters out worklogs that are not related to this issue
    #[structopt(long = "issue")]
    pub issue: Option<IssueKey>,
}

#[cfg(test)]
mod tests {}
