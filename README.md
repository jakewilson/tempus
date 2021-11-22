taevus
======
Easy time tracking.

Installation
------------
```
$ cargo install taevus
```

Usage
-----
```
$ taevus -p <project_name>
```

the same command ends the session.

This will create a directory in `$HOME/taevus/<project>` with a log file
at `$HOME/taevus/<project>/taevus_log.txt`. You can view all start & end times for each
session in this file. To calculate the total # of hours worked for a project, run
```
$ taevus -p <project_name> --hours
```

'taevus' is a [portmanteau](https://en.wikipedia.org/wiki/Portmanteau) of 'tempus' and 'aevum', two words in Latin that mean 'time'.
