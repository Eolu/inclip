# clip

This is a simple terminal utility for interacting with the system clipboard that should work on most platforms. Running it with no arguments will echo the system clipboard. Running the `clip diff [FILE]` subcommand will allow you to compare clipboard content to a file. Running `clip diff` without a file arg will cause the program to pause and wait for input, allowing you to change the clipboard contents and compare the previous contents to the new contents.

```
USAGE:
    clip.exe [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --diff <FILE>    Sets a custom config file

SUBCOMMANDS:
    diff    Compare clipboard contents
    help    Prints this message or the help of the given subcommand(s)
```
