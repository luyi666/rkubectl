#!/bin/sh
# this script builds the binary and deploys to kg cluster
cargo clean
# build
cargo build --release
# cp rbl to /usr/local/bin
cp target/release/rbl /usr/local/bin
# generate auto complete script
rbl -c bash > ~/.rbl_complete.sh
# deploy to other kg nodes
scp target/release/rbl kg-node43:/usr/local/bin
scp target/release/rbl kg-node44:/usr/local/bin
scp target/release/rbl kg-node45:/usr/local/bin
scp ~/.rbl_complete.sh kg-node43:~/.rbl_complete.sh
scp ~/.rbl_complete.sh kg-node44:~/.rbl_complete.sh
scp ~/.rbl_complete.sh kg-node45:~/.rbl_complete.sh
# source ~/.rbl_complete.sh in .bash_profile.sh on each node kg-node43 -- kg-node46
###########################################
# if [ -f ~/.rbl_complete.sh ]; then
#         . ~/.rbl_complete.sh
# fi
###########################################
