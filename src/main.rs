#![warn(rust_2018_idioms)]

mod commands;
mod config;
mod jira;
mod mapping;

use structopt::StructOpt;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate prettytable;

use commands::*;

#[derive(StructOpt, PartialEq, Debug)]
#[structopt(about = "Rusty TimeLogger is a cli for the timelogging")]
enum App {
    /// Log command allows you to create a work log for a specific issue.
    Log(log::Log),
    ///
    Hours(hours::Hours),
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
        App::Log(input) => {
            jira::add_worklog(input.into());
        }
        App::Hours(input) => {
            println!("{:?}", input);
        }
    };
    Ok(())
}
