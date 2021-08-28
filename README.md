# timer

A simple timer created in Rust

## Coverage

cargo kcov -o coverage -- --exclude-patter=cargo,rustup,include

## Rustdocs

cargo doc --no-deps --bins

All documentation is provided in rustdoc format. Please check the website [rustdoc]

[rustdoc]: https://thecastles.gitlab.com/timer

## Command-line Options

Sets the timer to run for a number of seconds

```
timer <seconds>
```

Sets the timer to run for a number of minutes
```bash
timer -m <minutes>
```

Displays the help, with the options
```bash
timer -h
```

