# inclip

This is a simple terminal utility for interacting with the system clipboard that should work on most platforms. Running it with no arguments will echo the system clipboard. Running the `inclip diff [FILE]` subcommand will allow you to compare clipboard content to a file. Running `inclip diff` without a file arg will cause the program to pause and wait for input, allowing you to change the clipboard contents and compare the previous contents to the new contents.

```
USAGE:
    inclip [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    diff [FILE]      Compare clipboard contents
    help             Prints this message or the help of the given subcommand(s)
```

### Planned Features

Allow arguments that the Linux diff or the windows FC command would allow for full control over the diff. This should be fairly simple to implement.

