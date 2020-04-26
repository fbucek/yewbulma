#!/usr/bin/env bash

#set -e # error -> trap -> exit
function info() { echo -e "[\033[0;34m $@ \033[0m]"; } # blue: [ info message ]
function pass() { echo -e "[\033[0;32mPASS\033[0m] $@"; } # green: [PASS]
function fail() { FAIL="true"; echo -e "[\033[0;31mFAIL\033[0m] $@"; } # red: [FAIL]
# Default exit handler
function exit_handler() { LASTRES=$?; LAST=$BASH_COMMAND; if [[ LASTRES -ne 0 ]]; then fail "Command: \"$LAST\" exited with exit code: $LASTRES"; elif [ "$FAIL" == "true"  ]; then fail finished with error; else echo -e "[\033[0;32mFinished\033[0m]";fi }
# Exit handler with killing PID
function kill_pid() { exit_handler; if [ ! -z $PID ]; then kill $PID;fi; }
trap kill_pid EXIT 

SRCDIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" >/dev/null 2>&1 && pwd )" # this source dir
DIRNAME="${SRCDIR##*/}"

# Watching build changes 
function watch() { cargo watch -i "pkg/**" -s "wasm-pack build"; }
# Watching test changes
function watchtest() { cargo watch -i "pkg/**" -s "wasm-pack test --chrome --headless"; }
# Watching example changes
function watchexample() { 
    info "Running Python server for dir: example/static ( pkill Python -> kill when not closed properly )"
    python3 -m http.server 8000 --directory examples/static &
    PID=$! # Store pid and 
    info "Runnnig cargo watch on example: $2"
    cargo watch -s "examples/run.sh $2"
}

if [ -z $2 ]; then 
    info "Run script"
    echo ""
    echo "usage: ./run.sh watch | watchtest | watchexample subdir"
fi

## Command line argument parsing
for arg in "$@"; do
    case "$arg" in
        watch) watch;;
        watchtest) watchtest;;
        watchexample) watchexample $@;;
    esac
done


