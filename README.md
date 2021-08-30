# Tima - A timer for humans

A simple timer created in Rust, for human beings. Tima (as in *tahym-uh*) can be set with common expressions like:
```
tima 1 m 32 s   # 1 minute and 32 seconds
tima 1min 22sec # 1 minutes and 22 seconds
tima 19 seconds
tima 2:13       # 2 minutes and 13 seconds
tima 1:23:04    # 1 hour, 23 minutes and 4 seconds
tima 1 seconds  # 1 second
```

Any combination of

* String options
  * h, hour, hours
  * m, minute, minutes
  * s, sec, seconds

* Time options
  * ##:##, ##:##:## 

is valid. Also, the conjunction `and` can be used at any point with the string options. The parameters are case insensitive.

## Coverage

cargo kcov -o coverage -- --exclude-patter=cargo,rustup,include

## Rustdocs

cargo doc --no-deps --bins

All documentation is provided in rustdoc format. Please check the website [rustdoc]

[rustdoc]: https://thecastles.gitlab.com/tima

## Command-line Options

Sets the timer to run for a number of seconds

```
tima <seconds>
```

Sets the timer to run for a number of minutes
```bash
tima -m <minutes>
```

Displays the help, with the options
```bash
tima -h
```

