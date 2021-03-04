#!/bin/bash
echo "\n\033[1;3mRust install launching\033[0m, just press 1 and wait for the install to finish." && sleep 4
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
echo "\n\033[1;3mInstall ✅, mooving on to \033[1;3mCargo crates and maturin\033[0m." && sleep 2
echo "source $HOME/.cargo/env" >> ~/.zshrc
cargo install maturin
echo "\n\033[1;3mCrates Install/Update\033[0m ✅\n\033[1;3mMaturin Installed\033[0m ✅."
echo "\n\033[1;3mSwitching to venv\033[0m"
echo "Now run : source bin/activate.\nAfter the source you are ready to run the project, use : sh lauch_gomoku.sh"