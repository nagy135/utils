# Files that contain

Simple utility program that takes stream of file names, has some arguments and prints out files that contain given string

# Examples

* find . | ftc -n 3 "bin/bash" # this returns every file that has "bin/bash" in first 3 lines (even if its substring)
