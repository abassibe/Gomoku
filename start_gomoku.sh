#!/bin/bash
DEFAULT_DYLIB_PATH=target/release/librust_ext.dylib
DEFAULT_PYTHON_BIN=bin/python3

if [[ $# -ge 1 && ($1 == "--help" || $1 == "-h") ]]; then
    echo -e "This script aims at building then running the project Gomoku.
    It uses the environment variable 'GOMOKU_RUST_LIB_PATH' to know where to find the built dylib.
    If this env var is NOT set then the default path is used which is: $DEFAULT_DYLIB_PATH

    No option are currently supported apart from '--help' and '-h'
    No build has been performed, to build and run the project please rerun this script without the help option"
    exit 42
elif [ $# -ge 1 ]; then
    echo -e "Unknonw option $1 found.
    This script currently only supports '--help' and '-h' as options"
    exit 42
fi

# Variable definitions
if [[ -n $GOMOKU_PYTHON_BIN_PATH ]]; then
    PYTHON_BIN=$GOMOKU_PYTHON_BIN_PATH
else
    PYTHON_BIN=$DEFAULT_PYTHON_BIN
fi

if [[ -n $GOMOKU_RUST_LIB_PATH ]]; then
    DYLIB=$GOMOKU_RUST_LIB_PATH
else
    DYLIB=$DEFAULT_DYLIB_PATH
fi
PYTHON=src/PyInterface/gomoku.py

echo "Target dylib is expected to by found at the following path: $DYLIB"

# Build
maturin develop --release

# Check if build is successful
if [ $? -ne 0 ]; then
    echo -e "\033[0;31mMaturin failed to build the project! Will NOT run the project\033[0m"
    exit 42
fi

# Move the generated lib if it exists
if test -f "$DYLIB"; then
    echo -e "A new version of the dylib has been found at $DYLIB.\nCopying into current working directory $PWD"
    mv $DYLIB .

    STATUS_CODE=$?
    # 0 is status code for success.
    # 2 is the status code returned by mv when the source and the target are the same (so not really an error we want to handle).
    if [ $STATUS_CODE -ne 0 ] && [ $STATUS_CODE -ne 2 ]; then
        echo -e "\033[0;31mAn error occured while trying to move $DYLIB to $PWD. Will NOT run the project\033[0m"
        exit 42
    fi
fi

# Run the project
$PYTHON_BIN $PYTHON

#maturin develop --release && mv target/debug/librust_ext.dylib . && bin/python3.9 src/PyInterface/gomoku.py