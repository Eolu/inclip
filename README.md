# inclip

This is a simple terminal utility for interacting with the system clipboard that should work on most platforms. Running it with no arguments will echo the system clipboard. Running the `inclip diff [FILE]` subcommand will allow you to compare clipboard content to a file. Running `inclip diff` without a file arg will cause the program to pause and wait for input, allowing you to change the clipboard contents and compare the previous contents to the new contents.

USAGE:
    inclip
        Echo clipboard content
    inclip diff [file] [args...]
        Perform a comparison of clipboard content against a file or other
        clipboard content. On Mac and Linux, accepts all args that the `diff`
        command accepts. On Windows, accepts all args that the `fc.exe` command
        accepts. If a file is included, it must come before all other arguments
        that would usually be passed to diff or fc.exe. 

