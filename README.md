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

1. Visualization of file tree initializing
2. Directory space visualization with pie charts and etc.
3. Mark files and delete them(?)
