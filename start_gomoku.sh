#!/bin/bash
DYLIB=target/release/librust_ext.dylib
GOMOKU_PATH=src/PyInterface/gomoku.py
LIBCURR=librust_ext.dylib
maturin develop --release
if test -f "$LIBCURR" && test -f "$DYLIB" ; then
	rm $LIBCURR
	mv $DYLIB .
fi
#if test -f "$DYLIB"; then
    #mv $DYLIB .
#fi
venv/bin/python3 $GOMOKU_PATH
#maturin develop --release && mv target/debug/librust_ext.dylib . && bin/python3.9 src/PyInterface/gomoku.py
