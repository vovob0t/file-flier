# FF - file-flier
A rust learning project that helps me with solving a world-old problem - not enough space.
It scans given directory and calculates size of all the contents.
Directory in scanned path could be traversed into - that helps with enumerating big folders and files

## Usage

```sh
❮ cargo run -- -p=/ --sort=size --help

CLI tools to analyze directory space
Usage:
--help | -h  -  prints out this help message
--path | -p  -  specify path of analyzing
--sort | -s  -  specify sort algorithm by:
                        size, alphabetical, modification, natural
```

## TODO 

0. Loading screen - DONE
!1. Make program run file scans in parallel mode
!2. Make program more platform friendly so it can run on Windows

1. Visualization of file tree initializing (would be cool to show off like a tree that grovs from root, and freeze it for a few seconds before showing files)
2. Directory space visualization with pie charts and etc.
3. Mark files and delete them(?)
4. Press / to search for a file in a curent directory
5. 
