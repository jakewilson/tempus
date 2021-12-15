# tempus
Easy time tracking.

## Installation
```
$ cargo install tempus-cli
```

## Usage
Start or end a session:
```
$ tempus -p <project_name>
```

View all session times for a project:
```
$ tempus -p <project_name> log
```

Calculate the total # of hours worked for a project:
```
$ tempus -p <project_name> log --hours
```

View the start time of a session currently in progress:
```
$ tempus -p <project_name> -s
```

This can also be used if you've forgotten whether you've started a session or not.

End a session without recording it (can be used if you accidentally started a session):
```
$ tempus -p <project_name> -x
```

If you ever need to view the raw data for the sessions, or add or delete a session manually, all session data is stored in `$HOME/tempus/<project_name>/tempus_log.txt`. If you delete this file, all session data will be lost.

### Ranges
To view times or calculate hours for a subset of sessions, use the date-range arg:
```
$ tempus -p <project_name> log 2021-11-01..2021-11-30
```

The range is _inclusive_, so the above example will include all sessions started in the month of November,
including those started on 11-30. `2021-11-01..2021-12-01` would be all of November and also December 1.

Date format: `yyyy-mm-dd | mm-dd | "today"`

You can use `today` instead of a date to use todays date:
```
$ tempus -p <project_name> log 2021-11-01..today
```

If the year is omitted, the current year is used. If a date or month only requires one digit,
you need only enter the one:
```
$ tempus -p <project_name> log 11-1..11-30 # same as yyyy-11-01..yyyy-11-30
```

Using one date will create a range from 1970-1-1 to the provided date:
```
$ tempus -p <project_name> log 2021-12-1 # same as 1970-1-1..2021-12-01
```

Omitting the second date creates a range from the first date to today:
```
$ tempus -p <project_name> log 2021-12-1.. # same as 2021-12-1..today
```
