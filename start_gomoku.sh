#!/bin/bash
DYLIB=target/debug/librust_ext.dylib
PYTHON=src/PyInterface/gomoku.py
maturin develop --release
if test -f "$DYLIB"; then
    mv $DYLIB .
fi
venv/bin/python $PYTHON
#maturin develop --release && mv target/debug/librust_ext.dylib . && bin/python3.9 src/PyInterface/gomoku.py
