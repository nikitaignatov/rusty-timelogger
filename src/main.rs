mod worklog;
use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Rusty TimeLogger is a cli for the timelogging")]
enum App {
    /// Log command allows you to create a work log for a specific issue.
    Log(worklog::Log),
}

fn main() {
    println!("{:?}", App::from_args());
}
