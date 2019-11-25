# Rusty TimeLogger

* Main goal of this project is to learn rust language. 
* Secondary goal is to create a cli that allows time logging from the terminal.


## Examples of usage

``` bash  
# add a log entry for a given issue
rusty log 1h3m task-12 "worked on stuff"

# show work log entries.
rusty show
rusty show today 
rusty show yesterday

# show total hours for the time period
rusty hours today
rusty hours yesterday
rusty hours 2019-01-20 2019-02-19
```

## Log
Log command should allow user to add a work entry with minimal typing effort.

Command should be used to log time after the work has been completed so the Begin time will be current time - the time spent on this work log.

### Params 

| Name       | Required | Description                                                           |
| ---------- | -------- | --------------------------------------------------------------------- |
| time-spent | Yes      | Time spent on the task can be provided as 1h or 30m.                  |
| issue      | Yes      | Issue Key or Id of the ticket form the project management system.     |
| comment    | No       | Comment should be used to described what has been done.               |
| start-time | No       | Defines when work has begun. By default `current-time` - `time-spent` |

### Usage
Below is the usage text for the log command
``` bash
Log command allows you to create a work log for a specific issue.

USAGE:
    rusty log <time-spent> <issue> [comment]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <time-spent>    How much time was spent on this worklog. Examples: is 1h or 1h30m or 30m
    <issue>         Issue Key for this worklog. Examples: PROJ-1234
    <comment>       Describe what you have done
```
## Show 


## Hours

## Config
In order to connect to Jira url and api key need to be configured. This version stores credentials on file system in plain text.

### Usage
``` bash
Config command allows to change some of the settings

USAGE:
    rusty.exe config --jira-api-key <jira-api-key> --jira-host <jira-host> --jira-username <jira-username>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --jira-api-key <jira-api-key>      api key for the jira account that will  be used to log time.
        --jira-host <jira-host>            url to the jira instance
        --jira-username <jira-username>    username for the jira account that will  be used to log time.
```