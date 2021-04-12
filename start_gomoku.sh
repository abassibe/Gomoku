#!/bin/bash
DEFAULT_DYLIB_PATH=target/release/librust_ext.dylib
DEFAULT_PYTHON_BIN=./venv/bin/pyth

if [[ $# -ge 1 && ($1 == "--help" || $1 == "-h") ]]; then
    echo -e "This script aims at building then running the project Gomoku.
    It uses the environment variable 'GOMOKU_RUST_LIB_PATH' to know where to find the built dylib.
    If this env var is NOT set then the default path is used which is: $DEFAULT_DYLIB_PATH

    It uses the environment variable 'GOMOKU_PYTHON_BIN_PATH' to know where to find the Python binary.
    If this env var is NOT set then the default path is used which is: $DEFAULT_PYTHON_BIN

    No option are currently supported apart from '--help' and '-h'
    No build has been performed, to build and run the project please rerun this script without the help option"
    exit 42
elif [ $# -ge 1 ]; then
    echo -e "Unknown option $1 found.
    This script currently only supports '--help' and '-h' as options"
    exit 42
fi

# Variable definitions
if [[ -n $GOMOKU_PYTHON_BIN_PATH ]]; then
    PYTHON_BIN=$GOMOKU_PYTHON_BIN_PATH
    elif [ -f $DEFAULT_PYTHON_BIN ]; then
    PYTHON_BIN=$DEFAULT_PYTHON_BIN
    else
    PYTHON_BIN=$(which python3)
fi

if [[ -n $GOMOKU_RUST_LIB_PATH ]]; then
    DYLIB=$GOMOKU_RUST_LIB_PATH
else
    DYLIB=$DEFAULT_DYLIB_PATH
fi
PYTHON=src/PyInterface/gomoku.py

echo "Target dylib is expected to by found at the following path: $DYLIB"

# Build
# Check if build is successful
if ! maturin develop --release; then
    echo -e "\033[0;31mMaturin failed to build the project in a venv.\033[0m Will try to build without a virtual environment"
    else
      HAS_VENV=true
    if ! pip3 install -r requirements.txt; then
    echo -e "\033[0;31mCould not install requirements. Cannot build project\033[0m"
    exit 42
    else
      HAS_VENV=false
    fi
fi

# Move the generated lib if it exists
if test -f "$DYLIB"; then
  if [ $HAS_VENV ]; then
  echo -e "A new version of the dylib has been found at $DYLIB.\nMoving into current working directory $PWD"
  mv $DYLIB .
  else
  echo -e "A new version of the dylib has been found at $DYLIB.\nMoving and renaming into appropriate directory src/PyInterface"
  mv $DYLIB src/PyInterface/.
  fi

    STATUS_CODE=$?
    # 0 is status code for success.
    # 2 is the status code returned by mv when the source and the target are the same (so not really an error we want to handle).
    if [ $STATUS_CODE -ne 0 ] && [ $STATUS_CODE -ne 2 ]; then
        echo -e "\033[0;31mAn error occurred while trying to move $DYLIB. Will NOT run the project\033[0m"
        exit 42
    fi
fi

# Run the project
$PYTHON_BIN $PYTHON