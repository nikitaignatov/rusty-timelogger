# Rusty Clock

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