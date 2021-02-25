#!/bin/sh
# this script builds the binary and deploys to kg cluster
cargo clean
# build
cargo build --release
# cp rkl to /usr/local/bin
cp target/release/rkl /usr/local/bin
# generate auto complete script
rkl -c bash > ~/.rkl_complete.sh
# deploy to other kg nodes
scp target/release/rkl kg-node43:/usr/local/bin
scp target/release/rkl kg-node44:/usr/local/bin
scp target/release/rkl kg-node45:/usr/local/bin
scp ~/.rkl_complete.sh kg-node43:~/.rkl_complete.sh
scp ~/.rkl_complete.sh kg-node44:~/.rkl_complete.sh
scp ~/.rkl_complete.sh kg-node45:~/.rkl_complete.sh
# source ~/.rkl_complete.sh in .bash_profile.sh on each node kg-node43 -- kg-node46
###########################################
# if [ -f ~/.rkl_complete.sh ]; then
#         . ~/.rkl_complete.sh
# fi
###########################################
