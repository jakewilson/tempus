tempus
======
Easy time tracking.

Installation
------------
```
$ cargo install tempus-cli
```

Usage
-----
To start a session:
```
$ tempus -p <project_name>
```

the same command ends the session.

To calculate the total # of hours worked for a project, run
```
$ tempus -p <project_name> log --hours
```

To view all session times for a project, run
```
$ tempus -p <project_name> log
```

To view the start time of a session currently in progress, run
```
$ tempus -p <project_name> -s
```
This can also be used if you've forgotten whether you've started a session or not.
