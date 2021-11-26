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

This will create a directory in `$HOME/tempus/<project>` with a log file
at `$HOME/tempus/<project>/taevus_log.txt`. You can view all start & end times for each
session in this file. To calculate the total # of hours worked for a project, run
```
$ tempus -p <project_name> --hours
```
