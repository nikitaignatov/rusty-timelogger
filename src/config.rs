extern crate confy;

use structopt::StructOpt;

#[derive(StructOpt, PartialEq, Serialize, Deserialize, Debug)]
pub struct RustyConfig {
    /// username for the jira account that will  be used to log time.
    #[structopt(long)]
    pub jira_username: String,
    /// api key for the jira account that will  be used to log time.
    #[structopt(long)]
    pub jira_api_key: String,
    /// url to the jira instance
    #[structopt(long)]
    pub jira_host: String,
}

impl ::std::default::Default for RustyConfig {
    fn default() -> Self {
        Self {
            jira_host: "".into(),
            jira_api_key: "".into(),
            jira_username: "".into(),
        }
    }
}

pub fn load() -> Result<RustyConfig, ::std::io::Error> {
    confy::load("rusty-timelogger")
}

pub fn store(configuration: RustyConfig) {
    confy::store("rusty-timelogger", configuration).expect("Did not store the configration");
}
