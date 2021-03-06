# Gomoku

## Building and running
(The current way to do things is not final at all)
### Using the shell script
You can run init_gomoku_env.sh then start_gomoku.sh, after which the game should open.
If it doesn't work out :

### Manually
First and foremost you will need to have rust and python3 installed with numpy and PyQt5, then (depending on your setup, but this Should work), just run cargo build (--release if you want)
then move the build target (.dll or .dylib  or whatever), to the project's root folder (same level as src), then run gomoku.py (in src/PyInterface) from the project's root.
