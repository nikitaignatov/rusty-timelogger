extern crate humantime;
use structopt::StructOpt;
use humantime::Duration;

#[derive(StructOpt, Debug)]
struct Log {
    /// How much time was spent on this worklog. Examples: is 1h or 1h30m or 30m
    time_spent: Duration,
    /// Issue Key for this worklog. Examples: PROJ-1234
    issue: String,
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
