mod config;
mod worklog;

use structopt::StructOpt;

#[macro_use]
extern crate serde_derive;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Rusty TimeLogger is a cli for the timelogging")]
enum App {
    /// Log command allows you to create a work log for a specific issue.
    Log(worklog::Log),
    /// Config command allows to change some of the settings
    Config(config::RustyConfig),
}

fn main() -> Result<(), ::std::io::Error> {
    let args = App::from_args();
    match args {
        App::Config(conf) => {
            println!("{:?}", conf);
            config::store(conf);
            println!("Configuration is saved.");
        }
        App::Log(input) => println!("{:?}", input),
    };
    Ok(())
}
