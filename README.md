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

### Log params 

| Name       | Required | Description                                                           |
| ---------- | -------- | --------------------------------------------------------------------- |
| time-spent | Yes      | Time spent on the task can be provided as 1h or 30m.                  |
| issue      | Yes      | Issue Key or Id of the ticket form the project management system.     |
| comment    | No       | Comment should be used to described what has been done.               |
| start-time | No       | Defines when work has begun. By default `current-time` - `time-spent` |

## Show 


## Hours

