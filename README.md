tempus
======
Easy time tracking.

Keep track of time spent on projects. To start a session:
```
$ tempus -p project
```

the same command ends the session.

This will create a directory in `$HOME/tempus/<project>` with a log file
in `$HOME/tempus/<project>/tempus_log.txt`. You can view all start & end times for each
session in this file. To calculate the total # of hours worked for a project, run
```
$ tempus -p project --hours
```
