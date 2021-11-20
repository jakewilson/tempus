taevus
======
Easy time tracking.

Keep track of time spent on projects. To start a session:
```
$ taevus -p project
```

the same command ends the session.

This will create a directory in `$HOME/taevus/<project>` with a log file
at `$HOME/taevus/<project>/taevus_log.txt`. You can view all start & end times for each
session in this file. To calculate the total # of hours worked for a project, run
```
$ taevus -p project --hours
```

'taevus' is a portmanteau of 'tempus' and 'aevum', two words in Latin that mean 'time'.
