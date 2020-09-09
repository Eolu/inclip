# clip

This is a simple terminal utility for interacting with the system clipboard that should work on most platforms. Running it with no arguments will echo the system clipboard. Running the `clip diff [FILE]` subcommand will allow you to compare clipboard content to a file. Running `clip diff` without a file arg will cause the program to pause and wait for input, allowing you to change the clipboard contents and compare the previous contents to the new contents.

Note: I wrote this originally on Linux, but I noticed that Windows already has a command called "clip" which does the inverse of this - outputs to the clipboard rather than taking input form the clipboard. I recommend renaming this to `inclip.exe` if you plan on using it on your Windows path.

```
USAGE:
    clip.exe [OPTIONS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    diff [FILE]      Compare clipboard contents
    help             Prints this message or the help of the given subcommand(s)
```

### Planned Features

Allow arguments that the Linux diff or the windows FC command would allow for full control over the diff. This should be fairly simple to implement.

